use assert_matches::assert_matches;
use csv_schema_validator::{ValidateCsv, ValidationError};
use serde::Deserialize;

#[derive(Deserialize, ValidateCsv, Debug)]
struct TestRecord {
    #[validate(range(min = 0.0, max = 100.0))]
    grade: f64,

    #[validate(regex = r"^[A-Z]{3}\d{4}$")]
    code: String,

    #[validate(required)]
    name: Option<String>,

    #[validate(custom = "length_validation")]
    comments: String,
}

fn length_validation(s: &str) -> Result<(), String> {
    if s.len() <= 10 {
        Ok(())
    } else {
        Err("too long".into())
    }
}

#[test]
fn test_valid_record() {
    let record = TestRecord { grade: 75.0, code: "ABC1234".to_string(), name: Some("John".into()), comments: "ok".into() };
    assert_matches!(record.validate_csv(), Ok(()));
}

#[test]
fn test_invalid_grade() {
    let record = TestRecord { grade: 150.0, code: "ABC1234".to_string(), name: Some("John".into()), comments: "ok".into() };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(errors[0], ValidationError { field: "grade".to_string(), message: "value out of expected range: 0 to 100".to_string() });
}

#[test]
fn test_invalid_regex() {
    let record = TestRecord { grade: 50.0, code: "abc1234".to_string(), name: Some("John".into()), comments: "ok".into() };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(errors[0].field, "code");
    assert!(errors[0].message.contains("pattern"));
}

#[test]
fn test_required_missing() {
    let record = TestRecord { grade: 50.0, code: "ABC1234".to_string(), name: None, comments: "ok".into() };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(errors[0].field, "name");
}

#[test]
fn test_custom_validator() {
    let record = TestRecord { grade: 50.0, code: "ABC1234".to_string(), name: Some("John".into()), comments: "too long indeed".into() };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(errors[0].field, "comments");
    assert_eq!(errors[0].message, "too long");
}
