#!/bin/bash

echo "Function building..."
cargo build --release --bin function --target x86_64-unknown-linux-gnu

echo "Zipping the target for the upload..."
cp ./target/x86_64-unknown-linux-gnu/release/function ./bootstrap && zip rab_aws/rab_lambda.zip bootstrap && rm bootstrap 

echo "Checking that aws-cli is installed..."
which aws
if [ $? -eq 0 ]; then
      echo "aws-cli is installed, continuing..."
else
      echo "You need aws-cli to deploy the lambda function. Exiting...'"
      exit 1
fi

echo "Generating the json files required for lambda creation..."
echo '
{
	"Version": "2012-10-17",
	"Statement": [
		{
			"Effect": "Allow",
			"Action": [
				"sqs:*"
			],
			"Resource": "*" 
		},
		{
			"Effect":"Allow",
			"Action": [
				"logs:CreateLogGroup",
				"logs:CreateLogStream",
				"logs:PutLogEvents"
			],
			"Resource": "*"
		}
	]
}' > policy.json
	
echo '
{
	"Version": "2012-10-17",
	"Statement": [
		{
			"Effect": "Allow",
			"Principal": {
				"Service": "lambda.amazonaws.com"
				},
				"Action": "sts:AssumeRole" 
			},
		{
			"Effect": "Allow",
			"Action": [
				"sqs:*"
			],
			"Resource": "*" 
		},
		{
			"Effect":"Allow",
			"Action": [
				"logs:CreateLogGroup",
				"logs:CreateLogStream",
				"logs:PutLogEvents"
			],
			"Resource": "*"
		}
	
}' > rolePolicy.json

echo "Creation of IAM Role rab_role..."
role_arn=$(aws iam get-role --role-name rab_role --query 'Role.Arn')
if [ $? -ne 0 ]; then 
	role_arn=$(aws iam create-role --role-name rab_role --assume-role-policy-document file://rab_aws/rolePolicy.json --query 'Role.Arn')
fi
echo "IAM Role rab_role created at ARN "$role_arn

echo "Attacching policy to IAM Role..."	
aws iam put-role-policy --role-name rab_role --policy-name rab_policy --policy-document file://policy.json

echo "Creation of the lambda function..."
aws lambda create-function --function-name rab_lambda --handler test --zip-file fileb://rab_aws/rab_lambda.zip --runtime provided.al2 --role $role_arn --environment Variables={RUST_BACKTRACE=1} --tracing-config Mode=Active 
echo "Lambda function created successfully!"

echo "Clearing the rab_aws folder..."
rm -r rab_aws/