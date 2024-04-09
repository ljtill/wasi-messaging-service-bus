use runtime::{build_runtime, types::Ctx};

use bindings::{
    wasi::messaging::{messaging_types, producer},
    Messaging,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[host] Starting runtime");

    let (mut store, component, mut linker) = build_runtime()?;
    producer::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)?;
    messaging_types::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)?;

    let (_messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;

    println!("[host] Terminating runtime");
    Ok(())
}
