#!/bin/bash

cargo build --release --bin function --target x86_64-unknown-linux-gnu

cp ./target/x86_64-unknown-linux-gnu/release/function ./bootstrap && zip lambda.zip bootstrap && rm bootstrap

aws lambda create-function --function-name rustab_function --handler test --zip-file fileb://./lambda.zip --runtime provided.al2 --role arn:aws:iam::865590474135:role/lambda-sqs-execution --environment Variables={RUST_BACKTRACE=1} --tracing-config Mode=Active

aws lambda invoke --cli-binary-format raw-in-base64-out --function-name rustab_function --payload '{"check": "success"}' --log-type Tail output.json