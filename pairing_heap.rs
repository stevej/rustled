use core::cmp::{Eq, Ord};
use std::list;
use std::list::{List, Cons, Nil};

/**
 * A purely functional Pairing Heap [FSST86]
 *
 * Our implementation uses Linked List (cons cells) so may not be the
 * fastest way to implement this in Rust.
 *
 * This implementation is a port of the Standard ML found in Okasaki's
 * Purely Functional Data Structures.
 */
pub enum PairingHeap<E: Copy Eq Ord> {
  Empty_,
  PairingHeapCell(
    E,
    @List<PairingHeap<E>>
  )
}

pub trait Heap<E: Copy Eq Ord> {
  // returns true if the Heap is empty.
  pure fn is_empty(&self) -> bool;

  // returns a new Heap with the element inserted.
  pure fn insert(elem: E) -> self;

  // returns the minimum element without a modified heap
  pure fn find_min() -> Option<E>;

  // returns the minimum element and a new Heap without that element.
  pure fn delete_min() -> (Option<E>, self);
}


pure fn Empty<E: Copy Eq Ord>() -> PairingHeap<E> {
  Empty_
}

pure fn PairingHeap<E: Copy Eq Ord>(initial_value: E) -> PairingHeap<E> {
  PairingHeapCell(
    initial_value,
    @Nil
  )
}

impl<E: Copy Eq Ord> PairingHeap<E> : Eq {
  pure fn eq(other: &PairingHeap<E>) -> bool {
    match (self, *other) {
      (Empty_, Empty_) => {true}
      (PairingHeapCell(headA, restA), PairingHeapCell(headB, restB)) => {
        (headA == headB) && (restA == restB)
      }
      //(Empty_, PairingHeapCell(_)) => {false} // why are these unreachable?
      //(PairingHeapCell(_), Empty) => {false}
      (_, _) => {false}
    }
  }

  pure fn ne(other: &PairingHeap<E>) -> bool { !(self).eq(other) }
}

impl<E: Copy Eq Ord> PairingHeap<E> : Heap<E> {
  pure fn is_empty(&self) -> bool {
    *self == Empty_
  }

  pure fn insert(e: E) -> PairingHeap<E> {
    self.merge(PairingHeap(e))
  }

  pure fn find_min() -> Option<E> {
    match self {
      Empty_ => { None }
      PairingHeapCell(head, _) => { Some(head) }
    }
  }

  pure fn delete_min() -> (Option<E>, PairingHeap<E>) {
    match self {
      Empty_ => {(None, self)}
      PairingHeapCell(head, rest) => {(Some(head), self.merge_pairs(rest))}
    }
  }

  pure fn merge(other: PairingHeap<E>) -> PairingHeap<E> {
    match (self, other) {
      (Empty_, b) => { b }
      (a, Empty_) => { a }
      (x@PairingHeapCell(headA, restA), y@PairingHeapCell(headB, restB)) => {
        if (headA.le(&headB)) {
          PairingHeapCell(
            headA,
            @Cons(y, restA)
          )
        } else {
          PairingHeapCell(
            headB,
            @Cons(x, restB)
          )
        }
      }
    }
  }

  pure fn merge_pairs(heaps: @List<PairingHeap<E>>) -> PairingHeap<E> {
    match heaps {
      // Why are @-signs required for pattern matching here?
      @Cons(a, @Cons(b, xs)) => {a.merge(b).merge(self.merge_pairs(xs))}
      @Cons(elem, @Nil) => {elem}
      @Nil => {Empty()}
    }
  }
}


#[test]
fn test_heap_create() {
  let heap = PairingHeap(1);
  assert(!heap.is_empty());

  // inference fails on this without a type declaration.
  let heap : PairingHeap<()> = Empty();
  assert(heap.is_empty());
}

#[test]
fn test_heap_insert() {
  let v1 = PairingHeap(10);
  let v2 = v1.insert(1);
  let (one, v3) = v2.delete_min();
  let (ten, v4) = v3.delete_min();
  let (e, v5) = v4.delete_min();

  assert(one == Some(1));
  assert(ten == Some(10));
  assert(v4 == Empty_);
  assert(e == None);
  assert(v5 == Empty_);
}

#[test]
fn test_heap_insert_delete_interleved() {
  let v1 = PairingHeap(10);
  let (a, v2) = v1.delete_min();
  assert(a == Some(10));
  assert(v2 == Empty_);

  let v3 = v2.insert(9);
  let v4 = v3.insert(8);
  let v5 = v4.insert(11);

  let (b, v6) = v5.delete_min();
  assert(b == Some(8));

  let v7 = v6.insert(7);
  let v8 = v7.insert(12);

  let x = v8.find_min();
  assert(x == Some(7));

  let (c, v9) = v8.delete_min();
  assert(c == Some(7));

  let (d, v10) = v9.delete_min();
  assert(d == Some(9));

  let (e, v11) = v10.delete_min();
  assert(e == Some(11));

  let (f, v12) = v11.delete_min();
  assert(f == Some(12));

  let (g, v13) = v12.delete_min();
  assert(g == None);
  assert(v13 == Empty_);
}

#[test]
fn test_immutable_heap() {
  let heap = PairingHeap(10);
  let x1 = heap.insert(1);

  assert(x1.find_min() == Some(1));
  assert(heap.find_min() == Some(10));

  let (a, v1) = heap.delete_min();
  assert(a == Some(10));
  assert(v1 == Empty_);

  let (b, x2) = x1.delete_min();
  assert(b == Some(1));
  assert(x2 != Empty_);
}

