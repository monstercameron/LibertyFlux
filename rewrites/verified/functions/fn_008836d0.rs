// original: 0x008836d0 stream_alloc_double (proposed)
/// Allocate a slot with two auxiliary objects, rolling back on any failure.
///
/// Takes a fresh slot from the pool allocator (intercepted callee 1,
/// cdecl, no arguments); a null slot fails. Takes two auxiliary objects
/// (intercepted callee 2, cdecl, no arguments, called twice); when either
/// is null the slot is released (intercepted callee 3, cdecl, one argument)
/// and each non-null auxiliary is released too (intercepted callee 4,
/// cdecl, one argument), and the function fails. On success links the slot
/// (`[slot+0x00] = key`, `[slot+0x04] = owner`, `[slot+0x08] = first`,
/// `[slot+0x0c] = second`), folds the owner's tag at `owner+0x450` into the
/// slot's low kind nibble (`[slot+0x10] ^= (tag ^ [slot+0x10]) & 0xf`),
/// clears the live flag (bit 0 of `[slot+0x14]`), and returns the slot.
///
/// Original: cdecl, two stack arguments (`key`, `owner`), returns the slot
/// (or null) in `eax`.
lf_checker_rt::export!(cdecl, rw_008836d0(key: u32, owner: u32) -> u32 {
    unsafe {
        const OWNER_TAG: u32 = 0x450;
        const KIND_WORD: u32 = 0x10;
        const KIND_MASK: u32 = 0x0f;
        const FLAG_BYTE: u32 = 0x14;
        const LIVE_FLAG: u8 = 1;
        const SLOT_CALLEE: u32 = 1;
        const AUX_CALLEE: u32 = 2;
        const SLOT_FREE_CALLEE: u32 = 3;
        const AUX_FREE_CALLEE: u32 = 4;
        let slot: u32 = lf_checker_rt::callee_cdecl!(SLOT_CALLEE, u32,);
        if slot == 0 {
            return 0;
        }
        let first: u32 = lf_checker_rt::callee_cdecl!(AUX_CALLEE, u32,);
        let second: u32 = lf_checker_rt::callee_cdecl!(AUX_CALLEE, u32,);
        if first == 0 || second == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(SLOT_FREE_CALLEE, u32, slot);
            if first != 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(AUX_FREE_CALLEE, u32, first);
            }
            if second != 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(AUX_FREE_CALLEE, u32, second);
            }
            return 0;
        }
        let tag = ((owner + OWNER_TAG) as *const u32).read_unaligned();
        let kind = ((slot + KIND_WORD) as *const u32).read_unaligned();
        (slot as *mut u32).write_unaligned(key);
        let fold = (tag ^ kind) & KIND_MASK;
        ((slot + KIND_WORD) as *mut u32).write_unaligned(kind ^ fold);
        let flags = (slot + FLAG_BYTE) as *mut u8;
        flags.write(flags.read() & !LIVE_FLAG);
        ((slot + 4) as *mut u32).write_unaligned(owner);
        ((slot + 8) as *mut u32).write_unaligned(first);
        ((slot + 0x0c) as *mut u32).write_unaligned(second);
        slot
    }
});
