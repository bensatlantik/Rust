// src/lib.rs

/// Converts bytes to kilobytes (KB).
///
/// # Example
///
/// ```
/// use byte_converter::bytes_to_kb;
///
/// let kb = bytes_to_kb(2048);
/// assert_eq!(kb, 2.0);
/// ```
pub fn bytes_to_kb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0
}

/// Converts bytes to megabytes (MB).
///
/// # Example
///
/// ```
/// use byte_converter::bytes_to_mb;
///
/// let mb = bytes_to_mb(1_048_576);
/// assert_eq!(mb, 1.0);
/// ```
pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

/// Converts kilobytes (KB) to bytes.
///
/// # Example
///
/// ```
/// use byte_converter::kb_to_bytes;
///
/// let bytes = kb_to_bytes(2.0);
/// assert_eq!(bytes, 2048);
/// ```
pub fn kb_to_bytes(kilobytes: f64) -> u64 {
    (kilobytes * 1024.0) as u64
}

/// Converts megabytes (MB) to bytes.
///
/// # Example
///
/// ```
/// use byte_converter::mb_to_bytes;
///
/// let bytes = mb_to_bytes(1.0);
/// assert_eq!(bytes, 1_048_576);
/// ```
pub fn mb_to_bytes(megabytes: f64) -> u64 {
    (megabytes * 1024.0 * 1024.0) as u64
}

/// Converts kilobytes (KB) to megabytes (MB).
///
/// # Example
///
/// ```
/// use byte_converter::kb_to_mb;
///
/// let mb = kb_to_mb(2048.0);
/// assert_eq!(mb, 2.0);
/// ```
pub fn kb_to_mb(kilobytes: f64) -> f64 {
    kilobytes / 1024.0
}

/// Converts megabytes (MB) to kilobytes (KB).
///
/// # Example
///
/// ```
/// use byte_converter::mb_to_kb;
///
/// let kb = mb_to_kb(1.0);
/// assert_eq!(kb, 1024.0);
/// ```
pub fn mb_to_kb(megabytes: f64) -> f64 {
    megabytes * 1024.0
}
