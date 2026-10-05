// original: 0x00B35450 partition_28B_key0_asc (proposed)

/// Partition a run of 28-byte records about a pivot key, smallest first.
///
/// Same layout as rw_00b35320 except the key is the float at record offset 0
/// (pivot word 0) and the order is ascending: the low cursor advances while a
/// record's key is below the pivot, the high cursor retreats while a record's
/// key is above it, stopping (NaN included) and exchanging as there.
///
/// Original: 0x00B35450 (cdecl, ten stack words), no callees, no globals.
lf_checker_rt::export!(cdecl, rw_00b35450(first: u32, last: u32, piv_key: u32, _p1: u32, _p2: u32, _p3: u32, _p4: u32, _p5: u32, _p6: u32, _extra: u32) -> u32 {
    unsafe { partition(first, last, f32::from_bits(piv_key), 0, false) }
});
