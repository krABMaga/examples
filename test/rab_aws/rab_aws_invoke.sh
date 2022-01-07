#!/bin/bash

	aws lambda invoke --cli-binary-format raw-in-base64-out --function-name rustab_function --payload file://rab_aws/parameters.json rab_aws/output.json
	