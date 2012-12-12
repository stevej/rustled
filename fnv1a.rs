/**
 * Pure rust implementations of 32-bit and 64-bit versions
 * of the FNV 1a hashing algorithms.
 */
const FNV_prime_64 : u64 = 1099511628211u64;
const FNV_offset_64 : u64 = 14695981039346656037u64;

pure fn fnv1a_64(data: ~[u8]) -> u64 {
  let mut hash = FNV_offset_64;

  for vec::each(data) |datum| {
    hash ^= (*datum as u64);
    hash *= FNV_prime_64;
  }

  return hash;
}

const FNV_prime_32 : u32 = 16777619u32;
const FNV_offset_32 : u32 = 2166136261u32;

pure fn fnv1a_32(data: ~[u8]) -> u32 {
  let mut hash = FNV_offset_32;

  for vec::each(data) |datum| {
    hash ^= (*datum as u32);
    hash *= FNV_prime_32;
  }

  return hash;
}

#[cfg(test)]
mod tests {
  fn f64(bytes: ~str) -> u64 {
    fnv1a_64(str::to_bytes(bytes))
  }

  #[test]
  fn test_fnv1a64() {
    assert(f64(~"foobar") == 0x85944171f73967e8);
    assert(f64(~"") == 0xcbf29ce484222325);
    assert(f64(~"f") == 0xaf63db4c8601ead9);
    assert(f64(~"chongo was here!\n") == 0x46810940eff5f915);
    assert(f64(~"\x54\x4e\x51\x40") == 0x4b7b10fa9fe83936);
    assert(f64(~"64.81.78.84") == 0xe73042c5d2ae266d);
  }

  fn f32(bytes: ~str) -> u32 {
    fnv1a_32(str::to_bytes(bytes))
  }

  #[test]
  fn test_fnv1a32() {
    assert(f32(~"foobar") == 0xbf9cf968);
    assert(f32(~"") == 0x811c9dc5);
    assert(f32(~"f") == 0xe30c2799);
    assert(f32(~"chongo was here!\n") == 0xd49930d5);
    assert(f32(~"\x54\x4e\x51\x40") == 0x772633d6);
    assert(f32(~"64.81.78.84") == 0xa55b89ed);
  }
}