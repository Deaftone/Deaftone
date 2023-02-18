use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub results: Vec<Result>,
    #[serde(skip_deserializing)]
    pub info: Info,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub gender: String,
    pub name: Name,
    #[serde(skip_deserializing)]
    pub location: Location,
    #[serde(skip_deserializing)]
    pub email: String,
    pub login: Login,
    #[serde(skip_deserializing)]
    pub dob: Dob,
    pub registered: Registered,
    pub phone: String,
    #[serde(skip_deserializing)]
    pub cell: String,
    #[serde(skip_deserializing)]
    pub id: Id,
    pub picture: Picture,
    #[serde(skip_deserializing)]
    pub nat: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub title: String,
    pub first: String,
    pub last: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postcode: i64,
    pub coordinates: Coordinates,
    pub timezone: Timezone,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timezone {
    pub offset: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dob {
    pub date: String,
    pub age: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registered {
    pub date: String,
    pub age: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Id {
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Picture {
    pub large: String,
    pub medium: String,
    pub thumbnail: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub seed: String,
    pub results: i64,
    pub page: i64,
    pub version: String,
}
