use rusqlite::{Connection, Result};

pub fn open_database(path: &str) -> Result<Connection> {
    let connection = Connection::open(path)?;
    migrate(&connection)?;
    Ok(connection)
}

pub fn migrate(connection: &Connection) -> Result<()> {
    connection.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            machine_id TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS devices (
            id TEXT PRIMARY KEY,
            project_id TEXT REFERENCES projects(id) ON DELETE SET NULL,
            board_id TEXT NOT NULL,
            serial_number TEXT,
            port TEXT,
            firmware_version TEXT,
            configuration_version TEXT,
            created_at TEXT NOT NULL,
            last_seen_at TEXT
        );

        CREATE TABLE IF NOT EXISTS configurations (
            id TEXT PRIMARY KEY,
            device_id TEXT REFERENCES devices(id) ON DELETE CASCADE,
            schema_version TEXT NOT NULL,
            definition_version TEXT NOT NULL,
            config_json TEXT NOT NULL,
            config_hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS firmware_builds (
            id TEXT PRIMARY KEY,
            device_id TEXT REFERENCES devices(id) ON DELETE SET NULL,
            configuration_id TEXT REFERENCES configurations(id) ON DELETE SET NULL,
            version TEXT NOT NULL,
            board_fqbn TEXT NOT NULL,
            status TEXT NOT NULL,
            source_checksum TEXT,
            artifact_checksum TEXT,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS upload_history (
            id TEXT PRIMARY KEY,
            device_id TEXT REFERENCES devices(id) ON DELETE SET NULL,
            firmware_build_id TEXT REFERENCES firmware_builds(id) ON DELETE SET NULL,
            port TEXT,
            status TEXT NOT NULL,
            verification_status TEXT,
            output_summary TEXT,
            created_at TEXT NOT NULL
        );
        "#,
    )?;

    Ok(())
}
