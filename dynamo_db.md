# DynamoDB
```shell
docker container run -itd \
  --name bumr_dynamodb \
  -p 0.0.0.0:8000:8000 \
  amazon/dynamodb-local:latest \
  -jar DynamoDBLocal.jar -sharedDb

aws dynamodb list-tables \
  --endpoint-url http://localhost:8000 --region eu-central-1
aws dynamodb describe-table --table-name BumrSession \
  --endpoint-url http://localhost:8000 --region eu-central-1
```

## Session table
```shell
aws dynamodb create-table \
  --table-name BumrSession \
  --attribute-definitions AttributeName=Id,AttributeType=S \
  --key-schema AttributeName=Id,KeyType=HASH \
  --provisioned-throughput ReadCapacityUnits=1,WriteCapacityUnits=1 \
  --endpoint-url http://localhost:8000 --region eu-central-1
```