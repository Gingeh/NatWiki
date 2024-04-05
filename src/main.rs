use std::fmt::Write;
use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::Html, routing::get, Router};
use num::{BigUint, Num};
use tokio::task::JoinSet;

mod nerds;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:n", get(handle_int));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_int(Path(param): Path<String>) -> Result<Html<String>, (StatusCode, String)> {
    let Ok(n) = BigUint::from_str_radix(&param, 10) else {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Error: {param:?} could not be parsed as an unsigned integer."),
        ));
    };
    let n = Arc::new(n);

    // Could/should we use scoped threads here?
    let mut tasks = JoinSet::new();
    for nerd in nerds::NERDS {
        let n = Arc::clone(&n);
        tasks.spawn_blocking(move || nerd(n));
    }

    let mut result = String::new();
    writeln!(result, "<h1>{n}</h1>\n<ul>").unwrap();
    while let Some(res) = tasks.join_next().await {
        if let Ok(Some(fact)) = res {
            writeln!(result, "<li>{fact}</li>").unwrap();
        }
    }
    write!(result, "</ul>").unwrap();
    Ok(Html(result))
}
