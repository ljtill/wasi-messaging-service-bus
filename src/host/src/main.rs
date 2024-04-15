//! Messaging Host
//!
//! The host implementation for WASI Messaging specification.
//!
//! The underlying messaging service is Azure Service Bus.
//!

use bindings::{
    wasi::messaging::{consumer, messaging_types, producer},
    Messaging,
};
use runtime::{Ctx, State};
use std::collections::HashMap;
use tokio::{
    select, signal,
    sync::mpsc,
    time::{sleep, Duration},
};

/// The worker service.
/// This service listens for messages from Service Bus.
async fn worker(mut state: State, channel: String, worker_id: usize, tx: mpsc::Sender<String>) {
    println!("[debug] Starting worker...");
    state.new_client(&channel);

    loop {
        // Peak messages from Service Bus
        let messages = state
            .get_client(&channel)
            .queue_client
            .peek_lock_message(Some(Duration::from_secs(5)))
            .await
            .unwrap();

        // Check if messages are found
        if !messages.is_empty() {
            println!(
                "[trace] Worker {} - Channel {} - Messages: {:?}",
                worker_id, channel, messages
            );

            // Send messages to channel
            if let Err(_) = tx.send(messages).await {
                eprintln!("[error] Failed sending messages from worker {}", worker_id);
            }
        }

        sleep(Duration::from_secs(5)).await;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[info] Starting runtime...");

    // Create state
    let state = State::new();

    // Create runtime components
    let builder = runtime::create_builder();
    let engine = runtime::create_engine();

    // Create store, component and linker
    let mut store = runtime::create_store(builder, &engine);
    let component = runtime::create_component(&engine);
    let mut linker = runtime::create_linker(&engine);

    // Add messaging components to linker
    consumer::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)?;
    producer::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)?;
    messaging_types::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)?;

    // Instantiate messaging component
    let (messaging, _instance) =
        Messaging::instantiate_async(&mut store, &component, &linker).await?;

    // Configure messaging
    let configuration = messaging
        .wasi_messaging_messaging_guest()
        .call_configure(&mut store)
        .await?
        .unwrap();

    // Number of workers
    let num_workers = configuration.channels.len();

    // Store channels
    let mut worker_channels = HashMap::<usize, mpsc::Receiver<String>>::new();
    let mut worker_handles = Vec::new();

    // Spawn workers and create sender channels
    for worker_id in 0..num_workers {
        // Set channel message buffer
        let (tx, rx) = mpsc::channel::<String>(100);
        worker_channels.insert(worker_id, rx);

        // Parse worker channel
        let channel = configuration.channels[worker_id].clone();

        let state = state.clone();

        // Spawn worker task
        let worker_handle = tokio::spawn(worker(state, channel, worker_id, tx));
        worker_handles.push(worker_handle);
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
                        // Process message here based on worker_id
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
