#[cfg(test)]
use super::*;

use rand::Rng;

// #[test]
pub fn random_query_random_sequence( n : usize ) {
  let mut rng = rand::thread_rng();
  let mut xs: Vec<usize> = (0..n).collect();
  let mut keys = vec![];
  for i in 0..n {
    keys.push(rng.gen_range(0..usize::MAX))
  }
  
  let rmq = create_rmq(xs, keys);
  for _ in 0..1000 {
    let left = rand::random::<usize>() % n;
    let right = rand::random::<usize>() % n;
    let (left, right) = (left.min(right), left.max(right));
    let mut min = usize::MAX;
    for i in left..=right {
      min = min.min(xs[i]);
    }
    assert_eq!(min, rmq.query(left, right));
  }
}