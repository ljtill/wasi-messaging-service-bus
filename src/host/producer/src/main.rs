use bindings::{
    wasi::messaging::{messaging_types, producer},
    Messaging,
};
use runtime::types::{Ctx, RuntimeBuilder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[host] Starting runtime");

    let mut builder = RuntimeBuilder::new()?;
    producer::add_to_linker(&mut builder.linker, |ctx: &mut Ctx| ctx)?;
    messaging_types::add_to_linker(&mut builder.linker, |ctx: &mut Ctx| ctx)?;

    let (_messaging, _) =
        Messaging::instantiate_async(&mut builder.store, &builder.component, &builder.linker)
            .await?;

    println!("[host] Terminating runtime");
    Ok(())
}
