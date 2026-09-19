-- Oryza-Elo Edge SQLite Schema Migration: 001_initial_schema.sql
-- Confinement: strictly inside src/dal/data/local/oryza_elo_edge.db

CREATE TABLE IF NOT EXISTS parcels (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    rice_variety TEXT NOT NULL,
    rice_ecosystem TEXT NOT NULL,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    planting_date TEXT NOT NULL,
    area_hectares REAL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS device_mappings (
    id TEXT PRIMARY KEY,
    device_name TEXT NOT NULL,
    manufacturer TEXT NOT NULL,
    is_preset INTEGER NOT NULL DEFAULT 0,
    date_col TEXT NOT NULL,
    date_format TEXT NOT NULL DEFAULT '%Y-%m-%d',
    t_max_col TEXT NOT NULL,
    t_max_unit TEXT NOT NULL DEFAULT 'C',
    t_max_scale REAL NOT NULL DEFAULT 1.0,
    t_min_col TEXT NOT NULL,
    t_min_unit TEXT NOT NULL DEFAULT 'C',
    t_min_scale REAL NOT NULL DEFAULT 1.0,
    rain_col TEXT NOT NULL,
    rain_unit TEXT NOT NULL DEFAULT 'mm',
    rain_scale REAL NOT NULL DEFAULT 1.0,
    rad_col TEXT NOT NULL,
    rad_unit TEXT NOT NULL DEFAULT 'MJ/m2',
    rad_scale REAL NOT NULL DEFAULT 1.0,
    rh_col TEXT NOT NULL,
    rh_unit TEXT NOT NULL DEFAULT '%',
    rh_scale REAL NOT NULL DEFAULT 1.0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS weather_records (
    parcel_id TEXT NOT NULL,
    record_date TEXT NOT NULL,
    t_max REAL NOT NULL,
    t_min REAL NOT NULL,
    precipitation_mm REAL NOT NULL,
    radiation_mj_m2 REAL NOT NULL,
    relative_humidity_pct REAL NOT NULL,
    source TEXT NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (parcel_id, record_date),
    FOREIGN KEY (parcel_id) REFERENCES parcels(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_weather_parcel_date ON weather_records(parcel_id, record_date DESC);

CREATE TABLE IF NOT EXISTS prediction_history (
    id TEXT PRIMARY KEY,
    parcel_id TEXT NOT NULL,
    evaluated_at TEXT NOT NULL,
    macro_phase TEXT NOT NULL,
    granular_stage TEXT NOT NULL,
    confidence REAL NOT NULL,
    is_transitioning INTEGER NOT NULL,
    probabilities_json TEXT NOT NULL,
    advisory_json TEXT NOT NULL,
    metrics_json TEXT NOT NULL,
    FOREIGN KEY (parcel_id) REFERENCES parcels(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_predictions_parcel_eval ON prediction_history(parcel_id, evaluated_at DESC);

CREATE TABLE IF NOT EXISTS edge_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
