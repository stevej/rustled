use core::cmp::{Eq, Ord};


pub trait PersistentMap<K: Copy Eq Ord, V: Copy> {
  pure fn get(k: K) -> @Option<V>;
  pure fn put(k: K, v: V) -> self;
  pure fn delete(k: K) -> self;
  pure fn traverse(f: fn((&K), (@Option<V>)));
}

enum RBColor {
  Red,
  Black
}

pub enum RBMap<K: Copy Eq Ord, V: Copy> {
  //   color    left         k  value       right
  Tree(RBColor, @RBMap<K,V>, K, @Option<V>, @RBMap<K,V>),
  Leaf
}

pure fn RBMap<K: Copy Eq Ord, V: Copy>(key: K, value: V) -> @RBMap<K, V> {
  @Tree(Red, @Leaf, key, @Some(value), @Leaf)
}

/**
 * A purely functional Red-Black tree.
 *
 * A port of Matt Might's Scala port of Okasaki's RB Trees: http://matt.might.net/articles/implementation-of-immutable-purely-functional-okasaki-red-black-tree-maps-in-scala/RBMap.scala
 * which was itself a port of http://www.ccs.neu.edu/course/cs3500wc/jfp99redblack.pdf
 */
impl<K: Copy Eq Ord, V: Copy> @RBMap<K, V> : PersistentMap<K, V> {
  pure fn get(k: K) -> @Option<V> {
    match self {
      @Leaf => @None,
      @Tree(_, left, key, maybe_value, right) => {
        if (k.lt(&key)) {
          left.get(k)
        } else if (k.gt(&key)) {
          right.get(k)
        } else {
          maybe_value
        }
      }
    }
  }

  /// Visit all pairs in the map in order.
  pure fn traverse(f: fn((&K), (@Option<V>))) {
    match *self {
      Leaf => (),
      Tree(_, left, key, maybe_value, right) => {
        left.traverse(f);
        f(&key, maybe_value);
        right.traverse(f);
      }
    }
  }

  pure fn put(k : K, v : V) -> @RBMap<K, V> {
    self.modifiedWith(k, @Some(v))
  }

  pure fn delete(k: K) -> @RBMap<K, V> {
    self.modifiedWith(k, @None)
  }

  priv pure fn blacken(n : @RBMap<K, V>) -> @RBMap<K, V> {
    match n {
      @Leaf => n,
      @Tree(_,l,k,v,r) => @Tree(Black,l,k,v,r)
    }
  }

  priv pure fn modifiedWith (k : K, new_value: @Option<V>) -> @RBMap<K, V> {
    self.blacken(self.modWith(k, new_value))
  }

  priv pure fn balance (c : RBColor, l : @RBMap<K,V>, k : K, v : @Option<V>, r : @RBMap<K,V>) -> @RBMap<K,V> {
    match (c,l,k,v,r) {
      (Black,@Tree(Red,@Tree(Red,a,xK,xV,b),yK,yV,c),zK,zV,d) =>
        @Tree(Red,@Tree(Black,a,xK,xV,b),yK,yV,@Tree(Black,c,zK,zV,d)),
      (Black,@Tree(Red,a,xK,xV,@Tree(Red,b,yK,yV,c)),zK,zV,d) =>
        @Tree(Red,@Tree(Black,a,xK,xV,b),yK,yV,@Tree(Black,c,zK,zV,d)),
      (Black,a,xK,xV,@Tree(Red,@Tree(Red,b,yK,yV,c),zK,zV,d)) =>
        @Tree(Red,@Tree(Black,a,xK,xV,b),yK,yV,@Tree(Black,c,zK,zV,d)),
      (Black,a,xK,xV,@Tree(Red,b,yK,yV,@Tree(Red,c,zK,zV,d))) =>
        @Tree(Red,@Tree(Black,a,xK,xV,b),yK,yV,@Tree(Black,c,zK,zV,d)),
      (c,a,xK,xV,b) =>
        @Tree(c,a,xK,xV,b)
    }
  }

  priv pure fn modWith (k : K, new_value: @Option<V>) -> @RBMap<K, V> {
    match self {
      @Leaf => @Tree(Red, self, k, new_value, self),
      @Tree(color, left, key, original_value, right) => {
        if (k.lt(&key)) {
          self.balance(color, left.modWith(k, new_value), key, original_value, right)
        } else if (k == key) {
          @Tree(color, left, k, new_value, right)
        } else {
          self.balance(color, left, key, original_value, right.modWith(k, new_value))
        }
      }
    }
  }
}

#[test]
fn test_rb_tree_create() {
  let v1 = RBMap("stevej", 150);
  let v2 = v1.put("thatstacy", 167);

  assert(v2.get("stevej") == @Some(150));
  assert(v2.get("thatstacy") == @Some(167));

  let v3 = v2.delete("stevej");
  assert(v3.get("stevej") == @None);
  assert(v3.get("thatstacy") == @Some(167));
}

#[test]
fn test_traverse() {
  let v1 = RBMap(1, 0);
  let v2 = v1.put(4, 0);
  let v3 = v2.put(3, 0);
  let v4 = v3.put(5, 0);
  let v5 = v4.put(2, 0);

  let n = @mut 1;

  fn t(n: @mut int, k: int, _v: Option<int>) {
    assert (*n == k); *n += 1;
  }

  v5.traverse(|x,y| t(n, *x, *y));
}
