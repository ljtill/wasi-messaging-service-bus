use crate::wasi::messaging::{
    messaging_types::{Channel, Client, Error, Message},
    producer,
};

use wasmtime_wasi::WasiView;

#[async_trait::async_trait]
impl<T: WasiView> producer::Host for T {
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
