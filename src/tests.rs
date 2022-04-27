use crate::*;

fn convert_bool_to_json_value(val: bool) {
    assert_eq!(AMQPValue::Boolean(val).to_json_value(), Ok(Value::Bool(val)));
}

fn convert_short_string_to_json_value(string: &str) {
    assert_eq!(AMQPValue::ShortString(string.into()).to_json_value(), Ok(Value::String(string.to_string())));
}

fn convert_long_string_to_json_value(string: &str) {
    assert_eq!(AMQPValue::LongString(string.into()).to_json_value(), Ok(Value::String(string.to_string())));
}

fn convert_float_to_json_value(num: f32) {
    assert_eq!(AMQPValue::Float(num).to_json_value(), Ok(Value::Number(Number::from_f64(num.into()).unwrap())));
}

fn convert_double_to_json_value(num: f64) {
    assert_eq!(AMQPValue::Double(num).to_json_value(), Ok(Value::Number(Number::from_f64(num).unwrap())));
}

#[test]
fn bool_to_json_value() {
    for val in [true, false] {
        convert_bool_to_json_value(val);
    }
}

#[test]
fn short_string_to_json_value() {
    for val in ["a", "hello", "hello, world"] {
        convert_short_string_to_json_value(val);
    }
}

#[test]
fn long_string_to_json_value() {
    for val in ["a", "hello", "hello, world"] {
        convert_long_string_to_json_value(val);
    }
}

#[test]
fn float_to_json_value() {
    for val in [0f32, 0.1f32, 15f32, 145432f32] {
        convert_float_to_json_value(val);
    }
}

#[test]
fn double_to_json_value() {
    for val in [0f64, 0.1f64, 15f64, 145464f64] {
        convert_double_to_json_value(val);
    }
}
