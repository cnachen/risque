use axum::Json;

pub async fn post_compile(Json(payload): Json<Vec<i32>>) -> Json<Vec<i32>> {
    Json(payload.iter().map(|x| x * 2).collect())
}
