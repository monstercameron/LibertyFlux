// original: 0x009546E0 id_table_flag (proposed)

/// Look up one flag byte for a 16-bit id.
///
/// Returns 0 when the low 16 bits of `id` are above `MAX_ID` (UNSIGNED
/// 16-bit comparison). Otherwise sign-extends the index (as the
/// original's `cwde`, a no-op for in-range ids) and returns the byte at
/// `TABLE + index * ENTRY_STRIDE`. Pure read. Original is cdecl/1,
/// returns AL.
lf_checker_rt::export!(cdecl, rw_009546E0(id: u32) -> u32 {
    const MAX_ID: u16 = 0x5DB;
    const TABLE: u32 = 0x011F7114;
    const ENTRY_STRIDE: u32 = 8;
    let idx = id as u16;
    if idx > MAX_ID {
        return 0;
    }
    let sext = (idx as i16) as i32 as u32;
    let addr = lf_checker_rt::relocated(TABLE).wrapping_add(sext.wrapping_mul(ENTRY_STRIDE));
    unsafe { (addr as *const u8).read() as u32 }
});
