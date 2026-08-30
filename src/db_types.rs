use std::{borrow::Cow, str::FromStr};

#[derive(Debug)]
pub enum DataType {
    Integer { value: i64 },
    Float { value: f64 },
    Text { value: String },
}

impl From<i64> for DataType {
    fn from(value: i64) -> Self {
        Self::Integer { value }
    }
}

impl From<f64> for DataType {
    fn from(value: f64) -> Self {
        Self::Float { value }
    }
}

impl From<String> for DataType {
    fn from(value: String) -> Self {
        Self::Text { value }
    }
}

impl From<&str> for DataType {
    fn from(value: &str) -> Self {
        Self::Text {
            value: value.to_owned(),
        }
    }
}

impl FromStr for DataType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<i64>() {
            Ok(Self::Integer { value })
        } else if let Ok(value) = s.parse::<f64>() {
            Ok(Self::Float { value })
        } else {
            Ok(Self::Text {
                value: s.to_owned(),
            })
        }
    }
}

impl DataType {
    pub fn to_str(&self) -> Cow<'_, str> {
        match self {
            Self::Integer { value } => Cow::Owned(value.to_string()),
            Self::Float { value } => Cow::Owned(value.to_string()),
            Self::Text { value } => Cow::Borrowed(value.as_str()),
        }
    }
}
