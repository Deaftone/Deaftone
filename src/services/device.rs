use sea_orm::{DatabaseConnection, EntityTrait};

use crate::ApiError;
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
            .all(&self.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                e
            })?
            .first()
        {
            Some(cast_device) => Ok(cast_device.clone()),
            None => Err(ApiError::RecordNotFound),
        }
    }
}
