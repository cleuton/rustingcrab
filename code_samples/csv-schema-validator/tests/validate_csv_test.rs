use assert_matches::assert_matches;
use csv_schema_validator::{ValidateCsv, ValidationError};
use serde::Deserialize;

#[derive(Deserialize, ValidateCsv, Debug)]
struct TestRecord {
    #[validate(range(min = 0.0, max = 100.0))]
    grade: f64,

    #[validate(regex = r"^[A-Z]{3}\d{4}$")]
    code: String,

    #[validate(required, length(min = 10, max = 50), not_blank)]
    name: Option<String>,

    #[validate(custom = "length_validation")]
    comments: String,

    #[validate(required, one_of("short", "medium", "long"))]
    more_comments: Option<String>,

    #[validate(required, not_in("forbidden", "banned"))]
    tag: Option<String>,
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
    let record = TestRecord {
        grade: 75.0,
        code: "ABC1234".to_string(),
        name: Some("John Smith Jr".into()),
        comments: "ok".into(),
        more_comments: Some("short".into()),
        tag: Some("allowed".into()),
    };
    assert_matches!(record.validate_csv(), Ok(()));
}

#[test]
fn test_invalid_grade() {
    let record = TestRecord {
        grade: 150.0,
        code: "ABC1234".to_string(),
        name: Some("John Smith Jr".into()),
        comments: "ok".into(),
        more_comments: Some("medium".into()),
        tag: Some("allowed".into()),
    };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(
        errors[0],
        ValidationError {
            field: "grade".to_string(),
            message: "value out of expected range: 0 to 100".to_string()
        }
    );
}

#[test]
fn test_invalid_regex() {
    let record = TestRecord {
        grade: 50.0,
        code: "abc1234".to_string(),
        name: Some("John Smith Jr".into()),
        comments: "ok".into(),
        more_comments: Some("long".into()),
        tag: Some("allowed".into()),
    };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(errors[0].field, "code");
    assert!(errors[0].message.contains("pattern"));
}

#[test]
fn test_required_name_missing() {
    let record = TestRecord {
        grade: 50.0,
        code: "ABC1234".to_string(),
        name: None,
        comments: "ok".into(),
        more_comments: Some("short".into()),
        tag: Some("allowed".into()),
    };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(errors[0].field, "name");
}

#[test]
fn test_blank_name() {
    let record = TestRecord {
        grade: 50.0,
        code: "ABC1234".to_string(),
        name: Some("  \n \t".into()),
        comments: "ok".into(),
        more_comments: Some("short".into()),
        tag: Some("allowed".into()),
    };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(
        errors[0],
        ValidationError {
            field: "name".to_string(),
            message: "length out of expected range: 10 to 50".to_string()
        }
    );
}

#[test]
fn test_invalid_name_length() {
    let record = TestRecord {
        grade: 80.0,
        code: "ABC1234".to_string(),
        name: Some("John".into()),
        comments: "ok".into(),
        more_comments: Some("medium".into()),
        tag: Some("allowed".into()),
    };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(
        errors[0],
        ValidationError {
            field: "name".to_string(),
            message: "length out of expected range: 10 to 50".to_string()
        }
    );
}

#[test]
fn test_custom_validator() {
    let record = TestRecord {
        grade: 50.0,
        code: "ABC1234".to_string(),
        name: Some("John Smith Jr".into()),
        comments: "too long indeed".into(),
        more_comments: Some("long".into()),
        tag: Some("allowed".into()),
    };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(errors[0].field, "comments");
    assert_eq!(errors[0].message, "too long");
}

#[test]
fn test_more_comments_missing() {
    let record = TestRecord {
        grade: 50.0,
        code: "ABC1234".to_string(),
        name: Some("John Smith Jr".into()),
        comments: "ok".into(),
        more_comments: None,
        tag: Some("allowed".into()),
    };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(errors[0].field, "more_comments");
    assert_eq!(errors[0].message, "mandatory field");
}

#[test]
fn test_more_comments_invalid_value() {
    let record = TestRecord {
        grade: 50.0,
        code: "ABC1234".to_string(),
        name: Some("John Smith Jr".into()),
        comments: "short ok".into(),
        more_comments: Some("banana".into()),
        tag: Some("allowed".into()),
    };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(errors[0].field, "more_comments");
    assert_eq!(errors[0].message, "invalid value");
}

#[test]
fn test_tag_missing() {
    let record = TestRecord {
        grade: 50.0,
        code: "ABC1234".to_string(),
        name: Some("John Smith Jr".into()),
        comments: "short ok".into(),
        more_comments: Some("short".into()),
        tag: None,
    };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(errors[0].field, "tag");
    assert_eq!(errors[0].message, "mandatory field");
}

#[test]
fn test_tag_invalid_value() {
    let record = TestRecord {
        grade: 50.0,
        code: "ABC1234".to_string(),
        name: Some("John Smith Jr".into()),
        comments: "short ok".into(),
        more_comments: Some("short".into()),
        tag: Some("banned".into()),
    };
    let errors = record.validate_csv().unwrap_err();
    assert_eq!(errors[0].field, "tag");
    assert_eq!(errors[0].message, "value not allowed");
}
