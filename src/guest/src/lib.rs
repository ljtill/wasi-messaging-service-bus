wit_bindgen::generate!({
    path: "../../wit"
});

use crate::wasi::messaging::messaging_types::Client;
use exports::wasi::messaging::messaging_guest::{Error, Guest, GuestConfiguration, Message};

struct Component;

impl Guest for Component {
    fn configure() -> Result<GuestConfiguration, Error> {
        println!("[guest] Called function (configure)");

        Ok(GuestConfiguration {
            channels: vec!["my_channel".to_string()],
            extensions: Option::None,
        })
    }

    fn handler(_ms: Vec<Message>) -> Result<(), Error> {
        println!("[guest] Called function (handler)");

        println!("[guest] Calling host function (connect)");
        let _client = Client::connect("service_bus")?;

        // TODO(ljtill): Implement handler

        Ok(())
    }
}

export!(Component);
