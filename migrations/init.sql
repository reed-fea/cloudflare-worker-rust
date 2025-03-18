-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    openid TEXT NOT NULL,
    phone TEXT,
    nickname TEXT,
    avatar TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create wifi table
CREATE TABLE IF NOT EXISTS wifi (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    password TEXT NOT NULL,
    ssid TEXT NOT NULL,
    bssid TEXT,
    security TEXT NOT NULL,
    frequency REAL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Create wifi_qrcode table
CREATE TABLE IF NOT EXISTS wifi_qrcode (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    wifi_id INTEGER NOT NULL,
    qr_data TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (wifi_id) REFERENCES wifi(id)
);

-- Insert some sample data
INSERT INTO users (openid, phone, nickname, avatar) 
VALUES ('user123', '13800138000', 'Test User', 'https://example.com/avatar.jpg');

-- Create index
CREATE INDEX IF NOT EXISTS idx_wifi_user_id ON wifi(user_id);
CREATE INDEX IF NOT EXISTS idx_wifi_ssid ON wifi(ssid);
CREATE INDEX IF NOT EXISTS idx_wifi_bssid ON wifi(bssid);

-- Add comment
PRAGMA table_info(wifi); 