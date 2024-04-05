use runtime::types::WasiMessagingView;

use crate::types::*;
use crate::wasi::messaging::{
    consumer,
    messaging_types::{Channel, GuestConfiguration, Host, HostClient, HostError, Message},
    producer,
};

mod types;

wasmtime::component::bindgen!({
    path: "../../wit",
    async: true,
    with: {
        "wasi:messaging/messaging-types/client": types::Client,
        "wasi:messaging/messaging-types/error": types::Error,
    }
});

#[async_trait::async_trait]
impl<T: WasiMessagingView> consumer::Host for T {
    async fn subscribe_try_receive(
        &mut self,
        _c: wasmtime::component::Resource<Client>,
        _ch: Channel,
        _t_milliseconds: u32,
    ) -> wasmtime::Result<Result<Option<Vec<Message>>, wasmtime::component::Resource<Error>>> {
        todo!()
    }

    async fn subscribe_receive(
        &mut self,
        _c: wasmtime::component::Resource<Client>,
        _ch: Channel,
    ) -> wasmtime::Result<Result<Vec<Message>, wasmtime::component::Resource<Error>>> {
        todo!()
    }

    async fn update_guest_configuration(
        &mut self,
        _gc: GuestConfiguration,
    ) -> wasmtime::Result<Result<(), wasmtime::component::Resource<Error>>> {
        todo!()
    }

    async fn complete_message(
        &mut self,
        _m: Message,
    ) -> wasmtime::Result<Result<(), wasmtime::component::Resource<Error>>> {
        todo!()
    }

    async fn abandon_message(
        &mut self,
        _m: Message,
    ) -> wasmtime::Result<Result<(), wasmtime::component::Resource<Error>>> {
        todo!()
    }
}

#[async_trait::async_trait]
impl<T: WasiMessagingView> producer::Host for T {
    async fn send(
        &mut self,
        c: wasmtime::component::Resource<Client>,
        _ch: Channel,
        _m: Vec<Message>,
    ) -> wasmtime::Result<Result<(), wasmtime::component::Resource<Error>>> {
        println!("[host] Called function (send)");

        let client = self.table().get(&c).unwrap();

        client.queue_client.send_message("hello world").await?;

        // TODO(ljtill): Implement error handling
        // match client.queue_client.send_message("hello world").await {
        //     Ok(_) => Ok(Ok(())),
        //     Err(_) => Ok(Err(self.table().push(Error {}).unwrap())),
        // }

        Ok(Ok(()))
    }
}

#[async_trait::async_trait]
impl<T: WasiMessagingView> Host for T {}

#[async_trait::async_trait]
impl<T: WasiMessagingView> HostClient for T {
    async fn connect(
        &mut self,
        name: String,
    ) -> wasmtime::Result<
        Result<wasmtime::component::Resource<Client>, wasmtime::component::Resource<Error>>,
    > {
        println!("[host] Called function (connect)");

        // Get the connection from the hashmap
        let connection = self.connections().get(name.as_str()).unwrap();

        // TODO(ljtill): Improve this logic
        let queue_client = connection.queue_client.clone();

        // Push the client to the resource table
        let resource = self
            .table()
            .push(Client {
                queue_client: queue_client,
            })
            .unwrap();

        Ok(Ok(resource))
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Client>) -> wasmtime::Result<()> {
        self.table().delete(rep).unwrap();

        Ok(())
    }
}

#[async_trait::async_trait]
impl<T: WasiMessagingView> HostError for T {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        todo!()
    }

    fn drop(&mut self, _rep: wasmtime::component::Resource<Error>) -> wasmtime::Result<()> {
        todo!()
    }
}
