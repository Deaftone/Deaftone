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
        paths(
            deaftone::handlers::albums::get_albums,
            deaftone::handlers::albums::get_album,
            deaftone::handlers::albums::get_cover,
            deaftone::handlers::artists::get_artists,
            deaftone::handlers::artists::get_artist,
            deaftone::handlers::songs::get_song,
            deaftone::handlers::streams::stream_handler,
        ),
        components(
            schemas(
                deaftone::handlers::GetAllAlbums,
                deaftone::handlers::AlbumResponse,
                deaftone::handlers::ArtistResponse,
                deaftone::handlers::SongResponse,
                deaftone::handlers::GetAllArtists,
                entity::album::Model,
                entity::song::Model,
                entity::artist::Model,
            )
        ),
        tags(
            (name = "deaftone::handlers::albums", description = "Deaftone Albums API"),
            (name = "deaftone::handlers::artists", description = "Deaftone Artists API")
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
