use runtime::{build_runtime, types::Ctx};

use bindings::{
    wasi::messaging::{consumer, messaging_types},
    Messaging,
};

fn listen() {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[host] Starting runtime");

    let (mut store, component, mut linker) = build_runtime();
    consumer::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)
        .expect("Failed to add consumer to linker");
    messaging_types::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)
        .expect("Failed to add types to linker");

    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;

    println!("[host] Calling guest function (configure)");
    let guest_configuration = messaging
        .wasi_messaging_messaging_guest()
        .call_configure(&mut store)
        .await?
        .unwrap();

    // TODO(ljtill): Subscribe
    let connection = store.data().connections.get("default").unwrap();

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
