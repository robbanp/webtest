use axum::{extract::Query, response::Html, response::Json, routing::get, routing::post, Router};
use serde::{Deserialize, Serialize};
use rand::{thread_rng, Rng};

use self::my_route::route1;
mod my_route;

mod invoice;
use self::invoice::{create_invoice, get_invoice};

pub fn create_routes() -> Router {
    Router::new()
    .route("/rnd", get(handler))
    .route("/", get(root_path))
    .route("/hey", get(route1))
    .route("/invoice", post(create_invoice))
    .route("/invoice/:id", get(get_invoice))
}

// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Deserialize, Serialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

async fn root_path() -> Json<RangeParameters> {
    let obj = RangeParameters {start: 1, end: 2};

    Json(obj)
}
async fn handler(Query(range): Query<RangeParameters>) -> Html<String> {
    // Generate a random number in range parsed from query.
    let random_number = thread_rng().gen_range(range.start..range.end);

    // Send response in html format.
    Html(format!("<h1>Random Number: {}</h1>", random_number))
}