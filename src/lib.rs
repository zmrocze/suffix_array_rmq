
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
  println!("after euler: {:?}", euler_walk.euler);
  // RMQ on the euler traversal order with depths as keys
  let rmq_data = euler_walk.euler.iter().map(|&x| euler_walk.depth[x]).collect();
  let mut rmq = rmq::RMQ::create_rmq(&rmq_data);
  // can simplify and already keep the lcp value in rmq
  let lcp_remap = (0..euler_walk.euler.len()).map(|i| lcp[euler_walk.euler[i]]).collect();
  rmq.remap_argmins(&lcp_remap);
  // can simplify and combine the sa_inverse and euler_walk_first_occ indice maps
  let indice_into_rmq: Vec<usize> = (0..a.len()).map(|i| euler_walk.first_occ[sa.sa_inverse[i]]).collect();
  println!("after rmq");
  SARMQ { indice_into_rmq, rmq }
}

/// Implements O(1) time queries for the longest common prefix of the suffixes starting at the given indices. 
pub struct SARMQ {
  indice_into_rmq: Vec<usize>,
  rmq: rmq::RMQ,
}

impl SARMQ {
  /// Returns the length of the longest common prefix of the suffixes starting at the given indices.
  pub fn query(&self, i: usize, j: usize) -> usize {
    println!("querying {:?} {:?}", i, j);
    //  = self.euler_walk_first_occ[self.sa_inverse[i]];
    // let (ii, jj) = (self.sa_inverse[i], self.sa_inverse[j]);
    // println!("pi_inv[i] pi_inv[j] {:?} {:?}. what means the suffixes i j are at these places in the sorted order", ii, jj);
    // println!("indices in euler walk: {:?}, {:?}", self.euler_walk_first_occ[ii], self.euler_walk_first_occ[jj]);
    // println!("euler walk: {:?}", self.euler_walk); 
    // println!("lcp: {:?}", self.lcp);
    self.rmq.query(self.indice_into_rmq[i], self.indice_into_rmq[j])
    // println!("argmin: {:?}", argmin);
    // self.lcp[ self.euler_walk[argmin] ]
  }
}