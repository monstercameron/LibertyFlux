// original: 0x00A8F8F0 interior_mark_frag_dirty

/// Pass the sign-extended 16-bit pool entry field at byte offset `0x52` to
/// the dirty-marking helper and return its result. The signed handle at `+0x2e`
/// selects the pool entry from the pointer table. The function has no stack
/// arguments and uses thiscall; the helper's one cdecl argument is caller-cleaned.
lf_checker_rt::export!(thiscall, rw_00a8f8f0(this: u32) -> u32 {
    unsafe {
        const HANDLE_OFFSET: u32 = 0x2e;
        const POOL_POINTER_TABLE: u32 = 0x0129_5cd8;
        const DIRTY_FIELD_OFFSET: u32 = 0x52;
        const DIRTY_HELPER: u32 = 1;

        let handle = ((this + HANDLE_OFFSET) as *const i16).read_unaligned();
        let table_slot = lf_checker_rt::relocated(POOL_POINTER_TABLE)
            .wrapping_add((handle as i32 as u32).wrapping_mul(4));
        let entry = (table_slot as *const u32).read_unaligned();
        let dirty_code = ((entry + DIRTY_FIELD_OFFSET) as *const i16).read_unaligned();
        lf_checker_rt::callee_cdecl!(DIRTY_HELPER, u32, dirty_code as i32 as u32)
    }
});
