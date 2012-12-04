use core::cmp::{Eq, Ord};
use core::option;

/**
 * A purely functional binary search tree.
 *
 * Cost for insert is ____
 * Cost for update is ____
 * Cost for contains is ___
 * Cost for delete is ___
 *
 * K is the key the object is stored under.
 * V is the type of objects stored.
 */

// it would be nice to keep all these type bounds in one place.
type Branch<K: Copy Eq Ord, V: Copy> = Option<Tree<K, V>>;

pub enum Tree<K: Copy Eq Ord, V: Copy> {
  Tree_(@{
    key       : K,
    value     : V,
    left  : Branch<K, V>,
    right : Branch<K, V>
  })
}

// why would the type bounds on this K ever be different than on the enum?
pure fn Tree<K: Copy Eq Ord, V: Copy>(initial_key: K, initial_value: V) -> Tree<K, V> {
  // what does underscore here mean?
  let root = Tree_(@{
    // why do we use colons here?
    key       : initial_key,
    value     : initial_value,
    left  : None,
    right : None
  });

  return root;
}

// why do we need to declare the type parameter constraints here?
impl<K: Copy Eq Ord, V: Copy> Tree<K, V> {
  fn get(search_key: K) -> Option<V> {
    if self.key.eq(&search_key) {
      return Some(self.value);
    } else if self.key.gt(&search_key) {
      // this is identical to right/lt so room for improvement.
      match self.left {
        // what are the performance implications here?
        Some(tree) => { return tree.get(search_key); }
        // does rust have flatMap?
        None => { return None; }
      }
    } else if self.key.lt(&search_key) {
      match self.right {
        Some(tree) => { return tree.get(search_key); }
        None => { return None; }
      }
    } else {
      return None;
    }
  }

  fn insert(new_key: K, new_value: V) -> Tree<K, V> {
    let branch = Tree(new_key, new_value);
    if self.key.lt(&new_key) {
      match self.left {
        Some(branch) => {
          return Tree_(@{
            key : self.key,
            value : self.value,
            left : Some(branch.insert(new_key, new_value)),
            right : self.right
          })
        }
        None => {
          return Tree_(@{
            key : self.key,
            value : self.value,
            left : Some(Tree(new_key, new_value)),
            right : self.right
          })
        }
      }
    } else if self.key.gt(&new_key) {
      match self.right {
        Some(branch) => {
          return Tree_(@{
            key : self.key,
            value : self.value,
            left : self.left,
            right : Some(branch.insert(new_key, new_value))
          })
        }
        None => {
          return Tree_(@{
            key : self.key,
            value : self.value,
            left : self.left,
            right : Some(Tree(new_key, new_value))
          })
        }
      }
    } else if self.key.eq(&new_key) {
      return Tree_(@{
        key   : new_key,
        value : new_value,
        left  : self.left,
        right : self.right
      });
    }
    return branch;
  }

  fn contains(maybe_key: K) -> bool {
    if self.key.eq(&maybe_key) {
      return true;
    } else if self.key.gt(&maybe_key) {
      match self.right {
        Some(branch) => { return branch.contains(maybe_key); }
        None => { return false; }
      }
    } else if self.key.lt(&maybe_key) {
      match self.left {
        Some(branch) => { return branch.contains(maybe_key); }
        None => { return false; }
      }
    } else {
      io::println("sorry, your key was not found in the tree");
      return false;
    }
  }
}

