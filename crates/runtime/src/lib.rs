use anyhow::{Context, Error};
use azure_messaging_servicebus::service_bus::QueueClient;
use std::{collections::HashMap, fs, path::Path};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wit_component::ComponentEncoder;

pub struct Connection {
    pub queue_client: QueueClient,
}

pub struct Ctx {
    pub connections: HashMap<String, Connection>,
    pub table: ResourceTable,
    pub wasi: WasiCtx,
}

impl WasiView for Ctx {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.table
    }
}

pub trait WasiMessagingView: Send {
    fn ctx(&mut self) -> &mut WasiCtx;
    fn table(&mut self) -> &mut ResourceTable;
    fn connections(&mut self) -> &mut HashMap<String, Connection>;
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

pub struct RuntimeBuilder {
    pub component: Component,
    pub linker: Linker<Ctx>,
    pub store: Store<Ctx>,
}

impl RuntimeBuilder {
    pub fn new() -> Result<Self, Error> {
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

        // TODO(ljtill): Handle Debug & Release builds
        let component = Component::from_binary(
            &engine,
            &convert_to_component("./target/wasm32-wasi/debug/guest.wasm")?,
        )?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::command::add_to_linker(&mut linker)?;

        Ok(Self {
            component,
            linker,
            store,
        })
    }
}

pub fn new_connection() -> Result<Connection, Error> {
    let queue_client = QueueClient::new(
        azure_core::new_http_client(),
        std::env::var("SERVICE_BUS_NAMESPACE")
            .expect("Environment variable `SERVICE_BUS_NAMESPACE` should be set."),
        std::env::var("SERVICE_BUS_QUEUE")
            .expect("Environment variable `SERVICE_BUS_QUEUE` should be set."),
        std::env::var("SERVICE_BUS_POLICY_NAME")
            .expect("Environment variable `SERVICE_BUS_POLICY_NAME` should be set."),
        std::env::var("SERVICE_BUS_POLICY_KEY")
            .expect("Environment variable `SERVICE_BUS_POLICY_KEY` should be set."),
    )?;

    Ok(Connection { queue_client })
}

pub fn convert_to_component(path: impl AsRef<Path>) -> wasmtime::Result<Vec<u8>> {
    // TODO(ljtill): Check file path exists
    let bytes = &fs::read(&path).context("failed to read file")?;
    let reactor_bytes = &fs::read("./adapters/wasi_snapshot_preview1.reactor.wasm")
        .context("failed to read adapter fle")?;

    ComponentEncoder::default()
        .module(&bytes)?
        .adapter("wasi_snapshot_preview1", reactor_bytes)?
        .encode()
}
