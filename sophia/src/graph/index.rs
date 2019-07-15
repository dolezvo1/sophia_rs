//! Types for indexing terms.

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

use crate::term::factory::{FTerm, TermFactory};
use crate::term::*;

/// A bidirectionnal mapping between [`Term`]s and *indexes* of a smaller type.
///
/// The TermIndexMap maintains a reference count for each index,
/// to automatically free them whenever they are not used.
///
/// One special index (called the *null index*) is never mapped to any [`Term`].
///
/// [`Term`]: ../../term/enum.Term.html
///
pub trait TermIndexMap: Default {
    /// The type used to represent terms
    type Index: Copy + Eq;
    /// The factory used to instantiate terms.
    type Factory: TermFactory;

    // A reserved index representing no term
    const NULL_INDEX: Self::Index;

    /// Return the index associated to the given term, if it exists.
    fn get_index(&self, t: &RefTerm) -> Option<Self::Index>;
    /// Return the index associated to the given term, creating it if required, and increasing its ref count.
    fn make_index(&mut self, t: &RefTerm) -> Self::Index;
    /// Return the term associated to the given index, if it exists.
    fn get_term(&self, i: Self::Index) -> Option<&FTerm<Self::Factory>>;
    /// Increase the reference count of a given (non-null) index.
    fn inc_ref(&mut self, i: Self::Index);
    /// Decrease the reference count of a given (non-null) index.
    fn dec_ref(&mut self, i: Self::Index);
    /// Shrinks the capacity of the TermIndexMap as much as possible.
    fn shrink_to_fit(&mut self);
}

/// A utility trait for implementing [`Graph`] and [`MutableGraph`]
/// based on an internal [`TermIndexMap`] for efficient storage.
///
/// The `impl_mutable_graph_for_indexed_mutable_graph!` macro
/// can be used to derive the `MutableGraph` implementation
/// for any implementation of `IndexedGraph`.
///
/// [`Graph`]: ../trait.Graph.html
/// [`MutableGraph`]: ../trait.MutableGraph.html
/// [`TermIndexMap`]: trait.TermIndexMap.html
///
pub trait IndexedGraph {
    /// The type used to represent terms internally.
    type Index: Copy + Eq + Hash;
    type TermData: TermData + 'static;

    /// Return the index for the given term, if it exists.
    fn get_index<T>(&self, t: &Term<T>) -> Option<Self::Index>
    where
        T: TermData;

    /// Return the term for the given index, if it exists.
    fn get_term(&self, i: Self::Index) -> Option<&Term<Self::TermData>>;

    /// Insert a triple in this Graph,
    /// and return the corresponding tuple of indices.
    fn insert_indexed<T, U, V>(
        &mut self,
        s: &Term<T>,
        p: &Term<U>,
        o: &Term<V>,
    ) -> Option<[Self::Index; 3]>
    where
        T: TermData,
        U: TermData,
        V: TermData;

    /// Remove a triple from this Graph,
    /// and return the corresponding tuple of indices.
    fn remove_indexed<T, U, V>(
        &mut self,
        s: &Term<T>,
        p: &Term<U>,
        o: &Term<V>,
    ) -> Option<[Self::Index; 3]>
    where
        T: TermData,
        U: TermData,
        V: TermData;

    fn shrink_to_fit(&mut self);
}

/// Defines the implementation of [`MutableGraph`] for [`IndexedGraph`].
///
/// [`MutableGraph`]: graph/trait.MutableGraph.html
/// [`IndexedGraph`]: graph/index/trait.IndexedGraph.html
#[macro_export]
macro_rules! impl_mutable_graph_for_indexed_mutable_graph {
    ($indexed_mutable_graph: ty) => {
        impl MutableGraph for $indexed_mutable_graph {
            impl_mutable_graph_for_indexed_mutable_graph!();
        }
    };
    () => {
        type MutationError = coercible_errors::Never;

        fn insert<T_, U_, V_> (&mut self, s: &Term<T_>, p: &Term<U_>, o: &Term<V_>) -> MGResult< Self, bool> where
            T_: $crate::term::TermData,
            U_: $crate::term::TermData,
            V_: $crate::term::TermData,
        {
            Ok(self.insert_indexed(s, p, o).is_some())
        }
        fn remove<T_, U_, V_> (&mut self, s: &Term<T_>, p: &Term<U_>, o: &Term<V_>) -> MGResult< Self, bool> where
            T_: $crate::term::TermData,
            U_: $crate::term::TermData,
            V_: $crate::term::TermData,
        {
            Ok(self.remove_indexed(s, p, o).is_some())
        }
    };
}

