use deaftone::handlers::albums::*;
use std::fs;
use utoipa::OpenApi;
fn main() {
    let doc = gen_my_openapi();
    match fs::write("./api_doc.json", doc) {
        Ok(_) => {
            println!("Successfully write api_doc.json")
        }
        Err(e) => {
            println!("Failed to write api_doc.json {:}", e)
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
            deaftone::handlers::artists::get_artists,
            deaftone::handlers::artists::get_artist
        ),
        components(
            schemas(
                deaftone::handlers::albums::GetAllAlbums,
                deaftone::handlers::albums::AlbumResponse,
                entity::album::Model,
                deaftone::handlers::artists::GetAllArtists,
                entity::artist::Model,
            )
        ),
        tags(
            (name = "deaftone::handlers::albums", description = "Deaftone Albums API"),
            (name = "deaftone::handlers::artists", description = "Deaftone Artists API")
            //(name = "deaftone", description = "Deaftone API")
        )
    )]
    #[openapi(
        paths(
            get_albums,
            get_album,
        ),
        components(
            schemas(
                deaftone::handlers::albums::GetAllAlbumsQuery,
                deaftone::handlers::albums::AlbumResponse,
                entity::album::Model,
                deaftone::handlers::artists::GetArtistsQuery,
                entity::artist::Model,
            )
        ),
        tags(
            (name = "Album Api", description = "Deaftone API")
        )
    )]
    struct ApiDoc;
    ApiDoc::openapi().to_pretty_json().unwrap()
}
