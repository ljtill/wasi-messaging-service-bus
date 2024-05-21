# WASI Messaging

This repository implements the WebAssembly System Interface (WASI) Messaging specification for Azure Service Bus using the Wasmtime runtime. The WASI Messaging specification provides a universal interface for WebAssembly modules to interact with message-oriented middleware, facilitating communication between WebAssembly modules and Azure Service Bus, a managed message broker for enterprise integration. Crafted in Rust, the implementation leverages Wasmtime, a standalone environment supporting WASI, to execute WebAssembly modules equipped with the WASI Messaging interface. The host is designed to manage multiple Azure Service Bus Queues concurrently, using a multi-threaded approach for processing of messages. The guest employs three channels — Abandon, Complete, and Redirect—to manage messages: Abandon for postponing message processing, Complete for successful processing, and Redirect for forwarding messages to different queues.

_Please note this repository is under development and subject to change._

```mermaid
sequenceDiagram
    participant Host
    participant Guest

    Host->>Guest: Call configure() function
    Guest-->>Host: Return GuestConfiguration

    loop Process
        Host->>Host: Monitor Queues
    end

    loop Process
        Guest->>Guest: Await Messages
    end

    Host->>Guest: Call handler() function

    Note right of Guest: Parse Messages

    Guest->>Host: Call connect() function
    Host-->>Guest: Return Client

    Note right of Guest: Channel A
    Guest-->>Host: Handle Messages (Abandon)

    Note right of Guest: Channel B
    Guest-->>Host: Handle Messages (Complete)

    Note right of Guest: Channel C
    Guest-->>Host: Handle Messages (Redirect)
```
