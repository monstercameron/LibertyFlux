// original: 0x00A8F7D0 interior_portal_has_geometry

/// Read one flag bit after resolving a pool handle through two helpers.
///
/// The signed 16-bit handle at byte offset `0x2e` selects a pointer-table
/// entry. Its page pointer is passed with `a` to the first thiscall helper;
/// that result supplies a 32-bit index at `+0x10`, which is added to `b` and
/// used to read a sign-extended 16-bit lookup value. The second helper returns
/// a record whose flag byte at `+0x4c` supplies bit 2. Thiscall consumes
/// the two stack arguments and returns the selected bit in AL.
lf_checker_rt::export!(thiscall, rw_00a8f7d0(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const HANDLE_OFFSET: u32 = 0x2e;
        const POOL_POINTER_TABLE: u32 = 0x00e9_5cd8;
        const PAGE_OFFSET: u32 = 0x70;
        const RESULT_INDEX_OFFSET: u32 = 0x10;
        const SIGNED_WORD_TABLE: u32 = 0x0125_56d8;
        const FLAG_OFFSET: u32 = 0x4c;
        const FLAG_BIT: u32 = 2;
        const FIRST_HELPER: u32 = 1;
        const SECOND_HELPER: u32 = 2;

        let handle = ((this + HANDLE_OFFSET) as *const i16).read_unaligned();
        let table_slot = lf_checker_rt::relocated(POOL_POINTER_TABLE)
            .wrapping_add((handle as i32 as u32).wrapping_mul(4));
        let pool_entry = (table_slot as *const u32).read_unaligned();
        let page = ((pool_entry + PAGE_OFFSET) as *const u32).read_unaligned();
        let first_result: u32 =
            lf_checker_rt::callee_thiscall!(FIRST_HELPER, u32, page, a);
        let base_index = ((first_result + RESULT_INDEX_OFFSET) as *const u32)
            .read_unaligned();
        let word_index = base_index.wrapping_add(b);
        let word_address = lf_checker_rt::relocated(SIGNED_WORD_TABLE)
            .wrapping_add(word_index.wrapping_mul(2));
        let lookup_value = ((word_address as *const i16).read_unaligned() as i32) as u32;
        let second_result: u32 =
            lf_checker_rt::callee_thiscall!(SECOND_HELPER, u32, page, lookup_value);
        let flags = ((second_result + FLAG_OFFSET) as *const u8).read_unaligned();
        u32::from(((flags >> FLAG_BIT) & 1) != 0)
    }
});
