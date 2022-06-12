use lambda_runtime::{handler_fn, Context};
use serde::Serialize;
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Serialize)]
pub struct SimpleResponse {
    pub name: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct ApiGatewayEvent {
    pub body: Event,
}

#[derive(Deserialize)]
pub struct Event {
    pub message: String,
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let handler_callback = handler_fn(world);
    lambda_runtime::run(handler_callback).await?;
    Ok(())
}

async fn world(_: ApiGatewayEvent, _: Context) -> Result<SimpleResponse, Error> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    Ok(SimpleResponse {
        name: "world".to_string(),
        message: "World custom message".to_string(),
    })
}
