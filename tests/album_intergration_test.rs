#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use chrono::{NaiveDateTime, Utc};
    use deaftone::{
        services::http::{handlers::AlbumResponse, SuccessResponse},
        test_util::{app, ADDR},
    };
    use http_body_util::BodyExt;
    use hyper::StatusCode;
    use serde_json::from_slice;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_album() {
        let app = app().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!(
                        "http://{ADDR}/albums/46ffbb9a-8c98-45d6-a561-0cb80214a642"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let response: SuccessResponse<AlbumResponse> = from_slice(&body).unwrap();
        assert!(response.message.id == r#"46ffbb9a-8c98-45d6-a561-0cb80214a642"#);
        assert!(response.message.name == *"Ain't No Peace");
        assert!(response.message.songs.len() == 7);
    }

    #[tokio::test]
    async fn test_get_album_not_found() {
        let app = app().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!(
                        "http://{ADDR}/albums/46ffbb9a-8c98-45d6-a561-0cb80214a642a"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    #[tokio::test]
    async fn test_get_albums_sort_by_name() {
        let app = app().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("http://{ADDR}/albums"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let albums: SuccessResponse<Vec<entity::album::Model>> =
            serde_json::from_slice(&body).unwrap();

        // Assert that the returned artists are sorted by name
        let mut prev_name = String::new();
        for album in &albums.message {
            assert!(album.name >= prev_name);
            prev_name = album.name.clone();
        }
    }
    #[tokio::test]
    async fn test_get_albums_sort_by_latest() {
        let app = app().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("http://{ADDR}/albums?sort=latest"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();

        let albums: SuccessResponse<Vec<entity::album::Model>> =
            serde_json::from_slice(&body).unwrap();
        // Assert that the returned artists are sorted by name
        let mut created_at: NaiveDateTime = Utc::now().naive_local();
        for album in &albums.message {
            let now_parsed: NaiveDateTime = album.created_at;
            assert!(now_parsed <= created_at);
            created_at = now_parsed;
        }
    }
    #[tokio::test]
    async fn test_get_albums_paginate() {
        let app = app().await;

        let page = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("http://{ADDR}/albums?page=0&size=4"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let page_one = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("http://{ADDR}/albums?page=0&size=2"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let page_two = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("http://{ADDR}/albums?page=1&size=2"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(page.status(), StatusCode::OK);
        let page_body = page.into_body().collect().await.unwrap().to_bytes();
        let page_albums: SuccessResponse<Vec<entity::album::Model>> =
            serde_json::from_slice(&page_body).unwrap();

        assert_eq!(page_one.status(), StatusCode::OK);
        let page_one_body = page_one.into_body().collect().await.unwrap().to_bytes();
        let page_one_albums: SuccessResponse<Vec<entity::album::Model>> =
            serde_json::from_slice(&page_one_body).unwrap();

        assert_eq!(page_two.status(), StatusCode::OK);
        let page_two_body = page_two.into_body().collect().await.unwrap().to_bytes();

        let page_two_albums: SuccessResponse<Vec<entity::album::Model>> =
            serde_json::from_slice(&page_two_body).unwrap();

        assert_eq!(page_albums.message.len(), 4);
        assert_eq!(page_one_albums.message.len(), 2);
        assert_eq!(page_two_albums.message.len(), 2);

        assert_eq!(page_one_albums.message[0], page_albums.message[0]);
        assert_eq!(page_one_albums.message[1], page_albums.message[1]);
        assert_eq!(page_two_albums.message[0], page_albums.message[2]);
        assert_eq!(page_two_albums.message[1], page_albums.message[3]);

        // Assert that the returned artists are sorted by name
        /*         let mut prev_name = String::new();
        for album in &albums {
            assert!(album.name >= prev_name);
            prev_name = album.name.clone();
        } */
    }
}
