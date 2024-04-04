use crate::wasi::messaging::messaging_types::{Host, HostClient, HostError};

use azure_messaging_servicebus::prelude::*;
use wasmtime_wasi::WasiView;

impl<T: WasiView> Host for T {}

pub struct Client {
    pub queue_client: QueueClient,
}

#[async_trait::async_trait]
impl<T: WasiView> HostClient for T {
    async fn connect(
        &mut self,
        _name: String,
    ) -> wasmtime::Result<
        Result<wasmtime::component::Resource<Client>, wasmtime::component::Resource<Error>>,
    > {
        println!("[host] Called function (connect)");

        // TODO(ljtill): Implement connections name lookup

        // TODO(ljtill): Replace with environment variables
        let service_bus_namespace = "";
        let queue_name = "default";
        let policy_name = "SendListenSharedAccessKey";
        let policy_key = "";

        let http_client = azure_core::new_http_client();

        // TODO(ljtill): Handle expect error
        let queue_client = QueueClient::new(
            http_client,
            service_bus_namespace,
            queue_name,
            policy_name,
            policy_key,
        )
        .expect("Failed to create queue client");

        let resource = self.table().push(Client { queue_client }).unwrap();

        Ok(Ok(resource))
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Client>) -> wasmtime::Result<()> {
        self.table().delete(rep).unwrap();

        Ok(())
    }
}

pub struct Error {}

#[async_trait::async_trait]
impl<T: WasiView> HostError for T {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        todo!()
    }

    fn drop(&mut self, _rep: wasmtime::component::Resource<Error>) -> wasmtime::Result<()> {
        todo!()
    }
}
