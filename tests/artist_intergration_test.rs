#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use deaftone::test_util::app;
    use std::net::TcpListener;
    #[tokio::test]
    async fn test_get_artists() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind ephemeral socket");
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(app().await.into_make_service())
                .await
                .unwrap();
        });

        let client = hyper::Client::new();

        let resp = client
            .request(
                Request::builder()
                    .uri(format!("http://{}/artists", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), hyper::StatusCode::OK);
        let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let artists: Vec<entity::artist::Model> = serde_json::from_slice(&body).unwrap();

        // Assert that the returned artists are sorted by name
        let mut prev_name = String::new();
        for artist in &artists {
            assert!(artist.name >= prev_name);
            prev_name = artist.name.clone();
        }
    }
}
