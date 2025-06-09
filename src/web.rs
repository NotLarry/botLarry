use axum::{response::Html, routing::get, Router};
use rusqlite::Connection;
use std::{sync::Arc};
use tokio::sync::Mutex;

pub fn create_router(conn: Arc<Mutex<Connection>>) -> Router {
    Router::new().route("/", get({
        let conn = conn.clone();
        move || async move { show_phone_list(conn).await }
    }))
}

async fn show_phone_list(conn: Arc<Mutex<Connection>>) -> Html<String> {
    let conn = conn.lock().await;
    let mut stmt = conn
        .prepare("SELECT areacode, phonenumber, recording_path FROM calls ORDER BY timestamp DESC")
        .unwrap();

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .unwrap();

    let mut html = String::from("<h1>📞 Valid Phone Numbers</h1><ul>");
    for row in rows {
        let (area, number, note) = row.unwrap();
        html.push_str(&format!(
            "<li>({}) {} — <em>{}</em></li>",
            area, number, note
        ));
    }
    html.push_str("</ul>");
    Html(html)
}

