use std::list;
use std::list::{List, Cons, Nil};

/**
 * A Huet Zipper built around @Lists.
 */
pub struct ListZipper<E: Copy> {
  hd: @List<E>,
  tail: @List<E>
}

trait Zipper<E: Copy> {
  pure fn go_left() -> self;
  pure fn go_right() -> self;
  pure fn at_begin() -> bool;
  pure fn at_end() -> bool;
  pure fn is_empty() -> bool;
  pure fn to_begin() -> self;
  pure fn cursor() -> Option<E>;
  pure fn insert(e: E) -> self;
  pure fn delete(e: E) -> self;
  pure fn replace(e: E) -> self;
  pure fn to_list() -> @List<E>;
  fn to_vec() -> ~[E];
}

pure fn ListZipper<E: Copy>() -> ListZipper<E> {
  ListZipper { hd:@Nil, tail:@Nil }
}

pure fn ListZipper_with_item<E: Copy>(item: E) -> ListZipper<E> {
  ListZipper { hd:@Cons(item, @Nil), tail:@Nil }
}

fn ListZipper_from_vec<E: Copy>(items: ~[E]) -> ListZipper<E> {
  ListZipper { hd: @Nil, tail: list::from_vec(items) }
}

// this needs to be given back to the list module
pure fn iter<T>(l: @List<T>, f: fn(&T)) {
    let mut cur = l;
    loop {
        cur = match *cur {
          Cons(ref hd, tl) => {
            f(hd);
            tl
          }
          Nil => break
        }
    }
}

// this needs to be given back to the list module
pure fn foldl<T: Copy, U>(z: T, ls: @List<U>, f: fn(&T, &U) -> T) -> T {
    let mut accum: T = z;
    do iter(ls) |elt| { accum = f(&accum, elt);}
    accum
}

// this needs to be given back to the list module
pure fn rev<E: Copy>(xs: @List<E>) -> @List<E> {
  pure fn f<E: Copy>(ys: &@List<E>, y: &E) -> @List<E> { @Cons(*y, *ys) }
  foldl(@Nil, xs, f)
}

impl<E: Copy> ListZipper<E> {
  pure fn go_left() -> ListZipper<E> {
    match self {
      ListZipper {
        hd: @Cons(a, xs),
        tail: ys } => ListZipper{hd:xs, tail:@Cons(a, ys)},
      _ => self
    }
  }

  pure fn go_right() -> ListZipper<E> {
    match self {
      ListZipper {
        hd: xs,
        tail: @Cons(a, ys) } => ListZipper { hd: @Cons(a, xs), tail: ys},
      _ => self
    }
  }

  pure fn at_begin() -> bool {
    match self {
      ListZipper { hd: @Nil, _} => true,
      _ => false
    }
  }

  pure fn at_end() -> bool {
    match self {
      ListZipper { hd: _, tail: @Nil } => true,
      _ => false
    }
  }

  pure fn is_empty() -> bool {
    match self {
      ListZipper { hd: @Nil, tail: @Nil } => true,
      _ => false
    }
  }

  pure fn to_begin() -> ListZipper<E> {
    match self {
      ListZipper {hd:xs, tail:ys } => ListZipper { hd:@Nil,  tail:list::append(rev(xs), ys) }
    }
  }

  pure fn cursor() -> Option<E> {
    match self {
      ListZipper { hd:@Nil, _ } => None,
      ListZipper { hd:@Cons(e, _), _ } => Some(e),
    }
  }

  pure fn insert(e: E) -> ListZipper<E> {
    match self {
      ListZipper{hd:xs, tail:ys} => ListZipper{hd:xs, tail:@Cons(e, ys)}
    }
  }

  pure fn delete() -> ListZipper<E> {
    match self {
      ListZipper{hd:xs, tail:@Cons(_, ys)} => ListZipper{hd:xs, tail:ys},
      _ => self
    }
  }

  pure fn replace(e: E) -> ListZipper<E> {
    match self {
      ListZipper { hd:xs, tail:@Cons(_, ys) } => ListZipper { hd:xs, tail:@Cons(e, ys) },
      _ => self
    }
  }




  pure fn to_list() -> @List<E> {
    match self {
      ListZipper { hd:xs, tail:ys } => list::append(rev(xs), ys)
    }
  }

  fn to_vec() -> ~[E] {
    let mut results : ~[E] = ~[];

    match self {
      ListZipper { hd:xs, tail:ys } => {
        do list::iter(rev(xs)) |x| {
          results.push(*x)
        }

        do list::iter(ys) |y| {
          results.push(*y)
        }
      }
    }

    return results;
  }

}

#[test]
fn test_empty() {
  let zipper : ListZipper<()> = ListZipper();
  assert(zipper.at_begin());
  assert(zipper.at_end());
  assert(zipper.is_empty());
}

#[test]
fn test_with_one_item() {
  let zipper = ListZipper_with_item(1);
  assert(!zipper.at_begin());
  assert(zipper.at_end());
  assert(!zipper.is_empty());

  let v2 = zipper.go_left();
  assert(v2.at_begin());
  assert(!v2.at_end());

  let v3 = v2.go_right();
  assert(!v3.at_begin());
  assert(v3.at_end());
}

#[test]
fn test_multiple_items() {
  let v1 = ListZipper_with_item(1);
  let v2 = v1.insert(2);
  let v3 = v2.insert(3);
  let v4 = v3.insert(4);

  assert(v4.cursor() == Some(1));
}

#[test]
fn test_reverse() {
  let a = @Cons(1, @Cons(2, @Cons(3, @Cons(4, @Nil))));
  let b = rev(a);

  assert(list::head(b) == 4);
  assert(b == @Cons(4, @Cons(3, @Cons(2, @Cons(1, @Nil)))));
}

#[test]
fn test_from_vec() {
  let orig_vec = ~[1, 2, 3, 4];
  let mut zipper = ListZipper_from_vec(orig_vec);
  assert(zipper.cursor() == None);

  zipper = zipper.go_right();
  assert(zipper.cursor() == Some(1));
  zipper = zipper.go_right();
  assert(zipper.cursor() == Some(2));
  zipper = zipper.go_right();
  assert(zipper.cursor() == Some(3));
  zipper = zipper.go_right();
  assert(zipper.cursor() == Some(4));
  zipper = zipper.go_right();
  assert(zipper.at_end());

  let new_vec = zipper.to_vec();
  assert(new_vec == orig_vec);


  zipper = zipper.go_left();
  zipper = zipper.delete();
  assert(zipper.to_vec() == ~[1, 2, 3]);

  zipper = zipper.go_left();
  zipper = zipper.replace(5);
  assert(zipper.to_vec() == ~[1, 2, 5]);
}

