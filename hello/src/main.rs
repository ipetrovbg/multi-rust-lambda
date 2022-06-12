use lambda_runtime::{handler_fn, Context};
use serde_json::Value;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_callback = handler_fn(hello);
    lambda_runtime::run(handler_callback).await?;

    Ok(())
}

async fn hello(event: Value, _: Context) -> Result<Value, Error> {
    Ok(event)
}