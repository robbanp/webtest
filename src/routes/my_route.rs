use axum::response::Html;

pub async fn route1() -> Html<String> {
    Html(format!("Hey you"))
}