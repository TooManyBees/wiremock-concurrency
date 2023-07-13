use axum::body::Full;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Router};
use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Request, StatusCode};
use once_cell::sync::Lazy;
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::{self, filter::Targets, prelude::*};

static CLIENT: Lazy<Client<HttpConnector>> = Lazy::new(|| {
    let mut connector = HttpConnector::new();
    connector.set_connect_timeout(Some(Duration::from_secs(2)));
    Client::builder().build(connector)
});

#[derive(Clone, Debug)]
struct ExternalUrls {
    url_1: String,
    url_2: String,
}

fn init_tracing_subscriber() {
    let targets = Targets::new()
        .with_target("hyper", Level::INFO)
        .with_default(Level::DEBUG);

    let reg = tracing_subscriber::registry();

    #[cfg(not(test))]
    let layer = tracing_subscriber::fmt::layer().pretty();
    #[cfg(test)]
    let layer = tracing_subscriber::fmt::layer().with_test_writer().pretty();

    reg.with(layer.with_filter(targets)).init();
}

async fn handler(State(external_urls): State<ExternalUrls>) -> Response {
    let request_1 = Request::builder()
        .method(Method::GET)
        .uri(&external_urls.url_1)
        .body(Body::from("body"))
        .expect("Coudln't build external request 1");

    let response = match CLIENT.request(request_1).await {
        Ok(r) => r,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(format!("Error making request 1\n{}", e)))
                .expect("Couldn't build error response 1")
                .into_response()
        }
    };

    let request_2 = Request::builder()
        .method(Method::GET)
        .uri(&external_urls.url_2)
        .body(Body::from("body"))
        .expect("Coudln't build external request 2");

    let response = match CLIENT.request(request_2).await {
        Ok(r) => r,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(format!("Error making request 2\n{}", e)))
                .expect("Couldn't build error response 2")
                .into_response()
        }
    };

    response.into_response()
}

fn make_router(url_1: &str, url_2: &str) -> Router {
    let state = ExternalUrls {
        url_1: url_1.to_string(),
        url_2: url_2.to_string(),
    };
    Router::new()
        .route("/", get(handler))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() {
    init_tracing_subscriber();

    let app = make_router(
        "http://toomanybees.github.io/beglitch",
        "http://toomanybees.github.io/beglitch",
    );

    axum::Server::bind(&([0, 0, 0, 0], 8888).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::{init_tracing_subscriber, make_router};
    use axum::Router;
    use hyper::{http::StatusCode, Body, Method, Request};
    use once_cell::sync::Lazy;
    use tower::util::ServiceExt;
    use wiremock::{matchers::method, Mock, MockServer, Respond, ResponseTemplate};

    static TRACING: Lazy<()> = Lazy::new(|| {
        init_tracing_subscriber();
    });

    #[tokio::test]
    async fn it_works_00() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_01() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_02() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_03() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_04() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_05() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_06() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_07() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_08() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_09() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_10() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_11() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_12() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_13() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_14() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_15() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_16() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_17() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_18() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_19() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_20() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_21() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_22() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_23() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_24() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_25() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_26() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_27() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_28() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_29() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_30() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_31() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_32() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_33() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_34() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_35() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_36() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_37() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_38() {
        test_app().await;
    }

    #[tokio::test]
    async fn it_works_39() {
        test_app().await;
    }

    async fn test_app() {
        Lazy::force(&TRACING);

        let app = build_default_router().await;

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/")
                    .body(Body::empty())
                    .expect("Couldn't build test request"),
            )
            .await
            .expect("Couldn't make test request");

        assert_eq!(response.status(), StatusCode::OK);
    }

    async fn build_default_router() -> Router {
        let external_server_1 = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ExternalServer)
            .mount(&external_server_1)
            .await;

        let external_server_2 = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ExternalServer)
            .mount(&external_server_2)
            .await;

        let router = make_router(&external_server_1.uri(), &external_server_2.uri());

        router
    }

    struct ExternalServer;

    impl Respond for ExternalServer {
        fn respond(&self, _req: &wiremock::Request) -> ResponseTemplate {
            ResponseTemplate::new(200).set_body_bytes("Got it!")
        }
    }
}
