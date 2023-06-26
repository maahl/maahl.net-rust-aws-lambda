use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

// Send a greeting
fn say_hello(name: Option<&str>) -> String {
    // if a name was not provided, address the greeting to "stranger"
    let name = name.unwrap_or("stranger");

    format!("Hello, {name}!")
}

// Wrapper for our core function
// Its role is to extract the relevant info from the incoming event, and convert the
// response to json.
async fn run_lambda(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();

    let name = event["name"].as_str();
    let result = say_hello(name);

    Ok(json!(result))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(run_lambda)).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::{Context, LambdaEvent};

    #[test]
    fn test_name_provided() {
        let name = "world";
        let result = say_hello(Some(name));
        assert_eq!(format!("Hello, {name}!"), result);
    }

    #[test]
    fn test_no_name_provided() {
        let result = say_hello(None);
        assert_eq!("Hello, stranger!".to_owned(), result);
    }

    #[tokio::test]
    async fn test_wrapper_name_provided() {
        let name = "world";
        let event = LambdaEvent::new(json!({ "name": name }), Context::default());
        let expected_result = json!(format!("Hello, {name}!"));

        let result = run_lambda(event).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, expected_result);
    }

    #[tokio::test]
    async fn test_wrapper_no_name_provided() {
        let event = LambdaEvent::new(
            json!({ "meaningless_key": "meaningless_value" }),
            Context::default(),
        );
        let expected_result = json!(format!("Hello, stranger!"));

        let result = run_lambda(event).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, expected_result);
    }
}
