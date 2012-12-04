use tree::*;

#[test]
fn test_create_tree_with_single_element() {
  let tree = Tree("stevej", 150);
  assert(tree.contains("stevej"));
  assert(!tree.contains("not_stevej"));
  assert(tree.get("stevej") == Some(150));
}

#[test]
fn test_change_items() {
  let tree = Tree("stevej", 150);
  let new_tree = tree.insert("stevej", 151);

  assert(tree.contains("stevej"));
  assert(new_tree.contains("stevej"));

  // original tree is unchanged
  assert(tree.get("stevej") == Some(150));

  // new tree has new value
  assert(new_tree.get("stevej") == Some(151));
}

#[test]
fn test_get() {
  let tree = Tree("stevej", 150);
  assert(tree.get("stevek") == None);
}

// Add 3 names:
// 1) stevej
// 2) thatstacy
// 3) qwerty
#[test]
fn test_add_items_single_level() {
  let v1 = Tree("stevej", 150);
  let v2 = v1.insert("thatstacy", 167);
  assert(v1.contains("stevej"));

  assert(v2.contains("stevej"));
  assert(v2.contains("thatstacy"));

  let v3 = v2.insert("qwerty", 200);
  assert(v3.contains("stevej"));
  assert(v3.contains("thatstacy"));
  assert(v3.contains("qwerty"));
  assert(!v2.contains("qwerty"));
  assert(!v1.contains("qwerty"));
}

// Add 3 names for a first level, then 4 more names to fill
// the next level
// 1) stevej
// 2) thatstacy
// 3) qwerty
// and the next level
// a) qxerty
// b) querty
// c) tiatstacy
// d) txatstacy

#[test]
fn test_add_items_double_level() {
  let v1 = Tree("stevej", 150);
  let v2 = v1.insert("thatstacy", 167);
  assert(v1.contains("stevej"));

  assert(v2.contains("stevej"));
  assert(v2.contains("thatstacy"));

  let v3 = v2.insert("qwerty", 200);
  assert(v3.contains("stevej"));
  assert(v3.contains("thatstacy"));
  assert(v3.contains("qwerty"));
  assert(!v2.contains("qwerty"));
  assert(!v1.contains("qwerty"));

  let v4 = v3.insert("qxerty", 1);
  let v5 = v4.insert("querty", 2);
  let v6 = v5.insert("tiatstacy", 3);
  let v7 = v6.insert("txatstacy", 4);

  assert(v7.contains("txatstacy"));
  assert(v7.contains("tiatstacy"));
  assert(v7.contains("querty"));
  assert(v7.contains("qxerty"));
}


fn test_iterate_in_order() {

}