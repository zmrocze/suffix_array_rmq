
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
  // println!("sa: {:?}", sa.sa);
  println!("lcp: {:?}", lcp);
  // tree of indices in the sorted order, arranged by lcp values
  let cart_tree = sa::cartesian_tree(&lcp);
  // array of the above indices, depths in the tree recorded
  let euler_walk = cart_tree.euler_walk();
  println!("euler: {:?}", euler_walk.euler);
  // RMQ on the euler traversal order with depths as keys
  let rmq_data = euler_walk.euler.iter().map(|&x| euler_walk.depth[x]).collect();
  println!("rmq data (euler depths): {:?}", rmq_data);
  let rmq = rmq::RMQ::create_rmq(&rmq_data);
  // can simplify and already keep the lcp value in rmq
  // let lcp_remap = (0..euler_walk.euler.len()).map(|i| lcp[euler_walk.euler[i]]).collect();
  // rmq.remap_argmins(&lcp_remap);
  // println!("lcp_remap: {:?}", lcp_remap);
  let lcp_euler: Vec<usize> = euler_walk.euler.iter().map(|i: &usize| lcp[*i]).collect();
  println!("lcp_euler: {:?}", lcp_euler);
  // println!("after rmq");
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
    println!("querying {:?} {:?}", i, j);
    //  = self.euler_walk_first_occ[self.sa_inverse[i]];
    // let (ii, jj) = (self.sa_inverse[i], self.sa_inverse[j]);
    // println!("pi_inv[i] pi_inv[j] {:?} {:?}. what means the suffixes i j are at these places in the sorted order", ii, jj);
    // println!("indices in euler walk: {:?}, {:?}", self.euler_walk_first_occ[ii], self.euler_walk_first_occ[jj]);
    // println!("euler walk: {:?}", self.euler_walk); 
    // println!("lcp: {:?}", self.lcp);
    if i == j { println!("i=j={:?}", i); return self.indice_into_rmq.len() - i }
    let ii = self.sa_inverse[i];
    let jj = self.sa_inverse[j];
    let (iii, jjj) =
      (self.indice_into_rmq[ii.min(jj)], self.indice_into_rmq[ii.max(jj)-1]);
    self.lcp_euler[
      self.rmq.query(iii.min(jjj), iii.max(jjj))
    ]
    // let ii = self.indice_into_rmq[i];
    // let jj = self.indice_into_rmq[j];
    // println!("wtf?");
    
    //   if ii <= jj {
    //     println!("querying rmq {:?} {:?}", ii, jj-1);
    //     let x = self.rmq.query(ii, jj-1);
    //     println!("got {:?}", x);
    //     x
    //   } else {
    //     println!("querying rmq {:?} {:?}", jj, ii-1);
    //     let x = self.rmq.query(jj, ii-1);
    //     println!("got {:?}", x);
    //     x
    //   }
    // ]
  }
}