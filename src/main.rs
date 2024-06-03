use std::fmt::Debug;

#[derive(Debug, Default, Clone)]
pub struct Entry<Key, Value> {
  occupied: bool,
  key: Key,
  value: Value,
}

#[derive(Debug)]
pub struct HashMap<K, V> {
  entries: Vec<Entry<K, V>>,
  occupied: usize,
}

pub trait Hashable {
  fn hash(&self) -> usize;
}

impl Hashable for String {
  fn hash(&self) -> usize {
    // djb2
    let mut hash: usize = 5381;
    for byte in self.bytes() {
      hash = (hash << 5).wrapping_add(hash) + byte as usize;
    }
    hash
  }
}

impl Hashable for &str {
  fn hash(&self) -> usize {
    // djb2
    let mut hash: usize = 5381;
    for byte in self.bytes() {
      hash = (hash << 5).wrapping_add(hash) + byte as usize;
    }
    hash
  }
}

impl Hashable for char {
  fn hash(&self) -> usize {
    *self as usize
  }
}

impl Hashable for i8 {
  fn hash(&self) -> usize {
    *self as usize
  }
}

impl Hashable for u8 {
  fn hash(&self) -> usize {
    *self as usize
  }
}

impl Hashable for i16 {
  fn hash(&self) -> usize {
    *self as usize
  }
}

impl Hashable for u16 {
  fn hash(&self) -> usize {
    *self as usize
  }
}

impl Hashable for i32 {
  fn hash(&self) -> usize {
    *self as usize
  }
}

impl Hashable for u32 {
  fn hash(&self) -> usize {
    *self as usize
  }
}

impl Hashable for i64 {
  fn hash(&self) -> usize {
    self.wrapping_abs() as usize
  }
}

impl Hashable for u64 {
  fn hash(&self) -> usize {
    *self as usize
  }
}

impl Hashable for i128 {
  fn hash(&self) -> usize {
    *self as usize
  }
}

impl Hashable for u128 {
  fn hash(&self) -> usize {
    *self as usize
  }
}

impl Hashable for isize {
  fn hash(&self) -> usize {
    *self as usize
  }
}

impl Hashable for usize {
  fn hash(&self) -> usize {
    *self
  }
}

impl Hashable for f32 {
  fn hash(&self) -> usize {
    self.to_bits() as usize
  }
}

impl Hashable for f64 {
  fn hash(&self) -> usize {
    self.to_bits() as usize
  }
}

impl Hashable for bool {
  fn hash(&self) -> usize {
    if *self {
      1
    } else {
      0
    }
  }
}

pub struct HashMapIterator<'a, Key, Value> {
  entries: &'a Vec<Entry<Key, Value>>,
  current_index: usize,
}

impl<'a, Key, Value> Iterator for HashMapIterator<'a, Key, Value> {
  type Item = (&'a Key, &'a Value);

  fn next(&mut self) -> Option<Self::Item> {
      while self.current_index < self.entries.len() {
          let entry = &self.entries[self.current_index];
          self.current_index += 1;
          if entry.occupied {
              return Some((&entry.key, &entry.value));
          }
      }
      None
  }
}

impl<Key, Value> Default for HashMap<Key, Value> where Key: Clone + Default + Hashable + Debug + PartialEq, Value: Clone + Default + Debug {
  fn default() -> Self {
    Self::new()
  }
}

#[allow(dead_code)]
impl<Key, Value> HashMap<Key, Value> where Key: Clone + Default + Hashable + Debug + PartialEq, Value: Clone + Default + Debug {
  pub fn new() -> Self {
    const INITIAL_CAPACITY: usize = 64;
    Self {
      entries: vec![Entry::<_, _>::default(); INITIAL_CAPACITY],
      occupied: 0,
    }
  }

