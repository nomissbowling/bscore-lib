#![doc(html_root_url = "https://docs.rs/bscore-lib/1.0.2")]
//! bscore-lib bowling score library for C (written in Rust)
//!

// #![crate_type = "cdylib"]

use std::slice;

use bscore::bgame::getscore;

/// bscore_s
/// - src: char *
/// - len: size_t of src string length
/// - mode: bool
/// - dst: char *
/// - size: size_t of dst buffer length
#[no_mangle]
pub extern "C" fn bscore_s(src: *const u8, len: usize, mode: bool,
  dst: *mut u8, size: *mut usize) -> i32 {
  let sz = unsafe { slice::from_raw_parts_mut(size, 1) };
  if dst != 0 as *mut u8 {
    let d = unsafe { slice::from_raw_parts_mut(dst, sz[0]) };
    if sz[0] > 0 { d[0] = 0u8; }
  }
  let s = unsafe { slice::from_raw_parts(src, len).to_vec() };
  let s = String::from_utf8(s).expect("src should be utf8");
  let scores = getscore(s.as_str(), mode).expect("score sequence");
  let o = scores.iter().map(|score|
    format!("{}{}\x0A", score, score.p)).collect::<Vec<_>>().join("");
  if dst == 0 as *mut u8 {
    sz[0] = o.len() + 1;
    return 0;
  }
  if o.len() > sz[0] - 1 { return -1; }
  if dst != 0 as *mut u8 {
    let d = unsafe { slice::from_raw_parts_mut(dst, sz[0]) };
    for (i, c) in o.chars().enumerate() { d[i] = c as u8; }
    d[o.len()] = 0u8;
  }
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
