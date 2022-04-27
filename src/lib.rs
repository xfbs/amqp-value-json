//! Module that publishes traits allowing to quickly convert
//! between an [AMQPValue](amq_protocol_types::AMQPValue) and JSON
//! [Value](serde_json::Value) (and vice versa).

use amq_protocol_types::{AMQPValue, FieldArray, FieldTable};
use anyhow::Result;
use serde_json::{Map, Number, Value};
use std::error::Error;
use std::fmt::Debug;
use thiserror::Error as ThisError;

#[cfg(test)]
mod tests;

/// Convert AMQPValue to a JSON Value.
pub trait ToJson {
    type Target;
    type Error: Error + Debug + Clone;
    fn to_json_value(&self) -> Result<Self::Target, Self::Error>;
}

/// Convert a JSON Value to an AMQPValue.
pub trait ToAmqp {
    type Target;
    type Error: Error + Debug + Clone;
    fn to_amqp_value(&self) -> Result<Self::Target, Self::Error>;
}

/// Error that can happen when converting an [AMQPValue](amq_protocol_types::AMQPValue) to a [JSON Value](serde_json::Value).
#[derive(Clone, ThisError, Debug, PartialEq)]
pub enum ToJsonError {
    #[error("Invalid float: {0:}")]
    InvalidFloat(f32),
    #[error("Invalid float: {0:}")]
    InvalidDouble(f64),
    #[error("Conversion not implemented")]
    Unimplemented,
}

impl ToJson for AMQPValue {
    type Target = Value;
    type Error = ToJsonError;
    fn to_json_value(&self) -> Result<Self::Target, Self::Error> {
        use AMQPValue::*;
        let result = match self {
            Boolean(val) => Value::Bool(*val),
            ShortShortInt(val) => Value::Number((*val).into()),
            ShortShortUInt(val) => Value::Number((*val).into()),
            ShortInt(val) => Value::Number((*val).into()),
            ShortUInt(val) => Value::Number((*val).into()),
            LongInt(val) => Value::Number((*val).into()),
            LongUInt(val) => Value::Number((*val).into()),
            LongLongInt(val) => Value::Number((*val).into()),
            Float(val) => Number::from_f64((*val).into())
                .map(|v| Value::Number(v))
                .ok_or(ToJsonError::InvalidFloat(*val))?,
            Double(val) => Number::from_f64(*val)
                .map(|v| Value::Number(v))
                .ok_or(ToJsonError::InvalidDouble(*val))?,
            DecimalValue(_val) => Err(ToJsonError::Unimplemented)?,
            LongString(val) => Value::String(val.to_string()),
            ShortString(val) => Value::String(val.to_string()),
            Timestamp(val) => Value::Number((*val).into()),
            FieldArray(val) => val
                .as_slice()
                .iter()
                .map(|val| val.to_json_value())
                .collect::<Result<Vec<_>, _>>()
                .map(|val| Value::Array(val))?,
            FieldTable(val) => val
                .inner()
                .iter()
                .map(|(key, value)| value.to_json_value().map(|v| (key.to_string(), v)))
                .collect::<Result<Map<String, Value>, _>>()
                .map(|res| Value::Object(res))?,
            ByteArray(val) => Value::Array(
                val.as_slice()
                    .iter()
                    .map(|val| Value::Number((*val).into()))
                    .collect::<Vec<_>>(),
            ),
            Void => Value::Null,
        };
        Ok(result)
    }
}

/// Error converting something to an [AMQPValue](amq_protocol_types::AMQPValue).
#[derive(Clone, ThisError, Debug)]
pub enum ToAmqpError {
    #[error("Error converting number to an AMQPValue")]
    NumberError,
}

impl ToAmqp for Value {
    type Target = AMQPValue;
    type Error = ToAmqpError;
    fn to_amqp_value(&self) -> Result<Self::Target, Self::Error> {
        use Value::*;
        let result = match self {
            Bool(val) => AMQPValue::Boolean(*val),
            Null => AMQPValue::Void,
            Number(val) => match (val.as_f64(), val.as_i64(), val.as_u64()) {
                (Some(double), _, _) => AMQPValue::Double(double),
                (_, Some(signed), _) => AMQPValue::LongLongInt(signed),
                (_, _, Some(unsigned)) => AMQPValue::Timestamp(unsigned),
                _ => Err(ToAmqpError::NumberError)?,
            },
            String(val) => AMQPValue::LongString(val.as_str().into()),
            Array(val) => AMQPValue::FieldArray(
                val.iter()
                    .map(|val| val.to_amqp_value())
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .fold(FieldArray::default(), |mut array, item| {
                        array.push(item);
                        array
                    }),
            ),
            Object(val) => AMQPValue::FieldTable(
                val.iter()
                    .map(|(key, val)| val.to_amqp_value().map(|val| (key, val)))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .fold(FieldTable::default(), |mut array, (key, val)| {
                        array.insert(key.as_str().into(), val);
                        array
                    }),
            ),
        };
        Ok(result)
    }
}
