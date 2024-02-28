#![doc(html_root_url = "https://docs.rs/bscore-lib/1.0.0")]
//! bscore-lib bowling score library for C (written in Rust)
//!

// #![crate_type = "cdylib"]

use std::slice;

use bscore::bgame::bscore;

/// bscore_s
/// - src: char *
/// - len: size_t of src string length
/// - mode: bool
/// - dst: char *
/// - size: size_t of dst buffer length
#[no_mangle]
pub extern "C" fn bscore_s(src: *const u8, len: usize, mode: bool,
  dst: *mut u8, size: usize) -> i32 {
  let s = unsafe { slice::from_raw_parts(src, len).to_vec() };
  let r = bscore(String::from_utf8(s).expect("u8").as_str(), mode).expect("e");
  let o = format!("{}", r[0]);
  let d = unsafe { slice::from_raw_parts_mut(dst, size) };
  if o.len() > size - 1 { if size > 0 { d[0] = 0u8; } return -1; }
  for (i, c) in o.chars().enumerate() { d[i] = c as u8; }
  d[o.len()] = 0u8;
  0
}

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  use bscore::bgame::bscore;

  /// test score
  #[test]
  fn test_score() {
    assert_eq!(bscore("xxxxxxxxxxxx", false).unwrap(), [300]);
  }
}
