# rive-gateway

A client for the Revolt WebSocket API.

It is used for the Rive crates ecosystem and is built on top of it. See the [`rive`](https://docs.rs/rive) documentation for more information.

## Features

- `native-tls` (default): enables TLS support via [`native-tls`](https://crates.io/crates/native-tls).
- `rustls-tls-native-roots`: enables TLS support via [`rustls`](https://crates.io/crates/rustls) with native root certificates.
- `rustls-tls-webpki-roots`: enables TLS support via [`native-tls`](https://crates.io/crates/native-tls) with root certificates from [`webpki-roots`](https://crates.io/crates/webpki-roots).

**Note**: if the self-hosted Revolt instance does not use a `wss://` connection, then you can disable TLS support by disabling the default features:

```toml
[dependencies]
rive-gateway = { version = "1", default-features = false }
```

The official instance is WSS only.
