//! # Guest
//!
//! The guest implementation for WASI Messaging specification.
//!

wit_bindgen::generate!({
    path: "../../wit"
});

use crate::wasi::messaging::messaging_types::Client;
use exports::wasi::messaging::messaging_guest::{Error, Guest, GuestConfiguration, Message};

struct Component;

impl Guest for Component {
    fn configure() -> Result<GuestConfiguration, Error> {
        println!("[trace] configure() function executed");
        let channels = vec!["default".to_string()];

        Ok(GuestConfiguration {
            channels: channels,
            extensions: Option::None,
        })
    }

    fn handler(_ms: Vec<Message>) -> Result<(), Error> {
        println!("[trace] handler() function executed");

        let _client = Client::connect("default")?;

        // TODO: Implement handler

        Ok(())
    }
}

export!(Component);
