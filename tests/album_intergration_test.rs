#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request, Server};
    use chrono::{NaiveDateTime, Utc};
    use deaftone::{handlers::AlbumResponse, test_util::app};
    use hyper::{body::to_bytes, Client, StatusCode};
    use serde_json::from_slice;
    use std::net::TcpListener;

    #[tokio::test]
    async fn test_get_album() {
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
                        "http://{}/albums/da120a26-d886-4995-a9ee-4b558ed5fcf9",
                        addr
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body()).await.unwrap();
        let album: AlbumResponse = from_slice(&body).unwrap();
        assert!(album.id == r#"da120a26-d886-4995-a9ee-4b558ed5fcf9"#);
        assert!(album.name == String::from("Keeper Of The Seven Keys Part II"));
        assert!(album.songs.len() == 10);
    }
    #[tokio::test]
    async fn test_get_albums_sort_by_name() {
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
                    .uri(format!("http://{}/albums", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body()).await.unwrap();
        let albums: Vec<entity::album::Model> = serde_json::from_slice(&body).unwrap();

        // Assert that the returned artists are sorted by name
        let mut prev_name = String::new();
        for album in &albums {
            assert!(album.name >= prev_name);
            prev_name = album.name.clone();
        }
    }
    #[tokio::test]
    async fn test_get_albums_sort_by_latest() {
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
                    .uri(format!("http://{}/albums?sort=latest", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body()).await.unwrap();
        let albums: Vec<entity::album::Model> = serde_json::from_slice(&body).unwrap();
        // Assert that the returned artists are sorted by name
        let mut created_at: NaiveDateTime = Utc::now().naive_local();
        for album in &albums {
            let now_parsed: NaiveDateTime = album.created_at;
            assert!(now_parsed <= created_at);
            created_at = now_parsed.clone();
        }
    }
    #[tokio::test]
    async fn test_get_albums_paginate() {
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
                    .uri(format!("http://{}/albums?page=0&size=4", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let page_one = client
            .request(
                Request::builder()
                    .uri(format!("http://{}/albums?page=0&size=2", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let page_two = client
            .request(
                Request::builder()
                    .uri(format!("http://{}/albums?page=1&size=2", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(page.status(), StatusCode::OK);
        let page_body = to_bytes(page.into_body()).await.unwrap();
        let page_albums: Vec<entity::album::Model> = serde_json::from_slice(&page_body).unwrap();

        assert_eq!(page_one.status(), StatusCode::OK);
        let page_one_body = to_bytes(page_one.into_body()).await.unwrap();
        let page_one_albums: Vec<entity::album::Model> =
            serde_json::from_slice(&page_one_body).unwrap();

        assert_eq!(page_two.status(), StatusCode::OK);
        let page_two_body = to_bytes(page_two.into_body()).await.unwrap();
        let page_two_albums: Vec<entity::album::Model> =
            serde_json::from_slice(&page_two_body).unwrap();
        println!("{:?}\n\n", page_albums);
        println!("{:?}\n\n", page_one_albums);

        assert_eq!(page_albums.len(), 4);
        assert_eq!(page_one_albums.len(), 2);
        assert_eq!(page_two_albums.len(), 2);

        assert_eq!(page_one_albums[0], page_albums[0]);
        assert_eq!(page_one_albums[1], page_albums[1]);
        assert_eq!(page_two_albums[0], page_albums[2]);
        assert_eq!(page_two_albums[1], page_albums[3]);

        // Assert that the returned artists are sorted by name
        /*         let mut prev_name = String::new();
        for album in &albums {
            assert!(album.name >= prev_name);
            prev_name = album.name.clone();
        } */
    }
}
