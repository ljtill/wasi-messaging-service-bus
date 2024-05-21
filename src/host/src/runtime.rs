use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::bindings::Client;
use azure_messaging_servicebus::service_bus::QueueClient;
use wasmtime::{
    component::{Component, Linker, Resource},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wit_component::ComponentEncoder;

pub fn initialise_client(channels: &Vec<String>, store: &mut Store<Ctx>) -> Resource<Client> {
    let mut client = Client {
        channels: HashMap::new(),
    };

    for channel in channels {
        match QueueClient::new(
            azure_core::new_http_client(),
            std::env::var("SERVICE_BUS_NAMESPACE")
                .expect("Environment variable `SERVICE_BUS_NAMESPACE` should be set."),
            channel,
            std::env::var("SERVICE_BUS_POLICY_NAME")
                .expect("Environment variable `SERVICE_BUS_POLICY_NAME` should be set."),
            std::env::var("SERVICE_BUS_POLICY_KEY")
                .expect("Environment variable `SERVICE_BUS_POLICY_KEY` should be set."),
        ) {
            Ok(c) => {
                println!("[trace] Creating QueueClient for channel: {}", channel);
                client.channels.insert(channel.to_string(), c);
            }
            Err(e) => {
                eprintln!("[error] Failed to create QueueClient: {:?}", e);
                panic!();
            }
        };
    }

    store.data_mut().table.push(client).unwrap()
}

// Runtime

pub struct Ctx {
    pub wasi: WasiCtx,
    pub table: ResourceTable,
}

impl WasiView for Ctx {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.table
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
            table: ResourceTable::new(),
            wasi: builder.build(),
        },
    )
}

pub fn create_component(engine: &Engine) -> Component {
    // TODO: Replace with environment variables
    let module_path = "./target/wasm32-wasi/debug/guest.wasm";
    let reactor_path = "./eng/adapters/wasi_snapshot_preview1.reactor.wasm";

    if !Path::new(&module_path).exists() {
        eprintln!("[error] Module file does not exist");
        panic!();
    }

    if !Path::new(&reactor_path).exists() {
        eprintln!("[error] Reactor file does not exist");
        panic!();
    }

    let bytes = match fs::read(&module_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[error] Failed to read module file: {:?}", e);
            panic!();
        }
    };

    let reactor_bytes = match fs::read(reactor_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[error] Failed to read reactor file: {:?}", e);
            panic!();
        }
    };

    let encoder = ComponentEncoder::default().module(&bytes);
    let binary = match encoder {
        Ok(encoder) => {
            let encoder = encoder.adapter("wasi_snapshot_preview1", &reactor_bytes);
            match encoder {
                Ok(encoder) => {
                    let binary = encoder.encode();
                    match binary {
                        Ok(binary) => binary,
                        Err(e) => panic!("Failed to encode: {}", e),
                    }
                }
                Err(e) => panic!("Failed to add adapter: {}", e),
            }
        }
        Err(e) => panic!("Failed to add module: {}", e),
    };

    // TODO: Handle Debug & Release builds
    Component::from_binary(&engine, &binary).unwrap()
}

pub fn create_linker(engine: &Engine) -> Linker<Ctx> {
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::command::add_to_linker(&mut linker).unwrap();

    linker
}
