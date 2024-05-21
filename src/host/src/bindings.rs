use crate::wasi::messaging::{consumer, messaging_types::*, producer};
use azure_messaging_servicebus::prelude::*;
use std::collections::HashMap;
use wasmtime_wasi::WasiView;

wasmtime::component::bindgen!({
    path: "../../wit",
    async: true,
    with: {
        "wasi:messaging/messaging-types/client": Client,
        "wasi:messaging/messaging-types/error": Error,
    }
});

pub struct Client {
    pub channels: HashMap<String, QueueClient>,
}

pub struct Error {}

#[async_trait::async_trait]
impl<T: WasiView> consumer::Host for T {
    async fn subscribe_try_receive(
        &mut self,
        _c: wasmtime::component::Resource<Client>,
        _ch: Channel,
        _t_milliseconds: u32,
    ) -> wasmtime::Result<Result<Option<Vec<Message>>, wasmtime::component::Resource<Error>>> {
        println!("[trace] subscribe_try_receive() function executed");
        todo!()
    }

    async fn subscribe_receive(
        &mut self,
        _c: wasmtime::component::Resource<Client>,
        _ch: Channel,
    ) -> wasmtime::Result<Result<Vec<Message>, wasmtime::component::Resource<Error>>> {
        println!("[trace] subscribe_receive() function executed");
        todo!()
    }

    async fn update_guest_configuration(
        &mut self,
        _gc: GuestConfiguration,
    ) -> wasmtime::Result<Result<(), wasmtime::component::Resource<Error>>> {
        println!("[trace] update_guest_configuration() function executed");
        todo!()
    }

    async fn complete_message(
        &mut self,
        _m: Message,
    ) -> wasmtime::Result<Result<(), wasmtime::component::Resource<Error>>> {
        println!("[trace] complete_message() function executed");
        todo!()
    }

    async fn abandon_message(
        &mut self,
        _m: Message,
    ) -> wasmtime::Result<Result<(), wasmtime::component::Resource<Error>>> {
        println!("[trace] abandon_message() function executed");
        todo!()
    }
}

#[async_trait::async_trait]
impl<T: WasiView> producer::Host for T {
    async fn send(
        &mut self,
        _c: wasmtime::component::Resource<Client>,
        _ch: Channel,
        _m: Vec<Message>,
    ) -> wasmtime::Result<Result<(), wasmtime::component::Resource<Error>>> {
        println!("[trace] send() function executed");

        todo!()

        // let queue_clients = self.table().iter_children(&c);
        // Iterate and find the queue_client we want based on the channel
        // client.queue_client.send_message("hello world").await?;

        // Ok(Ok(()))
    }
}

#[async_trait::async_trait]
impl<T: WasiView> Host for T {}

#[async_trait::async_trait]
impl<T: WasiView> HostClient for T {
    async fn connect(
        &mut self,
        _name: String,
    ) -> wasmtime::Result<
        Result<wasmtime::component::Resource<Client>, wasmtime::component::Resource<Error>>,
    > {
        println!("[trace] connect() function executed");

        // Takes a name to return a client connection

        todo!()

        // Get the connection from the hashmap
        // let connection = match self.connections().get(name.as_str()) {
        //     Some(c) => c,
        //     None => {
        //         return Ok(Err(self.table().push(Error {})?));
        //     }
        // };

        // TODO: Remove clone()
        // let queue_client = connection.queue_client.clone();

        // Push the client to the resource table
        // let resource = self.table().push(Client {
        //     queue_client: queue_client,
        // })?;

        // Ok(Ok(resource))
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Client>) -> wasmtime::Result<()> {
        println!("[trace] drop() function executed");
        self.table().delete(rep)?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<T: WasiView> HostError for T {
    async fn trace(&mut self) -> wasmtime::Result<String> {
        println!("[trace] trace() function executed");
        todo!()
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Error>) -> wasmtime::Result<()> {
        println!("[trace] drop() function executed");
        self.table().delete(rep)?;

        Ok(())
    }
}
