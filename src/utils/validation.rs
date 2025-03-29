use convert_case::{Case, Casing};
use serde_json::{Map, Value, json};
use validator::ValidationErrors;

pub fn validation_errors(errors: ValidationErrors) -> Value {
    let mut error_map = Map::new();
    for (field, errors) in errors.field_errors().iter() {
        let camel_case_field = field.to_case(Case::Camel); // Convert to camelCase
        let messages: Vec<String> = errors
            .iter()
            .map(|err| err.message.clone().unwrap_or_default().to_string()) // Fix: Convert Cow<'_, str> to String
            .collect();
        error_map.insert(camel_case_field, json!(messages));
    }

    json!({
        "message": "The given data was invalid.",
        "errors": error_map
    })
}

pub fn general_error(message: &str) -> Value {
    json!({ "message": message })
}
