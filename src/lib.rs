mod models;

use worker::*;
use serde_json::json;
use serde_wasm_bindgen::to_value;
use crate::models::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    // 创建路由器
    let router = Router::new();
    
    // 获取 WiFi 信息
    router.get_async("/api/wifi/:id", |_req, ctx| async move {
        let id = match ctx.param("id") {
            Some(id_str) => match id_str.parse::<i64>() {
                Ok(id) => id,
                Err(_) => return Response::error("Invalid ID", 400),
            },
            None => return Response::error("Missing ID", 400),
        };
        
        let db = ctx.env.d1("DB")?;
        
        // 使用 prepare 和 first 执行查询
        let stmt = db.prepare("SELECT * FROM wifi WHERE id = ?1");
        let query = match stmt.bind(&[to_value(&id)?]) {
            Ok(q) => q,
            Err(e) => {
                console_log!("Failed to bind parameters: {:?}", e);
                return Response::error("Failed to bind parameters", 500);
            }
        };
        
        match query.first::<WiFi>(None).await {
            Ok(Some(wifi)) => {
                let response = json!(ApiResponse::success(wifi));
                Response::from_json(&response)
            }
            Ok(None) => {
                let response = json!(ApiResponse::<()>::error(404, "WiFi not found".into()));
                Ok(Response::from_json(&response)?.with_status(404))
            }
            Err(e) => {
                console_log!("Database error: {:?}", e);
                Response::error("Database error", 500)
            }
        }
    })
    // 创建 WiFi 记录
    .post_async("/api/wifi", |mut req, ctx| async move {
        // 解析请求体
        let create_req: CreateWiFiRequest = match req.json().await {
            Ok(req) => req,
            Err(e) => {
                console_log!("Failed to parse request body: {:?}", e);
                return Response::error("Invalid request body", 400);
            }
        };
        
        console_log!("Received request: {:?}", create_req);
        
        let db = ctx.env.d1("DB")?;
        
        // 使用 prepare 和 first 执行插入
        let stmt = db.prepare(
            "INSERT INTO wifi (user_id, name, password, ssid, bssid, security, frequency, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, DATETIME('now'), DATETIME('now')) RETURNING id"
        );
        
        let params = vec![
            to_value(&create_req.user_id)?,
            to_value(&create_req.name)?,
            to_value(&create_req.password)?,
            to_value(&create_req.ssid)?,
            to_value(&create_req.bssid.unwrap_or_default())?,
            to_value(&create_req.security)?,
            to_value(&create_req.frequency.unwrap_or(0.0))?,
        ];
        
        console_log!("Binding parameters: {:?}", params);
        
        let query = match stmt.bind(&params) {
            Ok(q) => q,
            Err(e) => {
                console_log!("Failed to bind parameters: {:?}", e);
                return Response::error("Failed to bind parameters", 500);
            }
        };
        
        match query.first::<LastInsertId>(None).await {
            Ok(Some(result)) => {
                console_log!("Successfully inserted with ID: {}", result.id);
                let response = json!(ApiResponse::success(result.id));
                Response::from_json(&response)
            }
            Ok(None) => {
                console_log!("Failed to get last inserted ID");
                Response::error("Failed to get last inserted ID", 500)
            }
            Err(e) => {
                console_log!("Database error: {:?}", e);
                Response::error("Database error", 500)
            }
        }
    })
    .run(req, env)
    .await
}

// 用于获取最后插入 ID 的辅助结构体
#[derive(serde::Deserialize)]
struct LastInsertId {
    id: i64,
}
