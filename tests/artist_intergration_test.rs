#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request, Server};
    use chrono::{NaiveDateTime, Utc};
    use deaftone::{handlers::ArtistResponse, test_util::app};
    use hyper::{body::to_bytes, Client, StatusCode};
    use serde_json::from_slice;
    use std::net::TcpListener;

    #[tokio::test]
    async fn test_get_artist() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind ephemeral socket");
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(app().await.into_make_service())
                .await
                .unwrap();
        });

        let client = Client::new();

        let resp = client
            .request(
                Request::builder()
                    .uri(format!(
                        "http://{}/artists/7d110590-c4ed-4250-973b-f8fa5d60260e",
                        addr
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body()).await.unwrap();
        let artist: ArtistResponse = from_slice(&body).unwrap();
        assert!(artist.id == r#"7d110590-c4ed-4250-973b-f8fa5d60260e"#);
        assert!(artist.name == String::from("Akon"));
        assert!(artist.albums.len() == 6);
    }
    #[tokio::test]
    async fn test_get_artists_sort_by_name() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind ephemeral socket");
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            Server::from_tcp(listener)
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

        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body()).await.unwrap();
        let artists: Vec<entity::artist::Model> = serde_json::from_slice(&body).unwrap();

        // Assert that the returned artists are sorted by name
        let mut prev_name = String::new();
        for artist in &artists {
            assert!(artist.name >= prev_name);
            prev_name = artist.name.clone();
        }
    }
    #[tokio::test]
    async fn test_get_artists_sort_by_latest() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind ephemeral socket");
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            Server::from_tcp(listener)
                .unwrap()
                .serve(app().await.into_make_service())
                .await
                .unwrap();
        });

        let client = hyper::Client::new();

        let resp = client
            .request(
                Request::builder()
                    .uri(format!("http://{}/artists?sort=latest", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body()).await.unwrap();
        let artists: Vec<entity::artist::Model> = serde_json::from_slice(&body).unwrap();

        // Assert that the returned artists are sorted by name
        let mut created_at: NaiveDateTime = Utc::now().naive_local();
        for artist in &artists {
            let now_parsed: NaiveDateTime = artist.created_at;
            assert!(now_parsed <= created_at);
            created_at = now_parsed.clone();
        }
    }
    #[tokio::test]
    async fn test_get_artists_paginate() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind ephemeral socket");
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            Server::from_tcp(listener)
                .unwrap()
                .serve(app().await.into_make_service())
                .await
                .unwrap();
        });

        let client = hyper::Client::new();

        let page = client
            .request(
                Request::builder()
                    .uri(format!("http://{}/artists?page=0&size=4", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let page_one = client
            .request(
                Request::builder()
                    .uri(format!("http://{}/artists?page=0&size=2", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let page_two = client
            .request(
                Request::builder()
                    .uri(format!("http://{}/artists?page=1&size=2", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(page.status(), StatusCode::OK);
        let page_body = to_bytes(page.into_body()).await.unwrap();
        let page_artists: Vec<entity::artist::Model> = serde_json::from_slice(&page_body).unwrap();

        assert_eq!(page_one.status(), StatusCode::OK);
        let page_one_body = to_bytes(page_one.into_body()).await.unwrap();
        let page_one_artists: Vec<entity::artist::Model> =
            serde_json::from_slice(&page_one_body).unwrap();

        assert_eq!(page_two.status(), StatusCode::OK);
        let page_two_body = to_bytes(page_two.into_body()).await.unwrap();
        let page_two_artists: Vec<entity::artist::Model> =
            serde_json::from_slice(&page_two_body).unwrap();
        println!("{:?}\n\n", page_artists);
        println!("{:?}\n\n", page_one_artists);

        assert_eq!(page_artists.len(), 4);
        assert_eq!(page_one_artists.len(), 2);
        assert_eq!(page_two_artists.len(), 2);

        assert_eq!(page_one_artists[0], page_artists[0]);
        assert_eq!(page_one_artists[1], page_artists[1]);
        assert_eq!(page_two_artists[0], page_artists[2]);
        assert_eq!(page_two_artists[1], page_artists[3]);
    }
}
