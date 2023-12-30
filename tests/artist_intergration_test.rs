#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use chrono::{NaiveDateTime, Utc};
    use deaftone::{
        handlers::{ArtistResponse, GetResposne},
        test_util::{app, ADDR},
    };
    use http_body_util::BodyExt;
    use hyper::StatusCode;
    use serde_json::from_slice;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_artist() {
        let app = app().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!(
                        "http://{ADDR}/artists/7d110590-c4ed-4250-973b-f8fa5d60260e"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let artist: GetResposne<ArtistResponse> = from_slice(&body).unwrap();
        assert!(artist.data.id == r#"7d110590-c4ed-4250-973b-f8fa5d60260e"#);
        assert!(artist.data.name == *"Akon");
        assert!(artist.data.albums.len() == 6);
    }
    #[tokio::test]
    async fn test_get_artist_not_found() {
        let app = app().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!(
                        "http://{ADDR}/artists/7d110590-c4ed-4250-973b-f8fa5d60260ee"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    #[tokio::test]
    async fn test_get_artists_sort_by_name() {
        let app = app().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("http://{ADDR}/artists"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let artists: GetResposne<Vec<entity::artist::Model>> = from_slice(&body).unwrap();
        // Assert that the returned artists are sorted by name
        let mut prev_name = String::new();
        for artist in &artists.data {
            assert!(artist.name >= prev_name);
            prev_name = artist.name.clone();
        }
    }
    #[tokio::test]
    async fn test_get_artists_sort_by_latest() {
        let app = app().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("http://{ADDR}/artists?sort=latest"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let artists: GetResposne<Vec<entity::artist::Model>> =
            serde_json::from_slice(&body).unwrap();

        // Assert that the returned artists are sorted by name
        let mut created_at: NaiveDateTime = Utc::now().naive_local();
        for artist in &artists.data {
            let now_parsed: NaiveDateTime = artist.created_at;
            assert!(now_parsed <= created_at);
            created_at = now_parsed;
        }
    }
    #[tokio::test]
    async fn test_get_artists_paginate() {
        let app = app().await;
        let page = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("http://{ADDR}/artists?page=0&size=4"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let page_one = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("http://{ADDR}/artists?page=0&size=2"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let page_two = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("http://{ADDR}/artists?page=1&size=2"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(page.status(), StatusCode::OK);
        let page_body = page.into_body().collect().await.unwrap().to_bytes();

        let page_artists: GetResposne<Vec<entity::artist::Model>> =
            serde_json::from_slice(&page_body).unwrap();

        assert_eq!(page_one.status(), StatusCode::OK);
        let page_one_body = page_one.into_body().collect().await.unwrap().to_bytes();

        let page_one_artists: GetResposne<Vec<entity::artist::Model>> =
            serde_json::from_slice(&page_one_body).unwrap();

        assert_eq!(page_two.status(), StatusCode::OK);
        let page_two_body = page_two.into_body().collect().await.unwrap().to_bytes();

        let page_two_artists: GetResposne<Vec<entity::artist::Model>> =
            serde_json::from_slice(&page_two_body).unwrap();

        assert_eq!(page_artists.data.len(), 4);
        assert_eq!(page_one_artists.data.len(), 2);
        assert_eq!(page_two_artists.data.len(), 2);

        assert_eq!(page_one_artists.data[0], page_artists.data[0]);
        assert_eq!(page_one_artists.data[1], page_artists.data[1]);
        assert_eq!(page_two_artists.data[0], page_artists.data[2]);
        assert_eq!(page_two_artists.data[1], page_artists.data[3]);
    }
}