  pub fn empty() -> Self {
    Self {
      entries: vec![],
      occupied: 0,
    }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      entries: vec![Entry::<_, _>::default(); capacity],
      occupied: 0,
    }
  }

  pub fn capacity(&self) -> usize {
    self.entries.capacity()
  }

  pub fn len(&self) -> usize {
    self.occupied
  }

  pub fn is_empty(&self) -> bool {
    self.occupied == 0
  }

  pub fn get_index(&self, key: &Key) -> Option<usize> {
    let mut index = key.hash() % self.entries.len();
    for _ in 0..self.entries.len() {
      if !self.entries[index].occupied {
        break;
      }
      if self.entries[index].key == *key {
        break;
      }
      index = (index + 1) % self.entries.len();
    }
    if self.entries[index].occupied && self.entries[index].key == *key {
      Some(index)
    } else {
      None
    }
  }

  pub fn insert(&mut self, key: Key, new_value: Value) {
    if let Some(old) = self.get_mut(&key) {
      *old = new_value;
    } else {
      if self.occupied >= self.entries.len() / 2 {
        self.extend();
      }
      let mut index = key.hash() % self.entries.len();
      for _ in 0..self.entries.len() {
        if !self.entries[index].occupied {
          self.entries[index].occupied = true;
          self.entries[index].key = key;
          self.entries[index].value = new_value;
          self.occupied += 1;
          break;
        }
        index = (index + 1) % self.entries.len();
      }
    }
  }

  pub fn get(&self, key: &Key) -> Option<&Value> {
    if let Some(index) = self.get_index(key) {
      Some(&self.entries[index].value)
    } else {
      None
    }
  }

  pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
    if let Some(index) = self.get_index(key) {
      Some(&mut self.entries[index].value)
    } else {
      None
    }
  }

  fn extend(&mut self) {
    let mut new_self = Self::with_capacity(self.entries.len() * 2 + 1);
    for entry in self.entries.iter() {
      if entry.occupied {
        new_self.insert(entry.key.clone(), entry.value.clone());
      }
    }
    *self = new_self;
  }

  pub fn iter(&self) -> HashMapIterator<Key, Value> {
    HashMapIterator {
      entries: &self.entries,
      current_index: 0,
    }
  }

  #[cfg(debug_assertions)]
  fn debug_dump(&self) {
    for (index, entry) in self.entries.iter().enumerate() {
      if entry.occupied {
        println!("{}: {:?}", index, entry);
      } else {
        println!("{}: Empty", index)
      }
    }
  }
}

fn main() {
  let mut days = HashMap::<u8, &str>::with_capacity(7);
  days.insert(1, "Monday");
  days.insert(2, "Tuesday");
  days.insert(3, "Wednesday");
  days.insert(4, "Thursday");
  days.insert(5, "Friday");
  days.insert(6, "Saturday");
  days.insert(7, "Sunday");
  days.debug_dump();
  println!("{:?}", days.get(&1));
  println!("{:?}", days.len());
  for (key, value) in days.iter() {
    println!("{}: {}", key, value);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn performance() {
    const N: u64 = 1_000_000;

    let start_insert = std::time::Instant::now();
    let mut map = HashMap::<u64, u64>::new();
    for i in 0..N {
      map.insert(i, i);
    }
    println!("RustyMap Insert elapsed: {:?}", start_insert.elapsed());

    let start_get = std::time::Instant::now();
    for i in 0..N {
      assert_eq!(map.get(&i), Some(&i));
    }
    println!("RustyMap Get elapsed: {:?}", start_get.elapsed());

    let start_insert = std::time::Instant::now();
    let mut std_map = std::collections::HashMap::<u64, u64>::new();
    for i in 0..N {
      std_map.insert(i, i);
    }
    println!("std Insert elapsed: {:?}", start_insert.elapsed());
    let start_get = std::time::Instant::now();
    for i in 0..N {
      assert_eq!(std_map.get(&i), Some(&i));
    }
    println!("std Get elapsed: {:?}", start_get.elapsed());
  }
}