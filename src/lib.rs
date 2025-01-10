
mod rmq;
mod sa;
mod tests;

/// Creates a SA+RMQ structure which allows to query for any two indices in the sequence,
/// what is the length of the longest common prefix of the suffixes starting at these indices.
/// O(n) creation time, O(1) query time
pub fn create_sarmq(a: &Vec<usize>) -> SARMQ {
  // sort suffixes
  let sa = sa::SA::create_suffix_array(a);
  println!("after sa");
  // lcp of neighbouring suffixes in the sorted order
  let lcp = sa::lcp(a, &sa);
  println!("after lcp");
  // tree of indices in the sorted order, arranged by lcp values
  let cart_tree = sa::cartesian_tree(&lcp);
  println!("after cart");
  // array of the above indices, depths in the tree recorded
  let euler_walk = cart_tree.euler_walk();
  println!("after euler");
  // RMQ on the depths as keys and the indices in sorted order as values
  let rmq = rmq::create_rmq(&euler_walk.euler, &euler_walk.depth);
  println!("after rmq");
  SARMQ { sa_inverse : sa.sa_inverse, lcp, euler_walk_first_occ : euler_walk.first_occ, rmq }
}

/// Implements O(1) time queries for the longest common prefix of the suffixes starting at the given indices. 
pub struct SARMQ {
  sa_inverse : Vec<usize>,
  lcp: Vec<usize>,
  euler_walk_first_occ: Vec<usize>,
  rmq: rmq::RMQ,
}

impl SARMQ {
  /// Returns the length of the longest common prefix of the suffixes starting at the given indices.
  pub fn query(&self, i: usize, j: usize) -> usize {
    println!("querying {:?} {:?}", i, j);
    let (ii, jj) = (self.sa_inverse[i], self.sa_inverse[j]);
    println!("pi_inv[i] pi_inv[j] {:?} {:?}. what means the suffixes i j are at these places in the sorted order", ii, jj);
    println!("indices in euler walk: {:?}, {:?}", self.euler_walk_first_occ[ii], self.euler_walk_first_occ[jj]); 
    println!("lcp: {:?}", self.lcp);
    println!("rmq starts: {:?}", self.rmq);
    let argmin = self.rmq.query(self.euler_walk_first_occ[ii], self.euler_walk_first_occ[jj]);
    self.lcp[argmin]
  }
}