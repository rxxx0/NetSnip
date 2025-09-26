use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRecord {
    pub id: String,
    pub mac: String,
    pub ip: String,
    pub hostname: Option<String>,
    pub custom_name: Option<String>,
    pub manufacturer: Option<String>,
    pub device_type: String,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub total_bytes: i64,
    pub is_blocked: bool,
    pub bandwidth_limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEvent {
    pub id: i64,
    pub event_type: String,
    pub device_id: String,
    pub timestamp: DateTime<Utc>,
    pub details: Option<String>,
}

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(db_path: &str) -> Result<Self> {
        let connection_string = format!("sqlite://{}?mode=rwc", db_path);
        let pool = SqlitePool::connect(&connection_string).await?;

        let db = Self { pool };
        db.create_tables().await?;
        Ok(db)
    }

    async fn create_tables(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS devices (
                id TEXT PRIMARY KEY,
                mac TEXT UNIQUE NOT NULL,
                ip TEXT NOT NULL,
                hostname TEXT,
                custom_name TEXT,
                manufacturer TEXT,
                device_type TEXT NOT NULL,
                first_seen DATETIME NOT NULL,
                last_seen DATETIME NOT NULL,
                total_bytes INTEGER DEFAULT 0,
                is_blocked BOOLEAN DEFAULT FALSE,
                bandwidth_limit REAL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS network_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                event_type TEXT NOT NULL,
                device_id TEXT NOT NULL,
                timestamp DATETIME NOT NULL,
                details TEXT,
                FOREIGN KEY (device_id) REFERENCES devices(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_device(&self, device: &DeviceRecord) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO devices (
                id, mac, ip, hostname, custom_name, manufacturer,
                device_type, first_seen, last_seen, total_bytes,
                is_blocked, bandwidth_limit
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(mac) DO UPDATE SET
                ip = excluded.ip,
                hostname = excluded.hostname,
                custom_name = COALESCE(excluded.custom_name, custom_name),
                manufacturer = excluded.manufacturer,
                device_type = excluded.device_type,
                last_seen = excluded.last_seen,
                total_bytes = excluded.total_bytes,
                is_blocked = excluded.is_blocked,
                bandwidth_limit = excluded.bandwidth_limit
            "#,
        )
        .bind(&device.id)
        .bind(&device.mac)
        .bind(&device.ip)
        .bind(&device.hostname)
        .bind(&device.custom_name)
        .bind(&device.manufacturer)
        .bind(&device.device_type)
        .bind(&device.first_seen)
        .bind(&device.last_seen)
        .bind(device.total_bytes)
        .bind(device.is_blocked)
        .bind(device.bandwidth_limit)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_device_by_mac(&self, mac: &str) -> Result<Option<DeviceRecord>> {
        let device = sqlx::query_as!(
            DeviceRecord,
            "SELECT * FROM devices WHERE mac = ?",
            mac
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(device)
    }

    pub async fn get_all_devices(&self) -> Result<Vec<DeviceRecord>> {
        let devices = sqlx::query_as!(DeviceRecord, "SELECT * FROM devices ORDER BY last_seen DESC")
            .fetch_all(&self.pool)
            .await?;

        Ok(devices)
    }

    pub async fn log_event(&self, event: NetworkEvent) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO network_events (event_type, device_id, timestamp, details)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(&event.event_type)
        .bind(&event.device_id)
        .bind(&event.timestamp)
        .bind(&event.details)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let result = sqlx::query_scalar::<_, String>(
            "SELECT value FROM settings WHERE key = ?"
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO settings (key, value) VALUES (?, ?)
            ON CONFLICT(key) DO UPDATE SET value = excluded.value
            "#,
        )
        .bind(key)
        .bind(value)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}