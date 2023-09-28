/// Maintains the invariant that p and q are connected if and only if id[p] is equal to id[q].
/// In other words, all sites in a component must have the same value in id[].
pub struct QuickFindUF {
    id: Vec<usize>, // id[i] = component identifier of i
    count: usize,   // number of components
}

impl QuickFindUF {
    pub fn new(n: usize) -> Self {
        let id = (0..n).collect();
        Self { id, count: n }
    }

    /// Returns the canonical element of the set containing element p
    pub fn find(&self, p: usize) -> usize {
        self.id[p]
    }

    /// Merges the set containing element p with the
    /// the set containing element q
    pub fn union(&mut self, p: usize, q: usize) {
        let p_id = self.id[p];
        let q_id = self.id[q];
        // p and q are already in the same component
        if p_id == q_id {
            return;
        }

        // Rename p’s component to q’s name
        for i in 0..self.id.len() {
            if self.id[i] == p_id {
                self.id[i] = q_id;
            }
        }

        self.count -= 1;
    }
}

macro_rules! uf_util {
    ($UF: ty) => {
        impl $UF {
            /// Returns the number of sets
            pub fn count(&self) -> usize {
                self.count
            }

            /// Returns true if the two elements are in the same set
            pub fn connected(&mut self, p: usize, q: usize) -> bool {
                self.find(p) == self.find(q)
            }
        }
    };
}

uf_util!(QuickFindUF);