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
        let client = Client::connect("service_bus").expect("Unable to connect");
        let channel = "a".to_string();

        println!("[guest] Calling host function (send)");
        producer::send(client, &channel, &ms).expect("Unable to send message");

        Ok(())
    }
}

export!(Component);
