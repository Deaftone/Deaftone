use chrono::Utc;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use sqlx::{sqlite::SqliteQueryResult, Pool};
use std::{collections::HashSet, net::IpAddr, thread::sleep};
use tokio::time::Duration;
use tokio::time::Instant;
use uuid::Uuid;

use crate::database;

pub mod device;

pub struct Mdns {
    service_name: String,
    sqlite_pool: Pool<sqlx::Sqlite>, // This is an ugly workaround for: https://github.com/keepsimple1/mdns-sd/issues/145
}
pub const CHROMECAST_SERVICE_NAME: &str = "_googlecast._tcp.local.";
pub const AIRPLAY_SERVICE_NAME: &str = "_raop._tcp.local.";
#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct Device {
    id: String,
    name: String,
    address_v4: String,
    created_at: String,
    updated_at: String,
}
impl Mdns {
    pub async fn new(application_name: &str) -> Result<Self, anyhow::Error> {
        let sqlite_pool = match database::connect_db_sqlx().await {
            Ok(pool) => pool,
            Err(_) => database::connect_db_sqlx().await.unwrap(),
        };
        Ok(Self {
            service_name: application_name.to_string(),
            sqlite_pool,
        })
    }

    pub async fn discover(&self) {
        // Create a daemon

        let mdns = ServiceDaemon::new().expect("Failed to create daemon");
        let receiver = mdns.browse(&self.service_name).expect("Failed to browse");
        loop {
            tracing::debug!("Starting dns discovery...");
            let start_time = Instant::now();
            let max_duration = Duration::from_secs(60);
            while let Ok(event) = receiver.recv() {
                match event {
                    ServiceEvent::ServiceResolved(info) => {
                        tracing::debug!("Resolved a new service: {}", info.get_fullname());
                        if let Some(first_ipv4) = Self::find_first_ipv4(info.get_addresses()) {
                            tracing::debug!("First IPv4 address: {}", first_ipv4);
                            Self::insert_or_update_device(
                                info.get_fullname(),
                                &first_ipv4.to_string(),
                                &self.sqlite_pool,
                            )
                            .await
                            .expect("Failed to insert_or_update_device");
                        } else {
                            tracing::debug!("No IPv4 address found in the set");
                        }
                    }
                    other_event => {
                        tracing::trace!("Received other event: {:?}", &other_event);
                    }
                }

                // Check if 30 seconds have elapsed
                if start_time.elapsed() >= max_duration {
                    break; // Exit the loop if 30 seconds have passed
                }
            }
            tracing::debug!("Sleeping dns discovery...");
            sleep(Duration::from_secs(5 * 60)); // Sleep for 5 minutes before restarting the loop
        }
    }

    fn find_first_ipv4(ip_set: &HashSet<IpAddr>) -> Option<IpAddr> {
        ip_set.iter().find(|&&ip_addr| ip_addr.is_ipv4()).copied()
    }

    async fn insert_or_update_device(
        device_name: &str,
        new_address_v4: &str,
        db: &Pool<sqlx::Sqlite>,
    ) -> Result<SqliteQueryResult, anyhow::Error> {
        let init_time: String = Utc::now().naive_local().to_string();

        let existing_device =
            sqlx::query_as::<_, Device>("SELECT * FROM cast_devices WHERE name = ?")
                .bind(device_name)
                .persistent(true)
                .fetch_optional(db)
                .await?;
        if let Some(mut device) = existing_device {
            // Device exists, update address_v4
            device.address_v4 = new_address_v4.to_string();
            tracing::debug!("Updating device: {:?}", &device.name);

            Ok(
                sqlx::query("UPDATE cast_devices SET address_v4 = ?, updated_at = ? WHERE id = ?")
                    .bind(device.address_v4)
                    .bind(init_time)
                    .bind(device.id)
                    .execute(db)
                    .await?,
            )
        } else {
            tracing::debug!("Creating device: {:?}", &device_name);
            Ok(sqlx::query(
                "INSERT INTO cast_devices (
                        id,
                        name,
                        address_v4,
                        created_at,
                        updated_at
                    )
                    VALUES (?,?,?,?,?)",
            )
            .bind(Uuid::new_v4().to_string())
            .bind(device_name)
            .bind(new_address_v4)
            .bind(&init_time)
            .bind(&init_time)
            .execute(db)
            .await?)
        }
    }
}
