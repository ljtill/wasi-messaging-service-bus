mod bindings;
mod runtime;

wasmtime::component::bindgen!({
    path: "../../wit",
    async: true,
    with: {
        "wasi:messaging/messaging-types/client": bindings::types::Client,
        "wasi:messaging/messaging-types/error": bindings::types::Error,
    }
});

use crate::wasi::messaging::messaging_types::{FormatSpec, Message};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[host] Starting runtime");

    let (mut store, component, linker) = runtime::new_runtime();

    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;

    println!("[host] Calling guest function (configure)");
    let _res = messaging
        .wasi_messaging_messaging_guest()
        .call_configure(&mut store)
        .await?;

    println!("[host] Calling guest function (handler)");
    let _res = messaging
        .wasi_messaging_messaging_guest()
        .call_handler(
            &mut store,
            &[{
                Message {
                    data: "wasi".as_bytes().to_vec(),
                    format: FormatSpec::Raw,
                    metadata: Option::None,
                }
            }],
        )
        .await?;

    println!("[host] Terminating runtime");
    Ok(())
}
