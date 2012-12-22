use core::cmp::{Eq, Ord};
use iter::BaseIter;

pub trait PersistentMap<K: Copy Eq Ord, V: Copy> {
  pure fn get(k: K) -> Option<V>;
  pure fn put(k: K, v: V) -> self;
  pure fn delete(k: K) -> self;
}

enum RBColor {
  Red,
  Black
}

pub enum RBMap<K: Copy Eq Ord, V: Copy> {
  //   color    left         k  value       right
  Tree(RBColor, @RBMap<K,V>, K, Option<V>, @RBMap<K,V>),
  Leaf
}

pure fn RBMap<K: Copy Eq Ord, V: Copy>(key: K, value: V) -> @RBMap<K, V> {
  @Tree(Red, @Leaf, key, Some(value), @Leaf)
}

priv impl<K: Copy Eq Ord, V: Copy> @RBMap<K, V> {
  pure fn modify(k: K, new_value: Option<V>) -> @RBMap<K, V> {
    match self {
      @Leaf => @Tree(Red, self, k, copy new_value, self),
      @Tree(color, left, key, original_value, right) => {
        if (k.lt(&key)) {
          self.balanceLeft(color, left.modify(k, new_value), key, original_value, right)
        } else if (k == key) {
          @Tree(color, left, k, copy new_value, right)
        } else {
          self.balanceRight(color, left, key, original_value, right.modify(k, new_value))
        }
      }
    }
  }

  pure fn balanceLeft(c: RBColor, l: @RBMap<K,V>, k: K, v: Option<V>, r: @RBMap<K,V>) -> @RBMap<K,V> {
    match (c, l, k, v, r) {
      (Black,@Tree(Red,@Tree(Red,a,xK,xV,b),yK,yV,c),zK,zV,d) =>
        @Tree(Red,@Tree(Black,a,xK,xV,b),yK,yV,@Tree(Black,c,zK,zV,d)),
      (c,a,xK,xV,b) =>
        @Tree(c,a,xK,xV,b)
    }
  }

  pure fn balanceRight(c: RBColor, l: @RBMap<K,V>, k: K, v: Option<V>, r: @RBMap<K,V>) -> @RBMap<K,V> {
    match (c, l, k, v, r) {
      (Black, @Tree(Red, t1, k1, v1, t2), k0, v0, @Tree(Red, t3, k2, v2, t4)) =>
        @Tree(Red, @Tree(Black, t1, k1, v1, t2), k0, v0, @Tree(Black, t3, k2, v2, t4)),
      (c, l, k, v, @Tree(Red, t1, k1, v1, t2)) =>
        @Tree(c, @Tree(Red, l, k, v, t1), k1, v1, t2),
      (c,a,xK,xV,b) =>
        @Tree(c,a,xK,xV,b)
    }
  }
}


/**
 * A purely functional Left-Leaning Red-Black Tree.
 */
impl<K: Copy Eq Ord, V: Copy> @RBMap<K, V> : PersistentMap<K, V> {
  pure fn get(k: K) -> Option<V> {
    match self {
      @Leaf => None,
      @Tree(_, left, key, maybe_value, right) => {
        if (k.lt(&key)) {
          left.get(k)
        } else if (k.gt(&key)) {
          right.get(k)
        } else {
          copy maybe_value
        }
      }
    }
  }

  pure fn put(k: K, new_value: V) -> @RBMap<K, V> {
    self.modify(k, Some(new_value))
  }

  pure fn delete(k: K) -> @RBMap<K, V> {
    self.modify(k, None)
  }
}

impl<K: Copy Eq Ord, V: Copy> RBMap<K, V>: iter::BaseIter<(&K, &V)> {
  pure fn size_hint(&self) -> Option<uint> {
    None
  }

  pure fn each(&self, f: fn(&(&K, &V)) -> bool) {
    match *self {
      Leaf => (),
      Tree(_, left, key, maybe_value, right) => {
        left.each(f);
        match maybe_value {
          Some(value) => f(&(&key, &value)),
          None => false,
        };
        right.each(f);
      }
    }
  }
}


#[test]
fn test_rb_tree() {
  let v1 = RBMap("stevej", 150);
  let v2 = v1.put("thatstacy", 187);

  assert(v2.get("stevej") == Some(150));
  assert(v2.get("thatstacy") == Some(187));

  let v3 = v2.delete("stevej");
  assert(v3.get("stevej") == None);
  assert(v3.get("thatstacy") == Some(187));

  let v4 = v2.put("jeremy", 16);

  let v5 = v4.put("ev", 20);
  let v6 = v5.put("zhana", 51573);

  assert(v6.get("jeremy") == Some(16));
  assert(v6.get("stevej") == Some(150));
  assert(v6.get("thatstacy") == Some(187));
  assert(v6.get("ev") == Some(20));
  assert(v6.get("zhana") == Some(51573));
}

#[test]
fn test_base_iter_each() {
  let v1 = RBMap(1, 0);
  let v2 = v1.put(4, 0);
  let v3 = v2.put(3, 0);
  let v4 = v3.put(5, 0);
  let v5 = v4.put(2, 0);

  let n = @mut 1;

  fn t(n: @mut int, kv: &(&int, &int)) -> bool{
    match kv {
      &(k, _) => {
        assert (*n == *k);
        *n += 1;
      }
    }
    false
  }

  v5.each(|z| t(n, z));
}
