use crate::{AsBytes, AsValue, Bytes};

pub fn type_id(k: impl AsBytes) -> usize {
    let mut buff: [u8; 8] = [0u8; 8];
    buff.copy_from_slice(&k.as_bytes()[..8]);
    usize::from_le_bytes(buff)
}

pub fn values_to_bytes<T: AsValue + Clone>(v: Vec<T>) -> Vec<Bytes> {
    v.iter()
        .map(|v| v.clone().as_value())
        .collect::<Vec<Bytes>>()
}
