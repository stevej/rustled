const SK5  : u32 = 0x55555555;
const SK3  : u32 = 0x33333333;
const SKF0 : u32 = 0xF0F0F0F;
const SKFF : u32 = 0xFF00FF;

/**
 * Count the Population of an unsigned integer. Returns how many bits are
 * set to 1.
 *
 * Currently designed for a 32-bit unsigned integer.
 *
 * Many chips have a POPCNT instruction available and would be an order
 * of magnitude speed improvement.
 */
pure fn ctpop32(map : u32) -> u32 {
  let mut Map = copy map;

  Map-=((Map>>1)&SK5);
  Map=(Map&SK3)+((Map>>2)&SK3);
  Map=(Map&SKF0)+((Map>>4)&SKF0);
  Map+=Map>>8;

  return (Map+(Map>>16))&0x3F;
}

#[test]
fn test_ctpop32() {
  assert(ctpop32(0x0) == 0);
  assert(ctpop32(0x1) == 1);
  assert(ctpop32(0x3) == 2);
  assert(ctpop32(0x7) == 3);
  assert(ctpop32(0xF) == 4);
  assert(ctpop32(0x10) == 1);
  assert(ctpop32(0xFF) == 8);
  assert(ctpop32(0xAD) == 5);
  assert(ctpop32(0x2FADBFF) == 21);
  assert(ctpop32(0x82FFFFFF) == 26);
  assert(ctpop32(0x8FFFFFFF) == 29);
  assert(ctpop32(0xFFFFFFFF) == 32);
}