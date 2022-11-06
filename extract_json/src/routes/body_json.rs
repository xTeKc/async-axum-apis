use axum::Json;
use serde::{Deserialize, Serialize};

// extract json from body
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonBody {
    message: String,
}

// create new json and send out to client
#[derive(Serialize)]
pub struct JsonBodyTwo {
    message: String,
    server_msg: String,
}

// debug body_json
// pub async fn body_json(Json(body): Json<JsonBody>) {
//     dbg!(body);
//     todo!()
// }

// body_json
// pub async fn body_json(Json(body): Json<JsonBody>) -> Json<JsonBody> {
//     Json(body)
// }

// body_json_two
// pub async fn body_json_two(Json(body): Json<JsonBody>) -> Json<JsonBodyTwo> {
//     Json(JsonBodyTwo { message: body.message, server_msg: "Hello from Server".to_owned()})
// }

// body_json and body_json_two
pub async fn body_json(Json(body): Json<JsonBody>) -> Json<JsonBodyTwo> {
    Json(JsonBodyTwo { message: body.message, server_msg: "Hello from Server".to_owned()})
}