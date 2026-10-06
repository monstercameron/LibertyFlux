//! Generational arenas: how lifted code refers to objects.
//!
//! The original links objects with raw 32-bit pointers. Lifted (portable)
//! code never holds an address: every object that other objects refer to
//! lives in an [`Arena`], and references are [`Handle`]s, a slot index plus
//! a generation. A handle is two `u32`s on every target, so nothing about
//! it depends on pointer width, and `Option<Handle<T>>` is the same size
//! as `Handle<T>` (the generation is never zero), which is how a nullable
//! original pointer is carried.
//!
//! Rules (specification):
//! - A handle names at most one live value. Removing a value bumps its
//!   slot's generation, so every older handle to that slot stops resolving
//!   ([`Arena::get`] returns `None`) instead of aliasing the next occupant.
//! - A slot whose generation would wrap is retired, never reused.
//! - [`Arena::index`] (the `arena[handle]` form) panics on a stale or
//!   foreign handle. That is the lifted form of the original dereferencing
//!   a bad pointer: the lift fails loudly where the original would fault,
//!   and never guesses (rule from the first lift pilot).
//! - Handles are ordered and hashable so boundary maps can key on them.
//!
//! The arena is deliberately small: insert, look up, remove, iterate. It is
//! not a general-purpose allocator and never reorders its slots, so a
//! handle's index is stable for the value's lifetime (the differential
//! tests rely on that to map handles to 32-bit addresses).

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::num::NonZeroU32;
use core::ops::{Index, IndexMut};

/// A typed reference to a value in an [`Arena<T>`].
///
/// Eight bytes on every target: a `u32` slot index and a non-zero `u32`
/// generation. The `PhantomData<fn() -> T>` marker keeps the handle `Send`,
/// `Sync` and covariant whatever `T` is; a handle owns nothing.
pub struct Handle<T> {
    index: u32,
    generation: NonZeroU32,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Handle<T> {
    /// Builds a handle from its parts. Only boundary code (tests, the
    /// differential harness, save-state readers) needs this; lifted code
    /// gets handles from [`Arena::insert`].
    #[must_use]
    pub const fn from_parts(index: u32, generation: NonZeroU32) -> Self {
        Self {
            index,
            generation,
            _marker: PhantomData,
        }
    }

    /// The slot index.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.index
    }

    /// The slot generation this handle was issued for.
    #[must_use]
    pub const fn generation(self) -> NonZeroU32 {
        self.generation
    }

    /// Packs the handle into one `u64` (generation high, index low), for
    /// logs and maps that want a plain number.
    #[must_use]
    pub const fn to_bits(self) -> u64 {
        ((self.generation.get() as u64) << 32) | self.index as u64
    }

    /// Unpacks [`Self::to_bits`]; `None` when the generation half is zero.
    #[must_use]
    pub const fn from_bits(bits: u64) -> Option<Self> {
        // The halves are taken explicitly so the truncation is the point,
        // not an accident.
        let index = (bits & 0xFFFF_FFFF) as u32;
        match NonZeroU32::new((bits >> 32) as u32) {
            Some(generation) => Some(Self::from_parts(index, generation)),
            None => None,
        }
    }

    /// Re-types the handle. Boundary code uses this when one 32-bit
    /// address is known to hold an object of a more specific type.
    #[must_use]
    pub const fn cast<U>(self) -> Handle<U> {
        Handle::from_parts(self.index, self.generation)
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Handle<T> {}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.generation == other.generation
    }
}

impl<T> Eq for Handle<T> {}

impl<T> PartialOrd for Handle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Handle<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.index, self.generation).cmp(&(other.index, other.generation))
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.generation.hash(state);
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle({}v{})", self.index, self.generation)
    }
}

/// One arena slot: a live value, or the generation the next value gets.
#[derive(Clone, Debug)]
enum Slot<T> {
    Occupied { generation: NonZeroU32, value: T },
    Vacant { next_generation: Option<NonZeroU32> },
}

