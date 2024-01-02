use std::fs;
use utoipa::OpenApi;
fn main() {
    let doc = gen_my_openapi();
    match fs::write("./api_doc.json", doc) {
        Ok(_) => {
            println!("Successfully write api_doc.json")
        }
        Err(err) => {
            println!("Failed to write api_doc.json {err}")
        }
    }
}

// in /src/openapi.rs
fn gen_my_openapi() -> std::string::String {
    #[derive(OpenApi)]
    #[openapi(
        info(description = "## About 
This api is inspired by https://auraspec.readthedocs.io/en/latest/api.html
### Response Format and Errors

The MIME type for all responses MUST be ``application/vnd.api+json``. Every response is a JSON object. When a request is successful, the document has a top-level key data corresponding to the response’s “primary data.” When it fails, the document has an errors key, which maps to an array of JSON API error objects. Other keys may also be present, as described below.

On a successfuly query
```
{
    status: {success}
    data: {Response}
}
```
On a failure
```
{
    status: {error},
    message: {String},
}
```
"),
        paths(
            deaftone::services::http::handlers::playlist::get_playlist,
            deaftone::services::http::handlers::albums::get_albums,
            deaftone::services::http::handlers::albums::get_album,
            deaftone::services::http::handlers::albums::get_cover,
            deaftone::services::http::handlers::artists::get_artists,
            deaftone::services::http::handlers::artists::get_artist,
            deaftone::services::http::handlers::songs::get_song,
            deaftone::services::http::handlers::streams::stream_handler,
        ),
        components(
            schemas(
                deaftone::services::http::handlers::PlayListResponse,
                deaftone::services::http::ArtistResponseOpenApi,
                deaftone::services::http::ArtistsResponseOpenApi,
                deaftone::services::http::AlbumResponseOpenApi,
                deaftone::services::http::AlbumsResponseOpenApi,
                deaftone::services::http::error::ErrorResponse,
                deaftone::services::http::error::Status,
                deaftone::services::http::handlers::GetAllAlbums,
                deaftone::services::http::handlers::AlbumResponse,
                deaftone::services::http::handlers::ArtistResponse,
                deaftone::services::http::handlers::SongResponse,
                deaftone::services::http::handlers::GetAllArtists,
                deaftone::services::http::handlers::ArtistLinks,
                entity::album::Model,
                entity::song::Model,
                entity::artist::Model,
            )
        ),
        tags(
            (name = "deaftone::services::http::handlers::albums", description = "Deaftone Albums API"),
            (name = "deaftone::services::http::handlers::artists", description = "Deaftone Artists API"),
            (name = "deaftone::services::http::handlers::playlists", description = "Deaftone Playlists API")

            //(name = "deaftone", description = "Deaftone API")
        )
    )]
    /*     #[openapi(
        paths(
            get_albums,
            get_album,
        ),
        components(
            schemas(
                deaftone::handlers::albums::GetAllAlbumsQuery,
                deaftone::handlers::albums::AlbumResponse,
                entity::album::Model as AlbumModel,
                deaftone::handlers::artists::GetArtistsQuery,
                entity::artist::Model as ArtistModel,
            )
        ),
        tags(
            (name = "Album Api", description = "Deaftone API")
        )
    )] */
    struct ApiDoc;
    ApiDoc::openapi().to_pretty_json().unwrap()
}
