// original: 0x00A8B210 find_entry_by_name_prefix

/// Scan the pool's fixed-stride records with the name-comparison helper.
/// Return the first zero-result row index; return `0xff` when the signed row
/// count is nonpositive or every row produces a nonzero result.
///
/// `this` points to the pool, whose signed count is at byte offset `0x9c4` and
/// whose records begin at `0x54` with a stride of `0x64` bytes. `name` is
/// passed unchanged to the helper. The helper is cdecl with three stack args.
lf_checker_rt::export!(thiscall, rw_00a8b210(this: u32, name: u32) -> u32 {
    unsafe {
        const CALLEE_COMPARE_NAME: u32 = 1;
        const RECORDS: u32 = 0x54;
        const RECORD_STRIDE: u32 = 0x64;
        const SIGNED_COUNT: u32 = 0x9c4;
        const COMPARE_MODE: u32 = 3;

        let count = ((this + SIGNED_COUNT) as *const i32).read_unaligned();
        if count <= 0 {
            return 0xff;
        }

        for index in 0..count {
            let record = this
                .wrapping_add(RECORDS)
                .wrapping_add((index as u32).wrapping_mul(RECORD_STRIDE));
            let equal = lf_checker_rt::callee_cdecl!(
                CALLEE_COMPARE_NAME,
                u32,
                name,
                record,
                COMPARE_MODE
            );
            if equal == 0 {
                return index as u32;
            }
        }
        0xff
    }
});