/// Values of one type, addressed by [`Handle<T>`].
#[derive(Clone, Debug)]
pub struct Arena<T> {
    slots: Vec<Slot<T>>,
    /// Vacant slot indices that may be reused, most recent last.
    free: Vec<u32>,
    len: usize,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Arena<T> {
    /// An empty arena.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            slots: Vec::new(),
            free: Vec::new(),
            len: 0,
        }
    }

    /// Number of live values.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// True when no value is live.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Stores `value` and returns its handle.
    ///
    /// # Panics
    ///
    /// When the arena already has `u32::MAX` slots (a handle index is a
    /// `u32` on every target).
    pub fn insert(&mut self, value: T) -> Handle<T> {
        self.len += 1;
        while let Some(index) = self.free.pop() {
            let slot = &mut self.slots[index as usize];
            if let Slot::Vacant {
                next_generation: Some(generation),
            } = *slot
            {
                *slot = Slot::Occupied { generation, value };
                return Handle::from_parts(index, generation);
            }
        }
        let index = u32::try_from(self.slots.len()).expect("arena holds at most u32::MAX slots");
        assert!(index != u32::MAX, "arena holds at most u32::MAX slots");
        self.slots.push(Slot::Occupied {
            generation: NonZeroU32::MIN,
            value,
        });
        Handle::from_parts(index, NonZeroU32::MIN)
    }

    /// The value `handle` names, or `None` when it is stale or out of range.
    #[must_use]
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        match self.slots.get(handle.index as usize)? {
            Slot::Occupied { generation, value } if *generation == handle.generation => Some(value),
            _ => None,
        }
    }

    /// Mutable form of [`Self::get`].
    #[must_use]
    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        match self.slots.get_mut(handle.index as usize)? {
            Slot::Occupied { generation, value } if *generation == handle.generation => Some(value),
            _ => None,
        }
    }

    /// True when `handle` names a live value.
    #[must_use]
    pub fn contains(&self, handle: Handle<T>) -> bool {
        self.get(handle).is_some()
    }

    /// Removes and returns the value `handle` names. Every copy of the
    /// handle stops resolving. Returns `None` for a stale handle.
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        let slot = self.slots.get_mut(handle.index as usize)?;
        match slot {
            Slot::Occupied { generation, .. } if *generation == handle.generation => {}
            _ => return None,
        }
        let next_generation = handle.generation.checked_add(1);
        let old = core::mem::replace(slot, Slot::Vacant { next_generation });
        self.len -= 1;
        // A slot whose generation wrapped is retired: it is never put back
        // on the free list, so no handle can ever alias.
        if next_generation.is_some() {
            self.free.push(handle.index);
        }
        match old {
            Slot::Occupied { value, .. } => Some(value),
            Slot::Vacant { .. } => None,
        }
    }

    /// Live values with their handles, in slot order.
    pub fn iter(&self) -> impl Iterator<Item = (Handle<T>, &T)> {
        self.slots
            .iter()
            .enumerate()
            .filter_map(|(i, slot)| match slot {
                Slot::Occupied { generation, value } => {
                    // Slot indices never exceed u32::MAX (see insert).
                    let index = u32::try_from(i).ok()?;
                    Some((Handle::from_parts(index, *generation), value))
                }
                Slot::Vacant { .. } => None,
            })
    }
}

impl<T> Index<Handle<T>> for Arena<T> {
    type Output = T;

    /// # Panics
    ///
    /// On a stale or foreign handle: the lifted form of a bad dereference.
    fn index(&self, handle: Handle<T>) -> &T {
        match self.get(handle) {
            Some(value) => value,
            None => panic!("stale or foreign {handle:?}"),
        }
    }
}

impl<T> IndexMut<Handle<T>> for Arena<T> {
    /// # Panics
    ///
    /// On a stale or foreign handle, like [`Index::index`].
    fn index_mut(&mut self, handle: Handle<T>) -> &mut T {
        match self.get_mut(handle) {
            Some(value) => value,
            None => panic!("stale or foreign {handle:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Arena, Handle};
    use crate::assert_size;
    use core::num::NonZeroU32;

    // Pointer-width independence: a handle is two u32s on every target,
    // and the non-zero generation gives Option a free niche.
    assert_size!(Handle<u8>, 8);
    assert_size!(Option<Handle<u8>>, 8);
    assert_size!(Handle<[u64; 16]>, 8);

    #[test]
    fn insert_get_remove() {
        let mut arena = Arena::new();
        let a = arena.insert("a");
        let b = arena.insert("b");
        assert_eq!(arena.len(), 2);
        assert_eq!(arena[a], "a");
        assert_eq!(arena.get(b), Some(&"b"));
        assert_eq!(arena.remove(a), Some("a"));
        assert_eq!(arena.remove(a), None);
        assert!(!arena.contains(a));
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn reused_slot_does_not_alias_old_handle() {
        let mut arena = Arena::new();
        let old = arena.insert(1u32);
        arena.remove(old);
        let new = arena.insert(2u32);
        assert_eq!(new.index(), old.index());
        assert_ne!(new, old);
        assert_eq!(arena.get(old), None);
        assert_eq!(arena[new], 2);
    }

    #[test]
    #[should_panic(expected = "stale or foreign")]
    fn indexing_with_a_stale_handle_panics() {
        let mut arena = Arena::new();
        let h = arena.insert(5u8);
        arena.remove(h);
        let _ = arena[h];
    }

    #[test]
    fn bits_round_trip_and_zero_generation_is_rejected() {
        let h: Handle<u8> = Handle::from_parts(7, NonZeroU32::new(3).unwrap());
        assert_eq!(Handle::from_bits(h.to_bits()), Some(h));
        assert_eq!(Handle::<u8>::from_bits(7), None);
        let typed: Handle<u16> = h.cast();
        assert_eq!(typed.index(), 7);
    }

    #[test]
    fn iteration_is_in_slot_order_and_skips_vacant() {
        let mut arena = Arena::new();
        let a = arena.insert('a');
        let b = arena.insert('b');
        let c = arena.insert('c');
        arena.remove(b);
        let seen: Vec<_> = arena.iter().map(|(h, v)| (h, *v)).collect();
        assert_eq!(seen, vec![(a, 'a'), (c, 'c')]);
        let mut sorted = vec![c, a];
        sorted.sort();
        assert_eq!(sorted, vec![a, c]);
    }

    #[test]
    fn index_mut_updates_value() {
        let mut arena = Arena::new();
        let h = arena.insert(10i32);
        arena[h] += 5;
        assert_eq!(arena[h], 15);
        *arena.get_mut(h).unwrap() -= 1;
        assert_eq!(arena[h], 14);
    }
}
