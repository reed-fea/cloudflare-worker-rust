use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WiFi {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub password: String,
    pub ssid: String,
    pub bssid: Option<String>,
    pub security: String,
    pub frequency: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWiFiRequest {
    pub user_id: i64,
    pub name: String,
    pub password: String,
    pub ssid: String,
    pub bssid: Option<String>,
    pub security: String,
    pub frequency: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub openid: String,
    pub phone: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WiFiQRCode {
    pub id: i64,
    pub wifi_id: i64,
    pub qr_data: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub data: Option<T>,
    pub msg: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: Some(data),
            msg: None,
        }
    }

    pub fn error(code: i32, msg: String) -> Self {
        Self {
            code,
            data: None,
            msg: Some(msg),
        }
    }
} 