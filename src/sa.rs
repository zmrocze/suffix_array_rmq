use std::vec;

pub struct SA {
  pub sa: Vec<usize>,
  pub sa_inverse: Vec<usize>,
}

impl SA {

  /// Calculates the lexicographical order on suffixes of a.
  /// O(n) time and space.
  /// Assumes integers in 0..n range.
  pub fn create_suffix_array(a: &Vec<usize>) -> Self {
    // let mut s: Vec<usize> = (0 .. a.len()).collect();
    // s.sort_by_key(|&i| &a[i..]);
    let s = sa(a);
    // assert_eq!(ss, s);
    let mut sa_inverse: Vec<usize> = vec![0; a.len()];
    s.iter().enumerate().for_each(|(i, x)| sa_inverse[*x] = i);
    SA {
      sa: s,
      sa_inverse,
    }
  }

}

/// sort suffixes of xs in O(n)
/// assumes xs contains numbers in range 0..n
fn sa(xs : &Vec<usize>) -> Vec<usize> {
  // sort recursively suffixes of triplets mod3=0 and mod3=1,
  // then merge with mod3=2 suffixes
  let n = xs.len();

  let c= 10;
  if n < c {
    let mut r: Vec<usize> = (0..n).collect();
    r.sort_by_key(|&i| &xs[i..]);
    return r;
  }

  fn sort_triples(n : usize, triples : &Vec<(usize, usize, usize)>) -> (Vec<usize>, Vec<usize>) {
    let phase = |xs : Vec<usize>, pi : Box<dyn Fn((usize, usize, usize)) -> usize>| {
      let mut vs = vec![ vec![] ; n];
      for i in xs {
        vs[pi(triples[i])].push(i);
      }
      vs.concat()
    };
    let ph2 = phase((0..(triples.len())).collect(), Box::new(|x| x.2));
    let ph1 = phase(ph2, Box::new(|x| x.1));
    let ph0 = phase(ph1, Box::new(|x| x.0));
    // now collapse equal triples
    let mut group_id = 0;
    let mut groups = vec![0; ph0.len()];
    if !ph0.is_empty() {
      groups[ph0[0]] = 0;
      for i in 1..ph0.len() {
        if triples[ph0[i]] != triples[ph0[i - 1]] {
          group_id += 1;
        }
        groups[ph0[i]] = group_id;
      }
    }
    (ph0, groups) // (order, assignment (ranks but repeating for equal))
  }
  fn sort_tuples(n : usize, tuples : &Vec<(usize, usize)>) -> (Vec<usize>, Vec<usize>) {
    sort_triples(n, &tuples.iter().map(|(a, b)| (*a, *b, 0)).collect::<Vec<_>>())
  }
  fn invert(per : &Vec<usize>) -> Vec<usize> {
    let mut ys = vec![0; per.len()];
    for (i, x) in per.iter().enumerate() {
      ys[*x] = i;
    }
    ys
  }

  let triples : Vec<(usize, usize, usize)> = {
    let into_triple = |c: &[usize]| {
      let mut c = c.iter();
      let a = *c.next().unwrap();
      let b = *c.next().unwrap_or(&0);
      let c = *c.next().unwrap_or(&0);
      (a, b, c)
    };
    xs.chunks(3).map(into_triple)
      .chain(
    xs[1..].chunks(3).map(into_triple)).collect()
  };

  // need to move by 1, to use 0 as -inf in between mod3=0 and mod3=1 suffixes
  let (_, triples) = sort_triples(n+1, &triples);
  let mut triples = triples.iter().map(|&x| x+1).collect::<Vec<usize>>();
  let k = n.div_ceil(3); // middle
  triples.insert(k, 0); // insert a "#" splitter

  let mod01: Vec<usize> = sa(&triples);

  let ranks = invert(&mod01);
  let index = |i: usize| { // i-th suffix in sorted order
    if i % 3 == 0 {i/3} else {k + 1 + (i-1)/3}
  };
  let rev_index = |j: usize| { // j-th sorted suffix
    if j < k {3*j} else {3*(j-k-1) + 1}
  };
  let get = |xs : &Vec<usize>, i: usize| {xs.get(i).map(|&r|r+1).unwrap_or(0)};
  let rank = |i: usize| {get(&ranks, index(i))};
  // compare suffix l-th with r-th where l % 3 = 2
  let cmp = |l:usize,r:usize| {
    // empty =0, nonempty >=1
    let l0 = get(&xs, l);
    let r0 = get(&xs, r);
    if r % 3 == 0 || r % 3 == 2 {
      (l0, rank(l+1)).le(&(r0, rank(r+1)))
    } else { // r % 3 = 1
      let l1 = get(xs, l+1);
      let r1 = get(xs, r+1);
      (l0, l1, rank(l+2)).le(&(r0, r1, rank(r+2)))
    }
  };

  // sorting mod=2 suffixes
  let (mod2, _)= sort_tuples(n+1,&(0..((n-2).div_ceil(3))).map(|i| (xs[3*i+2], rank(3*i+3))).collect::<Vec<(usize, usize)>>());

  // merging
  let mut res = vec![];
  let mut mod2 = mod2.iter().map(|i| 3*i + 2).collect::<Vec<usize>>();
  let mut mod01 = mod01.into_iter().filter_map(|i| if i != k {Some(rev_index(i))} else {None}).collect::<Vec<usize>>();
  mod2.reverse();
  mod01.reverse();
  while ! mod2.is_empty() && ! mod01.is_empty() {
    let l = *mod2.last().unwrap();
    let r = *mod01.last().unwrap();
    // assert!(l < n && r < n, "indices within range: {:?} {:?}", l, r);
    if cmp(l, r) {
      res.push(l);
      mod2.pop();
    } else {
      res.push(r);
      mod01.pop();
    }
  }
  mod2.reverse();
  mod01.reverse();
  res.extend(mod2.into_iter());
  res.extend(mod01.into_iter());
  
  return res;
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
  pub root : usize,
  pub lefts : Vec<Option<usize>>,
  pub rights : Vec<Option<usize>>,
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
      if rightmost == root {
        // no right kid in root
        parent[i] = root;
        is_left[i] = false;
      } else if keys[rightmost] >= keys[i] {
        while keys[parent[rightmost]] >= keys[i]  { // surely false when we parent[rightmost] = root
          rightmost = parent[rightmost];
        }
        parent[i] = parent[rightmost];
        is_left[i] = false;
        parent[rightmost] = i;
        is_left[rightmost] = true;
      } else {
        parent[i] = rightmost;
        is_left[i] = false;
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
  BinTree {
    root: root,
    lefts: lefts,
    rights: rights,
  }
}
