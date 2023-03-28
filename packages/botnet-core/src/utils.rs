use crate::{AsValue, Bytes};

pub fn values_to_bytes<T: AsValue + Clone>(v: Vec<T>) -> Vec<Bytes> {
    v.iter()
        .map(|v| v.clone().as_value())
        .collect::<Vec<Bytes>>()
}
