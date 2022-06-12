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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn hello_handles() {
        let event = json!({
            "answer": 42
        });
        assert_eq!(
            hello(event.clone(), Context::default()).await.expect("expected Ok(_) values"),
            event
        )
    }
}