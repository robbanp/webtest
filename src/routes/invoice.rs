use axum::{Json, extract::{Path, Query}, http::{HeaderMap, header::USER_AGENT, StatusCode}, response::{IntoResponse, Response}};
use serde::{Serialize, Deserialize};

pub async fn create_invoice(Json(body): Json<Invoice>) -> Json<Invoice> {
    dbg!(&body);
    Json(body)
}

pub async fn get_invoice(
    Path(id): Path<u32>,
    Query(params): Query<QueryParams>,
    headers: HeaderMap
) -> Response {
    let user_agent = headers.get(USER_AGENT);
    let agent_str = user_agent.unwrap();
    println!("{:?}", agent_str);

    let invoice = Invoice {id: id,
        amount: 10,
        currency: "sek".into(),
        filter: match params.filter {
            Some(value) => value,
            None => "unknown".to_string(),
        },
        start: match params.start {
            Some(value) => value,
            None => 0,
        },
        end: match params.end {
            Some(value) => value,
            None => 0,
        },
    };
    if invoice.filter == "sek".to_owned() {
        return (
            StatusCode::CREATED,
            Json(invoice),
        ).into_response()
    } 
    return (StatusCode::UNPROCESSABLE_ENTITY, Json(ErrorMessage {message: "its bad".to_owned()})).into_response();    
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    pub message: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Invoice {
    /// Need to be smaller number
    amount: u16,
    pub currency: String,
    id: u32,
    filter: String,
    start: u32,
    end: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryParams {
    filter: Option<String>,
    start: Option<u32>,
    end: Option<u32>,
}