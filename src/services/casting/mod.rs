use async_stream::stream;
use chrono::Utc;
use futures_util::{Stream, StreamExt};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use sqlx::{sqlite::SqliteQueryResult, Pool};
use std::{
    collections::{HashMap, HashSet},
    net::IpAddr,
    sync::Arc,
};
use tokio::time;
use tokio::time::Duration;
use tokio::{sync::Mutex, time::Instant};
use uuid::Uuid;

use crate::database;
pub const SERVICE_NAME: &str = "_googlecast._tcp.local.";

pub fn discover(service_name: &str) -> impl Stream<Item = ServiceInfo> {
    let mdns = ServiceDaemon::new().unwrap();
    let receiver = mdns.browse(service_name).expect("Failed to browse");

    stream! {
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    yield info;
                }
                _ => {
                    tracing::debug!("Failed to find devices");
                }
            }
        }
    }
}

pub async fn run_discover() {
    let devices = Arc::new(Mutex::new(HashMap::new()));
    let services = discover(SERVICE_NAME);
    let sqlite_pool = match database::connect_db_sqlx().await {
        Ok(pool) => pool,
        Err(_) => database::connect_db_sqlx().await.unwrap(),
    };
    tokio::pin!(services);
    let start_time = Instant::now();
    let duration_limit = Duration::from_secs(2);
    loop {
        if let Some(info) = services.next().await {
            // Update the array with the current timestamp
            let mut devices = devices.lock().await;
            devices.insert(info.get_fullname().to_owned(), time::Instant::now());
            if let Some(first_ipv4) = find_first_ipv4(info.get_addresses()) {
                tracing::debug!("First IPv4 address: {}", first_ipv4);
                let q = insert_or_update_device(
                    info.get_fullname(),
                    &first_ipv4.to_string(),
                    &sqlite_pool,
                )
                .await
                .unwrap();

                tracing::debug!("{:?}", q);
            } else {
                tracing::debug!("No IPv4 address found in the set");
            }
            tracing::debug!("Processing device: {:?}", info);

            devices.retain(|_name, timestamp| {
                tracing::debug!("Checking device: {}", _name);
                let elapsed = timestamp.elapsed();
                elapsed < Duration::from_secs(60)
            });
            // Check if one minute has passed
            if start_time.elapsed() >= duration_limit {
                tracing::info!("Stopping casting search");
                break;
            }
        }
    }
}

fn find_first_ipv4(ip_set: &HashSet<IpAddr>) -> Option<IpAddr> {
    ip_set.iter().find(|&&ip_addr| ip_addr.is_ipv4()).copied()
}
async fn insert_or_update_device(
    device_name: &str,
    address_v4: &String,
    db: &Pool<sqlx::Sqlite>,
) -> Result<SqliteQueryResult, anyhow::Error> {
    let init_time: String = Utc::now().naive_local().to_string();

    Ok(sqlx::query(
        "INSERT OR REPLACE INTO cast_devices (
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
    .bind(address_v4)
    .bind(&init_time)
    .bind(&init_time)
    .execute(db)
    .await?)
}
