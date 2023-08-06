#![deny(unused_crate_dependencies)]

/// Utilities used in anomaly detection.
use nom::AsBytes;

pub fn type_id(k: impl AsBytes) -> usize {
    let mut buff: [u8; 8] = [0u8; 8];
    let pad = "00000000".as_bytes();

    let k = k.as_bytes();

    if k.len() < 8 {
        let b = &[k, pad].concat();
        buff.copy_from_slice(&b[..8]);
        return usize::from_le_bytes(buff);
    }

    buff.copy_from_slice(&k[..8]);
    usize::from_le_bytes(buff)
}
