

use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;




fn main() {
	Command::new("mkdir").arg("rab_aws").output().expect("Failed to create rab_aws folder!");

	let array = [0,1,2,3,4];
	let param2 = "ciao";
	let serialized_data = serde_json::to_string(&array).unwrap();


	let params = format!(r#"
		{{
			"positions": "{:?}",
			"text": "{}",
			"serialized": "{}"
		}}
	"#, array, param2, serialized_data);

	fs::write("rab_aws/parameters.json", params).expect("Unable to write parameters.json file.");

	let function = format!(r#"
		use lambda_runtime::{{handler_fn, Context, Error}};
		use serde_json::{{json, Value}};

		#[tokio::main]
		async fn main() -> Result<(), Error> {{
			let func = handler_fn(func);
			lambda_runtime::run(func).await?;
			Ok(())
		}}

		async fn func(event: Value, _: Context) -> Result<Value, Error> {{
			let text = event["text"].as_str().unwrap_or("error1");
			let positions = event["positions"].as_str().unwrap_or("error2");
			let serialized = event["serialized"].as_str().unwrap_or("error3");

			Ok(json!({{ "message": format!("I got {{}}, {{}}, {{}}!", text, positions, serialized) }}))
		}}
	"#,
	);

	fs::write("rab_aws/function.rs", function).expect("Unable to write function.rs file.");

	let script_deploy = "#!/bin/bash\n
cargo build --release --bin function --target x86_64-unknown-linux-gnu\n
cp ./target/x86_64-unknown-linux-gnu/release/function ./bootstrap && zip rab_aws/lambda.zip bootstrap && rm bootstrap \n
aws lambda create-function --function-name rustab_function --handler test --zip-file fileb://rab_aws/lambda.zip --runtime provided.al2 --role arn:aws:iam::865590474135:role/lambda-sqs-execution --environment Variables={RUST_BACKTRACE=1} --tracing-config Mode=Active \n
";
	
	fs::write("rab_aws/rab_aws_deploy.sh", script_deploy).expect("Unable to write deploy script file.");

	let script_invoke = "#!/bin/bash\n
	aws lambda invoke --cli-binary-format raw-in-base64-out --function-name rustab_function --payload file://rab_aws/parameters.json rab_aws/output.json
	";
	
	fs::write("rab_aws/rab_aws_invoke.sh", script_invoke).expect("Unable to write invoke script file.");

	Command::new("bash").arg("rab_aws/rab_aws_deploy.sh").output().expect("Launch of deploy script failed");
		
	Command::new("bash").arg("rab_aws/rab_aws_invoke.sh").output().expect("Launch of invoke script failed");
}