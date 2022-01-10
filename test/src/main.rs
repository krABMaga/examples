

use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use futures::executor::block_on;


use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs;
use aws_sdk_lambda::{ByteStream};
use tokio::runtime::Runtime; // 0.3.5

// #[tokio::main]
// async fn main() {
fn main() {

	// array of string containing the messages sent by the functions
	let result: Vec<String> = Vec::new();

	// blocking function for async
	let result = Runtime::new().unwrap().block_on(function_lambda());

	println!("Result len is {}", result.as_ref().unwrap().len());
	for msg in result.as_ref().unwrap(){
		println!("Result is {:?}", msg);
	}
}

async fn function_lambda() -> Result<Vec<String>, aws_sdk_lambda::Error>{
	
	let mut result: Vec<String> = Vec::new();

	// configuration of the different aws clients
	let region_provider = RegionProviderChain::default_provider();
	let config = aws_config::from_env().region(region_provider).load().await;

	// create the sqs client
	let client_sqs = aws_sdk_sqs::Client::new(&config);

	// create a queue where functions will publish the results
	// the queue name is the package name of the Cargo.toml
	let pkg = env!("CARGO_PKG_NAME");
	let create_queue = client_sqs.create_queue().queue_name(pkg).send().await;

	// create the lambda client
	let client_lambda = aws_sdk_lambda::Client::new(&config);
	
	// invoke the function using the sub_population json as payload
	for i in 0..5{
		client_lambda
			.invoke_async()
			.function_name("rustab_function")
			.invoke_args(ByteStream::from(format!("{{\"text\": \"msg{}\"}}", i).as_bytes().to_vec()))
			.send().await;
	}

	// receive the message from the queue to get the results from the functions
	let result = send_receive(&client_sqs).await;
	let to_write: Vec<String> = result.unwrap();
	Ok(to_write)
}

async fn send_receive(client: &aws_sdk_sqs::Client) -> Result<Vec<String>, aws_sdk_sqs::Error> {
    let queue = client.get_queue_url().set_queue_name(Some("aws_flockers".to_string())).send().await?;
    let queue_url = queue.queue_url.unwrap_or_default();
    // let queue_url = match queue_urls.first() {
    //     Some(url) => url,
    //     None => {
    //         panic!("No queues in this account. Please create a queue to proceed");
    //     }
    // };

    // println!(
    //     "Sending and receiving messages on with URL: `{}`",
    //     queue_url
    // );

	// let mut file = File::open("/home/giuseppe/Git/rab/rust-ab-examples/sir_ga_exploration/rab_aws/result.json").expect("Cannot open json file!");
	// let mut contents = String::new();
	// file.read_to_string(&mut contents);

	// let json: serde_json::Value = serde_json::from_str(&contents).expect("Cannot parse the json file!");

    // let rsp = client
    //     .send_message()
    //     .queue_url(queue_url.clone())
    //     .message_body("O mazz e mammt")
    //     .send()
    //     .await?;

	// let rsp = client
    //     .send_message()
    //     .queue_url(queue_url.clone())
    //     .message_body("O mazz e sort")
    //     .send()
    //     .await?;


    // println!("Response from sending a message: {:#?}", rsp);
	let mut num_msg = 15;
	let mut msg: Vec<String> = Vec::new();

	while num_msg > 0 {
		let request = client.receive_message();
		let request = request.queue_url(queue_url.clone());
		let request = request.send().await?;

		let mut receipts: Vec<String> = Vec::new();
		for message in request.messages.unwrap_or_default() {
			// println!("Got the message: {:?}", message);
			msg.push(message.body.unwrap());
			receipts.push(message.receipt_handle.unwrap());
		}
		for rec in receipts{
			let request_del = client.delete_message();
			let request_del = request_del.queue_url(queue_url.clone());
			let request_del = request_del.receipt_handle(rec);
			let request_del = request_del.send().await?;
		}
		num_msg -= 1;
	}

	// println!("Message number is {}", msg.len());
	// for mess in msg{
	// 	// let json: serde_json::Value = serde_json::from_str(&mess).expect("Cannot parse the json file!");

	// 	// let stuff = json["function"].as_array().unwrap();

	// 	// println!("Stuff is {}", stuff[0]["Fitness"]);
	// 	println!("Message is {}", mess);
	// }
	
    Ok(msg)
}

// fn main() {
// 	Command::new("mkdir").arg("rab_aws").output().expect("Failed to create rab_aws folder!");

// 	let array = [0,1,2,3,4];
// 	let param2 = "ciao";
// 	let serialized_data = serde_json::to_string(&array).unwrap();


// 	let params = format!(r#"
// 		{{
// 			"positions": "{:?}",
// 			"text": "{}",
// 			"serialized": "{}"
// 		}}
// 	"#, array, param2, serialized_data);

// 	fs::write("rab_aws/parameters.json", params).expect("Unable to write parameters.json file.");

// 	let function = format!(r#"
// 		use lambda_runtime::{{handler_fn, Context, Error}};
// 		use serde_json::{{json, Value}};

// 		#[tokio::main]
// 		async fn main() -> Result<(), Error> {{
// 			let func = handler_fn(func);
// 			lambda_runtime::run(func).await?;
// 			Ok(())
// 		}}

// 		async fn func(event: Value, _: Context) -> Result<Value, Error> {{
// 			let text = event["text"].as_str().unwrap_or("error1");
// 			let positions = event["positions"].as_str().unwrap_or("error2");
// 			let serialized = event["serialized"].as_str().unwrap_or("error3");

// 			Ok(json!({{ "message": format!("I got {{}}, {{}}, {{}}!", text, positions, serialized) }}))
// 		}}
// 	"#,
// 	);

// 	fs::write("rab_aws/function.rs", function).expect("Unable to write function.rs file.");

// 	let script_deploy = "#!/bin/bash\n
// cargo build --release --bin function --target x86_64-unknown-linux-gnu\n
// cp ./target/x86_64-unknown-linux-gnu/release/function ./bootstrap && zip rab_aws/lambda.zip bootstrap && rm bootstrap \n
// aws lambda create-function --function-name rustab_function --handler test --zip-file fileb://rab_aws/lambda.zip --runtime provided.al2 --role arn:aws:iam::865590474135:role/lambda-sqs-execution --environment Variables={RUST_BACKTRACE=1} --tracing-config Mode=Active \n
// ";
	
// 	fs::write("rab_aws/rab_aws_deploy.sh", script_deploy).expect("Unable to write deploy script file.");

// 	let script_invoke = "#!/bin/bash\n
// 	aws lambda invoke --cli-binary-format raw-in-base64-out --function-name rustab_function --payload file://rab_aws/parameters.json rab_aws/output.json
// 	";
	
// 	fs::write("rab_aws/rab_aws_invoke.sh", script_invoke).expect("Unable to write invoke script file.");

// 	Command::new("bash").arg("rab_aws/rab_aws_deploy.sh").output().expect("Launch of deploy script failed");
		
// 	Command::new("bash").arg("rab_aws/rab_aws_invoke.sh").output().expect("Launch of invoke script failed");
// }