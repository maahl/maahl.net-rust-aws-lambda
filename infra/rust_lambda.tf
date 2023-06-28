module "lambda_function" {
  source = "terraform-aws-modules/lambda/aws"

  function_name = "rust-aws-lambda"
  description   = "Create an AWS Lambda in Rust with Terraform"
  runtime       = "provided.al2"
  architectures = ["arm64"]
  handler       = "bootstrap"

  create_package         = false
  local_existing_package = "../target/lambda/rust-aws-lambda/bootstrap.zip"
}