/// Insert an absent value in the Vec value of a HashMap,
/// creating the Vec if it does not exist.
///
/// # Returns
///
/// `true` if the Vec was created,
///  meaning that "parent" indexes need to be updated.
///
pub(crate) fn insert_in_index<K, W>(hm: &mut HashMap<K, Vec<W>>, k: K, w: W) -> bool
where
    K: Eq + Hash,
    W: Copy + Eq,
{
    let mut ret = false;
    hm.entry(k).or_insert_with(|| {
        ret = true;
        Vec::new()
    }).push(w);
    ret
}

/// Remove an existing value in the Vec value of a HashMap,
/// removing the entry completely if the Vec ends up empty.
///
/// # Returns
///
/// `true` if the entry was removed,
///  meaning that "parent" indexes need to be updated.
///
/// # Panics
///
/// This function will panic if either
/// * `k` is not a key of `hm`, or
/// * `w` is not contained in the value associated to `k`.
pub(crate) fn remove_from_index<K, W>(hm: &mut HashMap<K, Vec<W>>, k: K, w: W) -> bool
where
    K: Eq + Hash,
    W: Copy + Eq,
{
    match hm.entry(k) {
        Entry::Occupied(mut e) => {
            {
                let ws = e.get_mut();
                if ws.len() > 1 {
                    let wi = ws
                        .iter()
                        .enumerate()
                        .filter_map(|(i, w2)| if *w2 == w { Some(i) } else { None })
                        .next()
                        .unwrap();
                    ws.swap_remove(wi);
                    return false;
                }
            }
            e.remove_entry();
            return true;
        }
        Entry::Vacant(_) => unreachable!(),
    }
}

#[cfg(test)]
/// Takes an empty TermIndexMap, and checks that it behaves as expected.
pub fn assert_term_index_works<T: TermIndexMap>(ti: &mut T) {
    let t = RefTerm::new_iri("http://example.org/").unwrap();
    assert!(ti.get_index(&t).is_none());

    // insert term, then remove it

    let it = ti.make_index(&t);
    assert!(ti.get_index(&t).is_some());
    assert!(ti.get_index(&t).unwrap() == it);
    assert!(ti.get_term(it).is_some());
    assert!(ti.get_term(it).unwrap() == &t);

    ti.dec_ref(it);
    assert!(ti.get_index(&t).is_none());
    assert!(ti.get_term(it).is_none());

    // insert term twice, then remove it

    let it = ti.make_index(&t);
    assert!(ti.get_index(&t).is_some());
    assert!(ti.get_index(&t).unwrap() == it);
    assert!(ti.get_term(it).is_some());
    assert!(ti.get_term(it).unwrap() == &t);

    let it2 = ti.make_index(&t);
    assert!(it == it2);

    ti.dec_ref(it);
    assert!(ti.get_index(&t).is_some());
    assert!(ti.get_index(&t).unwrap() == it);
    assert!(ti.get_term(it).is_some());
    assert!(ti.get_term(it).unwrap() == &t);

    ti.dec_ref(it);
    assert!(ti.get_index(&t).is_none());
    assert!(ti.get_term(it).is_none());

    // insert term, incref it, then remove it

    let it = ti.make_index(&t);
    assert!(ti.get_index(&t).is_some());
    assert!(ti.get_index(&t).unwrap() == it);
    assert!(ti.get_term(it).is_some());
    assert!(ti.get_term(it).unwrap() == &t);

    ti.inc_ref(it);

    ti.dec_ref(it);
    assert!(ti.get_index(&t).is_some());
    assert!(ti.get_index(&t).unwrap() == it);
    assert!(ti.get_term(it).is_some());
    assert!(ti.get_term(it).unwrap() == &t);

    ti.dec_ref(it);
    assert!(ti.get_index(&t).is_none());
    assert!(ti.get_term(it).is_none());

    // insert two terms, then remove them

    let t1 = t;
    let t2 = RefTerm::new_iri("http://example.org/2").unwrap();
    let it1 = ti.make_index(&t1);
    let it2 = ti.make_index(&t2);
    assert!(it1 != it2);

    ti.dec_ref(it2);
    assert!(ti.get_index(&t1).is_some());
    assert!(ti.get_index(&t2).is_none());

    ti.dec_ref(it1);
    assert!(ti.get_index(&t1).is_none());
    assert!(ti.get_index(&t2).is_none());
}

#[cfg(test)]
mod test {
    // Nothing really worth testing here
}
