use std::vec;

pub struct SA {
  pub sa: Vec<usize>,
  pub sa_inverse: Vec<usize>,
}

impl SA {

  /// Calculates the lexicographical order on suffixes of a.
  /// O(n*logn) time and space.
  pub fn create_suffix_array(a: &Vec<usize>) -> Self {
    let mut s: Vec<usize> = (0 .. a.len()).collect();
    s.sort_by_key(|&i| &a[i..]);
    let mut sa_inverse: Vec<usize> = vec![0; a.len()];
    s.iter().enumerate().for_each(|(i, x)| sa_inverse[*x] = i);
    SA {
      sa: s,
      sa_inverse,
    }
  }

}

/// Calculates array lcp, st. lcp(i) = lcp(SA[i],SA[i+1]).
/// Uses the fact that lcp[SA_inv[i]] − 1 ≤ lcp[SA_inv[i + 1]]
/// O(n) time, O(1) extra space
pub fn lcp(a: &Vec<usize>, sa: &SA) -> Vec<usize> {
  let n = a.len();
  let mut lcp_acc = 0;
  let mut lcp = vec![0; n];
  for i in 0..n {
    // calculating lcp between i and i+1 suffixes of SA
    // starting value of lcp_acc
    if sa.sa_inverse[i] == n - 1 {
      lcp_acc = 0; // lcp[n-1] is unused really
    } else {
      let j = sa.sa[sa.sa_inverse[i]+1];
      while i+lcp_acc < n && j+lcp_acc < n && a[i+lcp_acc] == a[j+lcp_acc] {
        lcp_acc += 1;
      }
    }
    lcp[sa.sa_inverse[i]]= lcp_acc;
    if lcp_acc > 0 {lcp_acc -= 1}
  }
  lcp
}

/// Bin tree in array form.
pub struct BinTree {
  root : usize,
  lefts : Vec<Option<usize>>,
  rights : Vec<Option<usize>>,
}

/// Traversal order on a binary tree where a node is visited up to three times, when entering and then when re-entering from its kids
/// with mapped first occurences of each node and depths. 
pub struct Euler {
  pub euler: Vec<usize>,
  pub first_occ: Vec<usize>,
  pub depth: Vec<usize>,
}

impl BinTree {
  /// produces traversal order where a node is visited up to three times, when entering and then when re-entering from its kids
  pub fn euler_walk(&self) -> Euler {
    let mut depth: Vec<usize> = vec![0; self.lefts.len()];
    let mut first_occ = vec![0; self.lefts.len()];
    let mut euler = vec![];
    let mut stack = vec![(self.root, true)]; // true for entering, false for re-entering
    depth[self.root] = 0;
    while ! stack.is_empty() {
      let (x, entering)= stack.pop().unwrap(); 
      euler.push(x);
      if entering {
        // println!("entering {}", x);
        first_occ[x] = euler.len()-1;
        if let Some(right) = self.rights[x] {
          stack.push((x, false));
          stack.push((right, true));
          depth[right] = depth[x] + 1;
        }
        if let Some(left) = self.lefts[x] {
          stack.push((x, false));
          stack.push((left, true));
          depth[left] = depth[x] + 1;
        }
      }
    }
    return Euler { euler: euler, first_occ: first_occ, depth: depth };
  }
}

/// Cartesian tree on 0..n by keys: Bin(minimum in a, cartesian tree of a[0..min_index), cartesian tree of a[min_index+1..n))
/// O(n) time, O(1) extra space
pub fn cartesian_tree(keys: &Vec<usize>) -> BinTree {
  // go from left to right, putting the new node either as new root or somewhere along the rightmost path
  // note: using 0..n as nodes, but keys[xs[i]] as corresponding keys, then putting xs[i] in the final result also
  let n = keys.len();
  let mut parent: Vec<usize> = vec![0; n];
  let mut is_left = vec![true; n];
  let mut root = 0;
  let mut rightmost = 0;
  for i in 1..n {
    // let x = xs[i];
    if keys[i] <= keys[root] {
      parent[root] = i;
      root = i;
      rightmost = root;
    } else {
      println!("rightmost: {}", rightmost);
      if rightmost == root {
        // no right kid in root
        parent[i] = root;
        is_left[i] = false;
      } else {
        while keys[parent[rightmost]] >= keys[i]  { // surely false when we parent[rightmost] = root
          rightmost = parent[rightmost];
        }
        parent[i] = parent[rightmost];
        is_left[i] = false;
        parent[rightmost] = i;
        is_left[rightmost] = true;
      }
      rightmost = i;
    }
  }
  
  // reorganize into normal top down tree structure
  let mut lefts = vec![None; n];
  let mut rights = vec![None; n];
  for i in 0..n {
    if i != root {
      if is_left[i] {
        lefts[parent[i]] = Some(i);
      } else {
        rights[parent[i]] = Some(i);
      }
    }
  }
  println!("{:?}, {:?}, {:?}, {:?}", &keys, parent, is_left, rightmost);
  BinTree {
    root: root,
    lefts: lefts,
    rights: rights,
  }
}
