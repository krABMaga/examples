
		use lambda_runtime::{handler_fn, Context, Error};
		use serde_json::{json, Value};
        use aws_config::meta::region::RegionProviderChain;
        use aws_sdk_sqs::Client;

		#[tokio::main]
        async fn main() -> Result<(), Error> {
            let func = handler_fn(func);
            lambda_runtime::run(func).await?;
			Ok(())
        }

		async fn func(event: Value, _: Context) -> Result<Value, Error> {
			let region_provider = RegionProviderChain::default_provider();
            let config = aws_config::from_env().region(region_provider).load().await;
            let client = Client::new(&config);
            let msg = event["text"].as_str().unwrap_or("nothing");
            let result = send_receive(&client, msg.to_string()).await;
			Ok(json!({ "message": format!("{:#?}", result) }))
		}
        
        async fn send_receive(client: &Client, results: String) -> Result<String, Error> {
            let mut response: String = String::new();
            let pkg = env!("CARGO_PKG_NAME");
            let queue = client.get_queue_url().queue_name(pkg.to_string()).send().await?;
            let queue_url = match queue.queue_url {
                Some(url) => url,
                None => {
                    response.push_str(&format!("No queues in this account. Please create a queue to proceed"));
                    "null".to_string()
                }
            };
        
            response.push_str(&format!(
                "Sending and receiving messages on with URL: `{}`",
                queue_url
            ));

            let send_request = client
                .send_message()
                .queue_url(queue_url)
                .message_body(results)
                .send()
                .await?;

            response.push_str(&format!("Response from sending a message: {:#?}", send_request));

            Ok(response)
        }
	