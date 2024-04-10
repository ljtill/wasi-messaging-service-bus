wit_bindgen::generate!({
    path: "../../wit"
});

use crate::wasi::messaging::messaging_types::Client;
use exports::wasi::messaging::messaging_guest::{Error, Guest, GuestConfiguration, Message};

struct Component;

impl Guest for Component {
    fn configure() -> Result<GuestConfiguration, Error> {
        println!("[trace] configure() function called");
        let channels = vec!["default".to_string()];

        Ok(GuestConfiguration {
            channels: channels,
            extensions: Option::None,
        })
    }

    fn handler(_ms: Vec<Message>) -> Result<(), Error> {
        println!("[trace] handler() function called");

        let _client = Client::connect("default")?;

        // TODO(ljtill): Implement handler

        Ok(())
    }
}

export!(Component);
