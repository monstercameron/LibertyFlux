// original: 0x00B35320 partition_28B_key12_desc (proposed)

/// Partition a run of 28-byte records about a pivot key, largest first.
///
/// `first`/`last` bound the run (`last` exclusive). The seven words after
/// them are the pivot record passed by value; only its float key at pivot
/// word 3 (record offset 12) is read. The final word is unread.
///
/// Records compare by the float at record offset 12. The low cursor advances
/// while a record's key is above the pivot and stops at the first key at or
/// below it (NaN stops it too); the high cursor retreats while a record's
/// key is below the pivot and stops at the first key at or above it. The
/// two records are exchanged and scanning resumes until the cursors meet;
/// the meeting point is returned. Empty and inverted runs still scan one
/// record from each end before the cursors are compared.
///
/// Original: 0x00B35320 (cdecl, ten stack words), no callees, no globals.
lf_checker_rt::export!(cdecl, rw_00b35320(first: u32, last: u32, _p0: u32, _p1: u32, _p2: u32, piv_key: u32, _p4: u32, _p5: u32, _p6: u32, _extra: u32) -> u32 {
    unsafe { partition(first, last, f32::from_bits(piv_key), 12, true) }
});
