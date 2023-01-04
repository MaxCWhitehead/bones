use std::{alloc::Layout, marker::PhantomData, slice::ChunksExactMut};

use crate::prelude::*;

/// Read-only iterator over components matching a given bitset
pub struct ComponentBitsetIterator<'a, T> {
    iter: UntypedComponentBitsetIterator<'a>,
    _phantom: PhantomData<T>,
}

impl<'a, T> ComponentBitsetIterator<'a, T> {
    /// # Safety
    /// The untyped iterator must be valid for type T.
    pub(crate) unsafe fn new(iter: UntypedComponentBitsetIterator<'a>) -> Self {
        Self {
            iter,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: 'static> Iterator for ComponentBitsetIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            // SAFE: It is unsafe to construct this iterator, and user affirms that untyped iterator
            // is valid for type T.
            .map(|x| unsafe { &*(x.as_ptr() as *const T) })
    }
}

/// Mutable iterator over components matching a given bitset
pub struct ComponentBitsetIteratorMut<'a, T> {
    iter: UntypedComponentBitsetIteratorMut<'a>,
    _phantom: PhantomData<T>,
}

impl<'a, T> ComponentBitsetIteratorMut<'a, T> {
    /// # Safety
    /// The untyped iterator must be valid for type T.
    pub(crate) unsafe fn new(iter: UntypedComponentBitsetIteratorMut<'a>) -> Self {
        Self {
            iter,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: 'static> Iterator for ComponentBitsetIteratorMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            // SAFE: It is unsafe to construct this iterator, and user affirms that untyped iterator
            // is valid for type T.
            .map(|x| unsafe { &mut *(x.as_mut_ptr() as *mut T) })
    }
}

/// Iterates over components using a provided bitset.
/// Each time the bitset has a 1 in index i, the iterator will fetch data
/// from the storage at index i and return it as an `Option`.
pub struct UntypedComponentBitsetIterator<'a> {
    pub(crate) current_id: usize,
    pub(crate) components: &'a UntypedComponentStore,
    pub(crate) bitset: &'a BitSetVec,
}

impl<'a> Iterator for UntypedComponentBitsetIterator<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        let max_id = self.components.max_id;
        let size = self.components.layout.size();
        while !(self.bitset.bit_test(self.current_id)
            && self.components.bitset.bit_test(self.current_id))
            && self.current_id <= max_id
        {
            self.current_id += 1;
        }
        let ret = if self.current_id <= max_id {
            let start = self.current_id * size;
            let end = start + size;
            Some(&self.components.storage[start..end])
        } else {
            None
        };
        self.current_id += 1;
        ret
    }
}

/// Iterates over components using a provided bitset.
/// Each time the bitset has a 1 in index i, the iterator will fetch data
/// from the storage at index i and return it as an `Option`.
pub struct UntypedComponentBitsetIteratorMut<'a> {
    pub(crate) current_id: usize,
    pub(crate) components: Option<ChunksExactMut<'a, u8>>,
    pub(crate) components_bitset: &'a BitSetVec,
    pub(crate) bitset: &'a BitSetVec,
    pub(crate) layout: Layout,
    pub(crate) max_id: usize,
}

impl<'a> Iterator for UntypedComponentBitsetIteratorMut<'a> {
    type Item = &'a mut [u8];
    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            current_id,
            bitset,
            components,
            components_bitset,
            layout,
            max_id,
        } = self;
        let chunk = components.as_mut().and_then(|x| x.next());
        let max_id = max_id;
        while !(bitset.bit_test(*current_id) && components_bitset.bit_test(*current_id))
            && *current_id <= *max_id
        {
            *current_id += 1;
        }
        let ret = if *current_id <= *max_id {
            if layout.size() != 0 {
                let bytes = chunk.unwrap();
                Some(bytes)
            } else {
                Some(&mut [] as &mut [u8])
            }
        } else {
            None
        };
        self.current_id += 1;
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, TypeUlid)]
    #[ulid = "01GNZ7A42K6KTPTFEQ3T445DNZ"]
    struct A;

    #[test]
    fn iter_with_empty_bitset() {
        let mut entities = Entities::default();
        let e = entities.create();
        let mut components = ComponentStore::<A>::default();

        components.insert(e, A);

        let bitset = BitSetVec::default();
        assert_eq!(components.iter_with_bitset(&bitset).count(), 0);
        assert_eq!(components.iter_mut_with_bitset(&bitset).count(), 0);
    }
}
