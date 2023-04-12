# Rive

![Rive banner](./assets/rive_banner.png)

Rive is a simple yet powerful and flexible ecosystem of Rust crates for the [Revolt](https://revolt.chat) API. It implements the entire Revolt API and is suitable for creating custom clients or bots.

The main principle of implementation is simplicity and straightforwardness. This makes it easier to understand and easier to contribute.

The ecosystem includes [`rive-models`](./rive-models/), [`rive-http`](./rive-http/), [`rive-gateway`](./rive-gateway/) and more. These are explained below.

The main [`rive`](./rive/) crate just re-exports these crates. Using crates individually is recommended though.

## MSRV

The minimum supported Rust version is 1.64.

## Crates

These are crates that most users will use together for a full development experience. You may not need all of these, but they are often used together to accomplish most of what you need.

- [`rive-models`](./rive-models/)

    Models defining structures, enumerations and bitflags of all Revolt API entities. Models are split into sub-modules, for example `user` for containing types specific to the user's entity, `event` for containing events coming from WebSocket or `payload` for containing fields used in API requests.

    Models can be serialized or deserialized using [`Serde`](https://serde.rs).

- [`rive-http`](./rive-http/)

    HTTP client supporting all of the Revolt REST API. It is based on [`reqwest`](https://docs.rs/reqwest/).

- [`rive-gateway`](./rive-gateway/)

    Implementation of Revolts WebSocket API. This is responsible for receiving events in real-time from Revolt and sending *some* information. It is based on [`tokio-tungstenite`](https://docs.rs/tokio-tungstenite).

    Please note that the client itself does not do a heartbeat (periodic ping to keep the connection alive), so this has to be done manually, for example by making an async task.

## Credits

- Models were taken directly from the Revolt's [backend](https://github.com/revoltchat/backend).
- This project is heavily inspired by [`twilight`](https://github.com/twilight-rs/twilight/) <3

## License

[GNU Lesser General Public License v2.1](./LICENSE)
