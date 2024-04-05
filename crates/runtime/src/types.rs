use azure_messaging_servicebus::service_bus::QueueClient;
use std::collections::HashMap;
use wasmtime_wasi::{ResourceTable, WasiCtx};

pub struct Connection {
    pub queue_client: QueueClient,
}

pub struct Ctx {
    pub connections: HashMap<String, Connection>,
    pub table: ResourceTable,
    pub wasi: WasiCtx,
}

pub trait WasiMessagingView: Send {
    fn ctx(&mut self) -> &mut WasiCtx;
    fn table(&mut self) -> &mut ResourceTable;
    fn connections(&mut self) -> &mut HashMap<String, Connection>;
}
