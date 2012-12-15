use ctpop::ctpop32;

/**
 * Counts the number of leading zero bits in an unsigned 32-bit integer.
 */
pure fn lzc32(m: u32) -> u32 {
  let mut x = copy m;
  x |= (x >> 1);
  x |= (x >> 2);
  x |= (x >> 4);
  x |= (x >> 8);
  x |= (x >> 16);

  return(32 - ctpop32(x));
}

#[test]
fn test_lzc32_1() {
  assert(lzc32(0x0) == 32);
  assert(lzc32(0x1) == 31);
  assert(lzc32(0x7F) == 25);
  assert(lzc32(0xFF) == 24);
  assert(lzc32(0x7FFF) == 17);
  assert(lzc32(0xFFFF) == 16);
  assert(lzc32(0x7FFFFF) == 9);
  assert(lzc32(0xFFFFFF) == 8);
  assert(lzc32(0x7FFFFFFF) == 1);
  assert(lzc32(0xFFFFFFFF) == 0);
}