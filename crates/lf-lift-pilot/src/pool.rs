//! Fixed-stride pools (lifted from the 0x009DCxxx family).
//!
//! Original shape: a 12-byte header `{count, zero, base}`; the allocator is
//! asked for `count * stride + 16` bytes (saturating at `u32::MAX`); on
//! success the block head stores the count, the element base follows 16
//! bytes in, and every element is stamped (vtable pointer plus status
//! words) or constructed through a callback.
//!
//! The lift keeps the size computation (including the saturating clamp)
//! and the per-element initial states, and replaces the block with a `Vec`.
//! Vtable stamps become the [`PoolKind`] tag: the stamp's only meaning is
//! which element type the pool holds. Pointer-valued returns (end pointer,
//! block pointer) have no lifted meaning; success versus failure plus the
//! element vector carries the behaviour.

/// Which element type a pool holds (one per original stride site).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PoolKind {
    /// Stride 0x60.
    S60,
    /// Stride 0x70.
    S70,
    /// Stride 0x80, first site.
    S80a,
    /// Stride 0x160.
    S160,
    /// Stride 0x70 with the wide element init.
    S70wide,
    /// Stride 0x3D0 with per-element construction.
    S3d0,
    /// Stride 0x80, second site.
    S80b,
}

impl PoolKind {
    /// Element stride in bytes, as in the original's size computation.
    #[must_use]
    pub fn stride(self) -> u32 {
        match self {
            Self::S60 => 0x60,
            Self::S70 | Self::S70wide => 0x70,
            Self::S80a | Self::S80b => 0x80,
            Self::S160 => 0x160,
            Self::S3d0 => 0x3D0,
        }
    }
}

/// One pooled element in its initial state.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PoolElement {
    /// Element type (the original's vtable stamp).
    pub kind: PoolKind,
    /// Status word (the original's zero word at +8).
    pub status: u32,
    /// Wide-init extra words at +0x60/+0x64 (only [`PoolKind::S70wide`).
    pub wide_extra: Option<(u32, u32)>,
}

/// An initialised pool: the header's count plus the stamped elements.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pool {
    /// Element count, from the header.
    pub count: u32,
    /// Elements in initial states.
    pub elements: Vec<PoolElement>,
}

/// The original's allocation-size computation: `count * stride + 16`,
/// saturating at `u32::MAX`. Kept verbatim so the clamp stays proven.
#[must_use]
pub fn checked_total(count: u32, stride: u32) -> u32 {
    let total = (count as u64) * (stride as u64) + 0x10;
    if total > u32::MAX as u64 { u32::MAX } else { total as u32 }
}

/// Initialise a simply-stamped pool. The allocator hook receives the exact
/// byte count the original requests and answers success or failure; on
/// failure the lift returns `None`, matching the original's null stores.
/// (Originals: the stride 0x60/0x70/0x80/0x160 sites and the wide site.)
pub fn init_simple(
    count: u32,
    kind: PoolKind,
    alloc: &mut dyn FnMut(u32) -> bool,
) -> Option<Pool> {
    if !alloc(checked_total(count, kind.stride())) {
        return None;
    }
    let wide_extra = match kind {
        PoolKind::S70wide => Some((0, 0xFFFF_FFFF)),
        _ => None,
    };
    let mut elements = Vec::new();
    // The original loops `i < count` with a wrapping increment; a test-only
    // guard keeps hostile counts from hanging the host.
    let mut i = 0u32;
    while i < count {
        elements.push(PoolElement { kind, status: 0, wide_extra });
        i = i.wrapping_add(1);
        debug_assert!(elements.len() < (1 << 30));
    }
    Some(Pool { count, elements })
}

/// Initialise a constructed pool: allocate, then run the constructor over
/// every element, returning the last construction result. An empty pool
/// runs no constructions and has no last result (the original returns the
/// block address there, which has no lifted meaning).
/// (Original: the stride 0x3D0 site.)
pub fn init_constructed(
    count: u32,
    alloc: &mut dyn FnMut(u32) -> bool,
    construct: &mut dyn FnMut(usize) -> u32,
) -> (Option<Pool>, Option<u32>) {
    if !alloc(checked_total(count, PoolKind::S3d0.stride())) {
        return (None, None);
    }
    let mut elements = Vec::new();
    let mut last: Option<u32> = None;
    let mut i = 0u32;
    while i < count {
        last = Some(construct(elements.len()));
        elements.push(PoolElement { kind: PoolKind::S3d0, status: 0, wide_extra: None });
        i = i.wrapping_add(1);
    }
    (Some(Pool { count, elements }), last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_clamp_matches_original() {
        assert_eq!(checked_total(10, 0x60), 10 * 0x60 + 0x10);
        assert_eq!(checked_total(0, 0x70), 0x10);
        assert_eq!(checked_total(u32::MAX, 0x3D0), u32::MAX);
        assert_eq!(checked_total(u32::MAX, 1), u32::MAX);
    }

    #[test]
    fn simple_init_states() {
        let mut ok = |_: u32| true;
        let pool = init_simple(3, PoolKind::S60, &mut ok).expect("alloc ok");
        assert_eq!(pool.count, 3);
        assert_eq!(pool.elements.len(), 3);
        assert!(pool.elements.iter().all(|e| e.status == 0 && e.wide_extra.is_none()));
        let wide = init_simple(2, PoolKind::S70wide, &mut ok).expect("alloc ok");
        assert!(wide.elements.iter().all(|e| e.wide_extra == Some((0, 0xFFFF_FFFF))));
        let mut fail = |_: u32| false;
        assert!(init_simple(3, PoolKind::S60, &mut fail).is_none());
    }

    #[test]
    fn constructed_init_last_result() {
        let mut ok = |_: u32| true;
        let mut seen = Vec::new();
        let (pool, last) = init_constructed(3, &mut ok, &mut |i| {
            seen.push(i);
            (i as u32) + 100
        });
        assert_eq!(seen, vec![0, 1, 2]);
        assert_eq!(last, Some(102));
        assert_eq!(pool.expect("alloc ok").elements.len(), 3);
        let (pool0, last0) = init_constructed(0, &mut ok, &mut |_| 1);
        assert!(pool0.is_some());
        assert_eq!(last0, None);
    }
}
