use core::option;

/**
 * Implementation of thunks in Rust.
 */
pub struct Lazy<T: Copy> {
  code          : @fn() -> T,
  mut value     : Option<T>
}

/**
 * Unfortunately requires a caller to make a closure.
 */
pure fn Lazy<T: Copy>(closure: @fn() -> T) -> Lazy<T> {
  let l : Lazy<T> = Lazy {
    code      : closure,
    value     : None
  };

  return l;
}

impl<T: Copy> Lazy<T> {
  fn force() -> T {
    match self.value {
      Some(value) => { return value; }
      None => {
        let result = (self.code)();
        self.value = Some(result);

        return result;
      }
    };
  }
}


use rand::*;

#[test]
fn test_thunk() {

  let thunk = Lazy (|| {
    random()
  });

  let a1 = thunk.force();
  let a2 = thunk.force();

  assert(a1 == a2);
}
