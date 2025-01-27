
mod rmq;
mod sa;
mod tests;

/// Creates a SA+RMQ structure which allows to query for any two indices in the sequence,
/// what is the length of the longest common prefix of the suffixes starting at these indices.
/// O(n) creation time, O(1) query time
pub fn create_sarmq(a: &Vec<usize>) -> SARMQ {
  // sort suffixes
  let sa = sa::SA::create_suffix_array(a);
  // lcp of neighbouring suffixes in the sorted order
  let lcp = sa::lcp(a, &sa);
  // tree of indices in the sorted order, arranged by lcp values
  let cart_tree = sa::cartesian_tree(&lcp);
  // array of the above indices, depths in the tree recorded
  let euler_walk = cart_tree.euler_walk();
  // RMQ on the euler traversal order with depths as keys
  let rmq_data = euler_walk.euler.iter().map(|&x| euler_walk.depth[x]).collect();
  let rmq = rmq::RMQ::create_rmq(&rmq_data);
  // for getting lcp values back
  let lcp_euler: Vec<usize> = euler_walk.euler.iter().map(|i: &usize| lcp[*i]).collect();
  SARMQ { indice_into_rmq : euler_walk.first_occ, rmq, lcp_euler: lcp_euler, sa_inverse: sa.sa_inverse }
}

/// Implements O(1) time queries for the longest common prefix of the suffixes starting at the given indices. 
pub struct SARMQ {
  indice_into_rmq: Vec<usize>,
  rmq: rmq::RMQ,
  lcp_euler : Vec<usize>,
  sa_inverse: Vec<usize>,
}

impl SARMQ {
  /// Returns the length of the longest common prefix of the suffixes starting at the given indices.
  pub fn query(&self, i: usize, j: usize) -> usize {
    if i == j { return self.indice_into_rmq.len() - i }
    let ii = self.sa_inverse[i];
    let jj = self.sa_inverse[j];
    let (iii, jjj) =
      (self.indice_into_rmq[ii.min(jj)], self.indice_into_rmq[ii.max(jj)-1]);
    self.lcp_euler[
      self.rmq.query(iii.min(jjj), iii.max(jjj))
    ]
  }
}