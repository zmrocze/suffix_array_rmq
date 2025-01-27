// #[cfg(test)]
// use super::*;

use crate::create_sarmq;
use rand::Rng;

// #[test]
pub fn random_query_random_sequence( n : usize, key_range: Option<usize> ) {
  let key_range = key_range.unwrap_or(n);
  let mut rng = rand::thread_rng();
  let xs = {
    let mut xs: Vec<usize> = vec![];
    for _ in 0..n {
      xs.push(rng.gen_range(0..key_range))
    }
    xs
  };
  
  // naive
  let i = rng.gen_range(0..n);
  let j = rng.gen_range(i..n);
  let mut k = 0;
  loop {
    if i+k < n && j+k < n && xs[i+k] == xs[j+k] {
      k += 1;
    } else {
      break;
    }
  }

  let rmq = create_sarmq(&xs);
  assert_eq!(rmq.query(i, j), k);
  
}

#[test]
pub fn test_small_n_binary() {
  random_query_random_sequence(10, Some(2));
  random_query_random_sequence(1, Some(1));
  random_query_random_sequence(20, Some(2));
  random_query_random_sequence(50, Some(2));
  random_query_random_sequence(250, Some(2));
  random_query_random_sequence(10, Some(2));
  random_query_random_sequence(1, Some(1));
  random_query_random_sequence(20, Some(2));
  random_query_random_sequence(50, Some(2));
  random_query_random_sequence(250, Some(2));
  random_query_random_sequence(10, Some(2));
  random_query_random_sequence(1, Some(1));
  random_query_random_sequence(20, Some(2));
  random_query_random_sequence(50, Some(2));
  random_query_random_sequence(250, Some(2));
  random_query_random_sequence(10, Some(2));
  random_query_random_sequence(1, Some(1));
  random_query_random_sequence(20, Some(2));
  random_query_random_sequence(50, Some(2));
  random_query_random_sequence(250, Some(2));
}

#[test]
pub fn test_small_n() {
  random_query_random_sequence(10, None);
  random_query_random_sequence(1, None);
  random_query_random_sequence(20, None);
  random_query_random_sequence(50, None);
  random_query_random_sequence(250, None);
  random_query_random_sequence(10, None);
  random_query_random_sequence(1, None);
  random_query_random_sequence(20, None);
  random_query_random_sequence(50, None);
  random_query_random_sequence(250, None);
  random_query_random_sequence(10, None);
  random_query_random_sequence(1, None);
  random_query_random_sequence(20, None);
  random_query_random_sequence(50, None);
  random_query_random_sequence(250, None);
  random_query_random_sequence(10, None);
  random_query_random_sequence(1, None);
  random_query_random_sequence(20, None);
  random_query_random_sequence(50, None);
  random_query_random_sequence(250, None);
}

#[test]
pub fn test_medium_n() {
  random_query_random_sequence(1000, Some(3));
  random_query_random_sequence(10000, Some(3));
  random_query_random_sequence(100000, Some(3));
  random_query_random_sequence(1000, Some(100));
  random_query_random_sequence(10000, Some(100));
  random_query_random_sequence(100000, Some(100));
}

#[test]
pub fn test_big_n() {
  random_query_random_sequence(1000000, None);
  random_query_random_sequence(10000000, Some(3));
  random_query_random_sequence(10000000, Some(5));
  random_query_random_sequence(100000000, Some(1000));
  random_query_random_sequence(1000000, Some(4));
  random_query_random_sequence(10000000, Some(7));
  random_query_random_sequence(10000000, Some(10000));
  random_query_random_sequence(100000000, Some(10));
}

#[test]
pub fn test_big_n_binary() {
  random_query_random_sequence(1000001, Some(2));
  random_query_random_sequence(10000001, Some(2));
  random_query_random_sequence(10000001, Some(2));
  random_query_random_sequence(100000001, Some(2));
  random_query_random_sequence(1000000, Some(2));
  random_query_random_sequence(10000000, Some(2));
  random_query_random_sequence(10000000, Some(2));
  random_query_random_sequence(100000000, Some(2));
}