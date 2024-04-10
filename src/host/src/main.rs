use bindings::{
    wasi::messaging::{consumer, messaging_types, producer},
    Messaging,
};
use runtime::types::{Ctx, RuntimeBuilder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[host] Starting runtime");

    let mut builder = RuntimeBuilder::new()?;
    consumer::add_to_linker(&mut builder.linker, |ctx: &mut Ctx| ctx)?;
    producer::add_to_linker(&mut builder.linker, |ctx: &mut Ctx| ctx)?;
    messaging_types::add_to_linker(&mut builder.linker, |ctx: &mut Ctx| ctx)?;

    let (messaging, _) =
        Messaging::instantiate_async(&mut builder.store, &builder.component, &builder.linker)
            .await?;

    println!("[host] Calling guest function (configure)");
    let _guest_configuration = messaging
        .wasi_messaging_messaging_guest()
        .call_configure(&mut builder.store)
        .await?;

    // TODO(ljtill): Subscribe
    //let _connection = builder.store.data().connections.get("default").unwrap();

    // Listen for messages
    // Receive them and pass them to the guest

    // println!("[host] Calling guest function (handler)");
    // let _res = messaging
    //     .wasi_messaging_messaging_guest()
    //     .call_handler(&mut store, &messages)
    //     .await?;

    println!("[host] Terminating runtime");
    Ok(())
}
