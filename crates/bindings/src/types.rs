use azure_messaging_servicebus::prelude::*;

pub struct Client {
    pub queue_client: QueueClient,
}

pub struct Error {}
