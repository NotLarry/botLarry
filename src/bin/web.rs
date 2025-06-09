use axum::{response::Html, routing::get, Router};
use rusqlite::Connection;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use log::{info, warn, error, debug};


#[tokio::main]
async fn main() {
    let conn = Arc::new(Mutex::new(Connection::open("data/calls.db").unwrap()));
    let app = Router::new().route("/", get({
        let conn = conn.clone();
        move || async move { show_phone_list(conn).await }
    }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("🌐 Web server running at http://{}/", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

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

