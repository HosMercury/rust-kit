use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::borrow::Cow;
use validator::ValidationErrors;

#[derive(Deserialize, Serialize)]
pub struct MyError {
    pub field: String,
    pub message: String,
}

pub fn validation_errors(errors: ValidationErrors) -> Vec<MyError> {
    let field_errors = errors.field_errors();
    let mut return_errs: Vec<MyError> = vec![];

    for (field, errs) in field_errors {
        let field = field.to_case(Case::Camel);
        for err in errs {
            return_errs.push(MyError {
                field: field.clone(),
                message: err.message.clone().unwrap_or(Cow::Borrowed("")).into(),
            });
        }
    }

    return_errs
}

pub fn general_error(message: &str) -> Value {
    json!([{ "message": message }])
}
