use std::cell::Cell;

use fxhash::FxHashMap;

pub struct DisjointSet {
    parents: Vec<Cell<usize>>,
    ranks: Vec<u8>,
}

impl DisjointSet {
    /// Creates a new disjoint set where each of the elements are in their own set
    #[inline]
    #[must_use]
    pub fn with_len(len: usize) -> Self {
        Self { parents: (0..len).map(Cell::new).collect(), ranks: vec![0; len] }
    }

    /// Returns the total count of elements represented in this set
    pub fn len(&self) -> usize { self.parents.len() }

    /// Returns true if there are no elements in this set
    pub fn is_empty(&self) -> bool { self.parents.is_empty() }

    /// Returns an element in the set of the child, or None if the child >= len
    /// All elements in the same subset are represented by the same element.
    ///
    /// Important: There is no guarantee what the result represents next to the description above
    fn root_of(&self, child: usize) -> Option<usize> {
        if child < self.len() {
            // SAFETY: Did the manual index check
            Some(unsafe { self.unchecked_root_of(child) })
        } else {
            None
        }
    }

    /// Returns an element in the set of the child
    /// All elements in the same subset are represented by the same element.
    ///
    /// Important: There is no guarantee what the result represents next to the description above
    /// SAFETY: Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used.
    #[inline]
    #[must_use]
    unsafe fn unchecked_root_of(&self, mut child: usize) -> usize {
        let mut parent = unsafe { self.parents.get_unchecked(child) }.get();

        if child == parent {
            return child;
        };

        loop {
            // SAFETY: Any ID that we get from the list is known to be here
            let grandparent = unsafe { self.parents.get_unchecked(parent).get() };
            if parent == grandparent {
                return parent;
            }

            // SAFETY: The child is known to be a valid ID, since it was checked on the first line
            unsafe { self.parents.get_unchecked(child).set(grandparent) }
            child = parent;
            parent = grandparent;
        }
    }

    /// Joins the 2 given elements into the same subset.
    /// Will return `false` when either element doesn't exist in the set.
    /// Will return `true` when the joining succeeded and this object was updated.
    /// Will return `false` if the elements were already in the same set.
    #[inline]
    pub fn join(&mut self, first_element: usize, second_element: usize) -> bool {
        // Immediate parent check.
        match (
            self.parents.get(first_element),
            self.parents.get(second_element),
        ) {
            (None, _) => return false,
            (_, None) => return false,
            (Some(first), Some(second)) if first == second => return false,
            _ => {}
        }

        // SAFETY: If either element weren't valid indices, the check above would catch that
        let root_first = unsafe { self.unchecked_root_of(first_element) };
        let root_second = unsafe { self.unchecked_root_of(second_element) };

        if root_first == root_second {
            return false;
        }

        // SAFETY: The ranges of the ranks and parents are the same and they are checked to be disjoint
        let [rank_first, rank_second] =
            unsafe { self.ranks.get_disjoint_unchecked_mut([root_first, root_second]) };

        if rank_first < rank_second {
            // SAFETY: The root_first is a safe index determined by us
            unsafe { self.parents.get_unchecked(root_first) }.set(root_second);
        } else {
            if rank_first == rank_second {
                *rank_first += 1;
            }
            // SAFETY: The root_second is a safe index determined by us
            unsafe { self.parents.get_unchecked(root_second) }.set(root_first);
        }

        true
    }

    /// Returns `true` if the first and second element are in the same subset.
    /// Will return `false` when any of the elements fall outside of the set range.
    #[must_use]
    #[inline]
    pub fn is_joined(&self, first_element: usize, second_element: usize) -> bool {
        self.root_of(first_element)
            .is_some_and(|first_root| Some(first_root) == self.root_of(second_element))
    }

    /// Returns a iterator with all sets. Each entry corresponds to one set, and is a `Vec` of its elements.
    /// The elements in the vecs are guaranteed to be sorted from low to high index.
    /// There is no ordering guarantee from the iterator.
    pub fn sets(&self) -> impl Iterator<Item = Vec<usize>> + use<> {
        let mut collector = FxHashMap::<usize, Vec<usize>>::default();

        for index in 0..self.len() {
            // SAFETY: The index is from the correct range
            let root = unsafe { self.unchecked_root_of(index) };
            collector.entry(root).or_insert_with(|| Vec::with_capacity(1)).push(index);
        }

        collector.into_values()
    }
}
