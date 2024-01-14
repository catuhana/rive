use hyper_util::client::legacy::connect::HttpConnector;

#[cfg(any(
    all(
        feature = "native",
        any(feature = "rustls-native-roots", feature = "rustls-webpki-roots")
    ),
    all(
        feature = "rustls-native-roots",
        any(feature = "native", feature = "rustls-webpki-roots")
    ),
    all(
        feature = "rustls-webpki-roots",
        any(feature = "native", feature = "rustls-native-roots")
    ),
))]
compile_error!(
    r#"multiple features "native", "rustls-native-roots" and "rustls-webpki-roots" cannot be enabled at the same time"#
);

#[cfg(feature = "native")]
pub(crate) type Connector = hyper_tls::HttpsConnector<HttpConnector>;

#[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
pub(crate) type Connector = hyper_rustls::HttpsConnector<HttpConnector>;

#[cfg(not(any(
    feature = "native",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
)))]
pub(crate) type Connector = HttpConnector;

pub(crate) fn create_connector() -> Connector {
    let mut connector = HttpConnector::new();
    connector.enforce_http(false);

    #[cfg(feature = "native")]
    let connector = hyper_tls::HttpsConnector::new();

    #[cfg(feature = "rustls-native-roots")]
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .expect("no native root CA certificates found")
        .https_or_http()
        .enable_http1()
        .build();

    #[cfg(feature = "rustls-webpki-roots")]
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_or_http()
        .enable_http1()
        .build();

    connector
}
