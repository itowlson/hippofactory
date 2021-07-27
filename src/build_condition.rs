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

    pub fn lookup(&self, key: &str) -> Option<String> {
        self.values.get(key).map(|s| s.clone())
    }
}

pub enum BuildConditionExpression {
    None,
    Equal(EqualityCondition),
    Unequal(InequalityCondition),
}

impl BuildConditionExpression {
    fn should_expand(&self) -> bool {
        match self {
            Self::None => true,
            _ => todo!("a complicated condition, oh no"),
        }
    }
}

pub enum BuildConditionValue {
    OptionRef(String),
    Literal(String),
}

impl BuildConditionValue {
    fn eval(&self, context: &BuildConditionOptions) -> Option<String> {
        match self {
            Self::Literal(s) => Some(s.clone()),
            Self::OptionRef(k) => context.lookup(k),
        }
    }
}

pub struct EqualityCondition {
    left: BuildConditionValue,
    right: BuildConditionValue,
}

pub struct InequalityCondition {
    left: BuildConditionValue,
    right: BuildConditionValue,
}
