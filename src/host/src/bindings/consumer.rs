use crate::wasi::messaging::{
    consumer,
    messaging_types::{Channel, Client, Error, GuestConfiguration, Message},
};

use wasmtime_wasi::WasiView;

#[async_trait::async_trait]
impl<T: WasiView> consumer::Host for T {
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
