echo "Deleting resources created on AWS for the execution..."

echo "Deleting the lambda function rab_lambda..."
aws lambda delete-function --function-name rab_lambda

echo "Deleting the SQS queue rab_queue..."
queue_url=$(aws sqs get-queue-url --queue-name rab_queue --query "QueueUrl")
aws sqs delete-queue --queue-url $queue_url

echo "Deleting the IAM role rab_role..."
aws iam delete-role-policy --role-name rab_role --policy-name rab_policy
aws iam delete-role --role-name rab_role