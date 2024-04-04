use crate::wasi::messaging::{consumer, messaging_types, producer};

use anyhow::Context;
use std::{fs, path::Path};
use wit_component;

use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

pub struct Ctx {
    table: ResourceTable,
    wasi: WasiCtx,
}

impl WasiView for Ctx {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.table
    }
}

pub fn convert_to_component(path: impl AsRef<Path>) -> wasmtime::Result<Vec<u8>> {
    // TODO(ljtill): Check file path exists
    let bytes = &fs::read(&path).context("failed to read file")?;
    let reactor_bytes = &fs::read("./eng/adapters/wasi_snapshot_preview1.reactor.wasm")
        .context("failed to read adapter fle")?;

    wit_component::ComponentEncoder::default()
        .module(&bytes)?
        .adapter("wasi_snapshot_preview1", reactor_bytes)?
        .encode()
}

pub fn new_runtime() -> (Store<Ctx>, Component, Linker<Ctx>) {
    let mut builder = WasiCtxBuilder::new();
    builder.inherit_stdout();
    builder.inherit_stderr();

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config).expect("Failed to create engine");

    let store = Store::new(
        &engine,
        Ctx {
            table: ResourceTable::new(),
            wasi: builder.build(),
        },
    );

    let mut linker = Linker::new(&engine);

    producer::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)
        .expect("Failed to add producer to linker");
    consumer::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)
        .expect("Failed to add consumer to linker");
    messaging_types::add_to_linker(&mut linker, |ctx: &mut Ctx| ctx)
        .expect("Failed to add types to linker");

    wasmtime_wasi::command::add_to_linker(&mut linker).expect("Failed to add wasi to linker");

    let component = Component::from_binary(
        &engine,
        &convert_to_component("./target/wasm32-wasi/debug/guest.wasm").expect("Failed to convert"),
    )
    .expect("Failed to create component");

    (store, component, linker)
}
