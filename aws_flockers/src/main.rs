use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::fs;


fn main() {
	// Command::new("cargo")
	// 	.arg("build")
    //     .arg("--release")
	// 	.arg("--bin")
	// 	.arg("function")
    //     .arg("--target")
	// 	.arg("x86_64-unknown-linux-gnu")
    //     .spawn()
    //     .expect("command failed to start");


	let data = "#!/bin/bash \n
cargo build --release --bin function --target x86_64-unknown-linux-gnu\n
cp ./target/x86_64-unknown-linux-gnu/release/function ./bootstrap && zip lambda.zip bootstrap && rm bootstrap \n
aws lambda create-function --function-name rustab_function --handler test --zip-file fileb://./lambda.zip --runtime provided.al2 --role arn:aws:iam::865590474135:role/lambda-sqs-execution --environment Variables={RUST_BACKTRACE=1} --tracing-config Mode=Active \n	
aws lambda invoke --cli-binary-format raw-in-base64-out --function-name rustab_function --payload '{\"check\": \"success\"}' --log-type Tail output.json";
	
	fs::write("src/rab_aws.sh", data).expect("Unable to write file");
	
	Command::new("src/aws.sh").spawn().expect("command failed");
	
	// let mut file = File::create("rab_aws.sh");
	// file.write_all(b"#!/bin/bash \n 
	// cargo build --release --bin function --target x86_64-unknown-linux-gnu\n
	// cp ./target/x86_64-unknown-linux-gnu/release/function ./bootstrap && zip lambda.zip bootstrap && rm bootstrap \n
	// aws lambda create-function --function-name rustab_function --handler test --zip-file fileb://./lambda.zip --runtime provided.al2 --role arn:aws:iam::865590474135:role/lambda-sqs-execution --environment Variables={RUST_BACKTRACE=1} --tracing-config Mode=Active \n	
	// aws lambda invoke --cli-binary-format raw-in-base64-out --function-name rustab_function --payload '{\"check\": \"success\"}' --log-type Tail output.json");

	// Command::new("src/aws.sh").spawn().expect("command failed");

	// "aws
	// lambda
	// create-function
	// --function-name 
	// --handler test
	// --zip-file fileb://./function.zip --runtime provided.al2 --role arn:aws:iam::865590474135:role/lambda-sqs-execution --environment Variables={RUST_BACKTRACE=1} --tracing-config Mode=Active"
	// "cp ./target/x86_64-unknown-linux-gnu/release/function ./bootstrap && zip function.zip bootstrap && rm bootstrap"
}