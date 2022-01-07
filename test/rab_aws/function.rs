
		use lambda_runtime::{handler_fn, Context, Error};
		use serde_json::{json, Value};

		#[tokio::main]
		async fn main() -> Result<(), Error> {
			let func = handler_fn(func);
			lambda_runtime::run(func).await?;
			Ok(())
		}

		async fn func(event: Value, _: Context) -> Result<Value, Error> {
			let text = event["text"].as_str().unwrap_or("error1");
			let positions = event["positions"].as_str().unwrap_or("error2");
			let serialized = event["serialized"].as_str().unwrap_or("error3");

			Ok(json!({ "message": format!("I got {}, {}, {}!", text, positions, serialized) }))
		}
	