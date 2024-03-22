use serde_json::Value;

pub trait Contexts {
    fn contexts() -> Value;
}
