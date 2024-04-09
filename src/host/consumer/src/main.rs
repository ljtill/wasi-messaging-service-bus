use runtime::{build_runtime, types::Ctx};

use bindings::{
    wasi::messaging::{consumer, messaging_types},
    Messaging,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[host] Starting runtime");

    let (mut store, component, mut linker) = build_runtime()?;
    consumer::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)?;
    messaging_types::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)?;

    let (messaging, _instance) =
        Messaging::instantiate_async(&mut store, &component, &linker).await?;

    println!("[host] Calling guest function (configure)");
    let _guest_configuration = messaging
        .wasi_messaging_messaging_guest()
        .call_configure(&mut store)
        .await?;

    // TODO(ljtill): Subscribe
    let _connection = store.data().connections.get("default").unwrap();

    // &[{
    //     Message {
    //         data: "wasi".as_bytes().to_vec(),
    //         format: FormatSpec::Raw,
    //         metadata: Option::None,
    //     }
    // }]

    // println!("[host] Calling guest function (handler)");
    // let _res = messaging
    //     .wasi_messaging_messaging_guest()
    //     .call_handler(&mut store, &messages)
    //     .await?;

    println!("[host] Terminating runtime");
    Ok(())
}
