wit_bindgen::generate!({
    path: "../../wit"
});

use crate::wasi::messaging::{messaging_types::Client, producer};
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

    fn handler(ms: Vec<Message>) -> Result<(), Error> {
        println!("[guest] Called function (handler)");

        println!("[guest] Calling host function (connect)");
        // TODO(ljtill): Return error from host
        let client = Client::connect("service_bus")?;
        let channel = "a".to_string();

        println!("[guest] Calling host function (send)");
        // TODO(ljtill): Return error from host
        producer::send(client, &channel, &ms)?;

        Ok(())
    }
}

export!(Component);
