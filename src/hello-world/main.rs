use lambda_http::{service_fn, tower::BoxError, Error, IntoResponse, Request, RequestExt};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service_func = service_fn(hello);
    lambda_http::run(service_func).await?;
    Ok(())
}

async fn hello(request: Request) -> Result<impl IntoResponse, BoxError> {
    let _context = request.lambda_context();
    let name = request
        .query_string_parameters()
        .first("name")
        .unwrap_or("world")
        .to_string();

    let res = json!({ "message": format!("hello {}", name) });
    Ok(res)
}

// #[tokio::test]
// async fn test_hello() {
//     let request = Request::default();
//     let response = hello(request, Context::default())
//         .await
//         .unwrap()
//         .into_response();
//     assert_eq!(
//         response.body(),
//         &lambda_http::Body::Text("Hello, World!".to_string())
//     );
// }
