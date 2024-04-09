use anyhow::{Context, Error};
use std::{collections::HashMap, fs, path::Path};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wit_component;

use crate::connections::*;
use crate::types::*;

mod connections;
pub mod types;

impl WasiView for Ctx {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.table
    }
}

impl WasiMessagingView for Ctx {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.table
    }
    fn connections(&mut self) -> &mut HashMap<String, Connection> {
        &mut self.connections
    }
}

pub fn convert_to_component(path: impl AsRef<Path>) -> wasmtime::Result<Vec<u8>> {
    // TODO(ljtill): Check file path exists
    let bytes = &fs::read(&path).context("failed to read file")?;
    let reactor_bytes = &fs::read("./adapters/wasi_snapshot_preview1.reactor.wasm")
        .context("failed to read adapter fle")?;

    wit_component::ComponentEncoder::default()
        .module(&bytes)?
        .adapter("wasi_snapshot_preview1", reactor_bytes)?
        .encode()
}

pub fn build_runtime() -> Result<(Store<Ctx>, Component, Linker<Ctx>), Error> {
    let mut builder = WasiCtxBuilder::new();
    builder.inherit_stdout();
    builder.inherit_stderr();

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;

    // TODO(ljtill): Re-implement connections
    let mut connections = HashMap::new();

    // TODO(ljtill): Default connection set
    // TODO(ljtill): Workload Identity support
    connections.insert("default".to_string(), new_connection()?);

    let store = Store::new(
        &engine,
        Ctx {
            connections: connections,
            table: ResourceTable::new(),
            wasi: builder.build(),
        },
    );

    let mut linker = Linker::new(&engine);

    wasmtime_wasi::command::add_to_linker(&mut linker)?;

    let component = Component::from_binary(
        &engine,
        &convert_to_component("./target/wasm32-wasi/debug/guest.wasm")?,
    )?;

    Ok((store, component, linker))
}
