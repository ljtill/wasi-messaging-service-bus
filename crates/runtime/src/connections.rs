use crate::types;

use anyhow::Error;
use azure_messaging_servicebus::service_bus::QueueClient;
use types::*;

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
