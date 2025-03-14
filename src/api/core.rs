use axum::Json;

pub async fn get_memory(Json(payload): Json<Vec<i32>>) -> Json<Vec<i32>> {
    Json(payload.iter().map(|x| x * 2).collect())
}

pub async fn get_registers(Json(payload): Json<Vec<i32>>) -> Json<Vec<i32>> {
    Json(payload.iter().map(|x| x * 2).collect())
}

pub async fn post_run(Json(payload): Json<Vec<i32>>) -> Json<Vec<i32>> {
    Json(payload.iter().map(|x| x * 2).collect())
}
