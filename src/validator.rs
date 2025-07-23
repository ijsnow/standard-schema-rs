use ::validator::ValidationErrors;

use crate::*;

impl From<ValidationErrors> for FailureResult {
    fn from(errors: ValidationErrors) -> Self {
        let mut issues = vec![];

        for (key, errors) in errors.field_errors() {
            for error in errors {
                issues.push(Issue {
                    message: error
                        .message
                        .as_ref()
                        .map(|message| message.to_string())
                        .unwrap_or_else(|| error.code.to_string()),
                    path: Some(vec![IssuePath::PropertyKey(PropertyKey::String(
                        key.to_string(),
                    ))]),
                });
            }
        }

        Self { issues }
    }
}
