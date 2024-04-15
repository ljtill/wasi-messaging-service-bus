use anyhow::{Context, Error};
use azure_messaging_servicebus::service_bus::QueueClient;
use std::{collections::HashMap, fs};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wit_component::ComponentEncoder;

#[derive(Clone)]
pub struct Client {
    pub queue_client: QueueClient,
}

fn new_client(channel: String) -> Result<Client, Error> {
    let namespace = std::env::var("SERVICE_BUS_NAMESPACE")
        .expect("Environment variable `SERVICE_BUS_NAMESPACE` should be set.");

    let policy_name = std::env::var("SERVICE_BUS_POLICY_NAME")
        .expect("Environment variable `SERVICE_BUS_POLICY_NAME` should be set.");

    let policy_key = std::env::var("SERVICE_BUS_POLICY_KEY")
        .expect("Environment variable `SERVICE_BUS_POLICY_KEY` should be set.");

    let queue_client = QueueClient::new(
        azure_core::new_http_client(),
        namespace,
        channel,
        policy_name,
        policy_key,
    )?;

    Ok(Client { queue_client })
}

pub struct Ctx {
    pub wasi: WasiCtx,
    pub table: ResourceTable,
    pub clients: HashMap<String, Client>,
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
    fn connections(&mut self) -> &mut HashMap<String, Client>;
}

impl WasiMessagingView for Ctx {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.table
    }
    fn connections(&mut self) -> &mut HashMap<String, Client> {
        &mut self.clients
    }
}

#[derive(Clone)]
pub struct State {
    clients: HashMap<String, Client>,
}

impl State {
    pub fn new() -> Self {
        State {
            clients: HashMap::new(),
        }
    }

    pub fn new_client(&mut self, channel: &String) -> Option<Client> {
        self.clients.insert(
            channel.to_string(),
            new_client(channel.to_string()).unwrap(),
        )
    }

    pub fn get_client(&self, channel: &String) -> &Client {
        self.clients.get(channel.as_str()).unwrap()
    }
}

pub fn create_builder() -> WasiCtxBuilder {
    let mut builder = WasiCtxBuilder::new();
    builder.inherit_stdout();
    builder.inherit_stderr();

    builder
}

pub fn create_engine() -> Engine {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    Engine::new(&config).unwrap()
}

pub fn create_store(mut builder: WasiCtxBuilder, engine: &Engine) -> Store<Ctx> {
    Store::new(
        &engine,
        Ctx {
            clients: HashMap::new(),
            table: ResourceTable::new(),
            wasi: builder.build(),
        },
    )
}

pub fn create_component(engine: &Engine) -> Component {
    let module_path = "./target/wasm32-wasi/debug/guest.wasm";
    let reactor_path = "./adapters/wasi_snapshot_preview1.reactor.wasm";

    // TODO: Check file path exists
    let bytes = &fs::read(&module_path)
        .context("failed to read file")
        .unwrap();

    let reactor_bytes = &fs::read(reactor_path)
        .context("failed to read adapter fle")
        .unwrap();

    let binary = ComponentEncoder::default()
        .module(&bytes)
        .unwrap()
        .adapter("wasi_snapshot_preview1", reactor_bytes)
        .unwrap()
        .encode()
        .unwrap();

    // TODO: Handle Debug & Release builds
    Component::from_binary(&engine, &binary).unwrap()
}

pub fn create_linker(engine: &Engine) -> Linker<Ctx> {
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::command::add_to_linker(&mut linker).unwrap();

    linker
}
