use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};

use anyhow::anyhow;

use crate::services::http::error::ApiError;
#[derive(Clone)]
pub struct DeviceService {
    db: DatabaseConnection,
}
impl DeviceService {
    pub fn new(db: DatabaseConnection) -> DeviceService {
        DeviceService { db }
    }
    pub async fn get_cast_device_by_id(
        &self,
        cast_device_id: &str,
    ) -> Result<entity::cast_devices::Model, ApiError> {
        match entity::cast_devices::Entity::find_by_id(cast_device_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                e
            })? {
            Some(cast_device) => Ok(cast_device.clone()),
            None => Err(ApiError(
                StatusCode::NOT_FOUND,
                anyhow!("Unable to find Device with id: {}", cast_device_id),
            )),
        }
    }
}
