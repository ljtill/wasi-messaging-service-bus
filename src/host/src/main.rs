use crate::bindings::*;
use crate::runtime::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::{
    select, signal,
    sync::mpsc,
    time::{sleep, Duration},
};

mod bindings;
mod runtime;

async fn worker(
    client: Arc<Mutex<&Client>>,
    channel: String,
    worker_id: usize,
    tx: mpsc::Sender<String>,
) -> ! {
    println!("[debug] Starting worker...");
    let client = client.lock().unwrap().channels.get(&channel).unwrap();

    loop {
        // Peak messages from Service Bus
        let messages = match client.peek_lock_message(Some(Duration::from_secs(5))).await {
            Ok(m) => m,
            Err(e) => {
                eprintln!(
                    "[error] Failed to peek messages from channel {}: {:?}",
                    channel, e
                );
                continue;
            }
        };

        // Check if messages are found
        if !messages.is_empty() {
            println!(
                "[trace] Worker {} - Channel {} - Messages: {:?}",
                worker_id, channel, messages
            );

            // Send messages to channel
            match tx.send(messages).await {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("[error] Failed sending messages from worker {}", worker_id);
                }
            }
        }

        sleep(Duration::from_secs(5)).await;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[info] Starting runtime...");

    // Create runtime components
    let builder = runtime::create_builder();
    let engine = runtime::create_engine();

    // Create store, component and linker
    let mut store = runtime::create_store(builder, &engine);
    let component = runtime::create_component(&engine);
    let mut linker = runtime::create_linker(&engine);

    // Add messaging components to linker
    bindings::wasi::messaging::consumer::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)?;
    bindings::wasi::messaging::producer::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)?;
    bindings::wasi::messaging::messaging_types::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)?;

    // Instantiate messaging component
    let (messaging, _instance) =
        Messaging::instantiate_async(&mut store, &component, &linker).await?;

    // Configure messaging
    let configuration = match messaging
        .wasi_messaging_messaging_guest()
        .call_configure(&mut store)
        .await?
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[error] Failed to configure messaging: {:?}", e);
            println!("[info] Terminating runtime...");
            std::process::exit(1);
        }
    };

    // Initialize state with clients in resource table
    let resource_client = runtime::initialise_client(&configuration.channels, &mut store);

    // Number of workers
    let num_workers = configuration.channels.len();

    // Store channels
    let mut worker_channels = HashMap::<usize, mpsc::Receiver<String>>::new();
    // let mut worker_handles = Vec::new();

    let client = store.data().table.get(&resource_client).unwrap();

    let client = Arc::new(Mutex::new(client));

    // Wrap store in Arc<Mutex<Store<Ctx>>>
    // let store = Arc::new(Mutex::new(store));

    // let client = Arc::new(Mutex::new(
    //     store.data().table.get(&resource_client).unwrap(),
    // ));

    // let client = match store.data().table.get(&resource_client) {
    //     Ok(c) => c,
    //     Err(e) => {
    //         eprintln!("[error] Failed to get client: {:?}", e);
    //         panic!();
    //     }
    // };

    // Spawn workers and create sender channels
    for worker_id in 0..num_workers {
        // Set channel message buffer
        let (tx, rx) = mpsc::channel::<String>(100);
        worker_channels.insert(worker_id, rx);

        // Parse worker channel
        let channel = configuration.channels[worker_id].clone();

        let client = client.clone();

        // Spawn worker task
        let worker_handle = tokio::spawn(worker(client, channel, worker_id, tx));
        // worker_handles.push(worker_handle);
    }

    loop {
        select! {
            _ = signal::ctrl_c() => {
                println!("[info] Terminating runtime...");
                break;
            }
            _ = async {
                for (worker_id, sender) in &mut worker_channels {
                    if let Some(message) = sender.recv().await {
                        println!("[trace] Received data: {}", message);
                        // TODO: Implement message processing
                        // println!("[host] Calling guest function (handler)");
                        // let _res = messaging
                        //     .wasi_messaging_messaging_guest()
                        //     .call_handler(&mut store, &messages)
                        //     .await?;
                    } else {
                        eprintln!("[error] Worker {} channel closed unexpectedly", worker_id);
                        // Handle channel closed event (optional)
                    }
                }
            } => {}
        }
    }

    Ok(())
}
