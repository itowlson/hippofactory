use std::collections::HashMap;

pub struct BuildConditionOptions {
    values: HashMap<String, String>,
}

impl std::convert::From<HashMap<String, String>> for BuildConditionOptions {
    fn from(values: HashMap<String, String>) -> Self {
        Self { values }
    }
}

impl BuildConditionOptions {
    pub fn none() -> Self {
        Self { values: HashMap::new() }
    }
}

pub enum BuildConditionExpression {
    None,
    Equal(EqualityCondition),
    Unequal(InequalityCondition),
}

pub enum BuildConditionValue {
    OptionRef(String),
    Literal(String),
}

pub struct EqualityCondition {
    left: BuildConditionValue,
    right: BuildConditionValue,
}

pub struct InequalityCondition {
    left: BuildConditionValue,
    right: BuildConditionValue,
}
