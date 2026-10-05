// original: 0x00883650 stream_alloc_single (proposed)
/// Allocate a slot with one auxiliary object, rolling back on any failure.
///
/// Takes a fresh slot from the pool allocator (intercepted callee 1,
/// cdecl, no arguments); a null slot fails. Takes one auxiliary object
/// (intercepted callee 2, cdecl, no arguments); a null auxiliary releases
/// the slot (intercepted callee 3, cdecl, one argument) and fails. When the
/// owner's tag at `owner+0x450` exceeds 1, takes a second auxiliary object
/// into the high half; a null second auxiliary releases both the slot and
/// the first auxiliary (intercepted callee 4, cdecl, one argument) and
/// fails. On success links the slot (`[slot+0x00] = key`, `[slot+0x04] =
/// owner`, `[slot+0x08] = first`, `[slot+0x0c] = second-or-null`), folds the
/// owner's tag into the slot's low kind nibble (`[slot+0x10] ^= (tag ^
/// [slot+0x10]) & 0xf`), sets the live flag (bit 0 of `[slot+0x14]`), and
/// returns the slot.
///
/// Original: cdecl, two stack arguments (`key`, `owner`), returns the slot
/// (or null) in `eax`.
lf_checker_rt::export!(cdecl, rw_00883650(key: u32, owner: u32) -> u32 {
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
        if first == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(SLOT_FREE_CALLEE, u32, slot);
            return 0;
        }
        let tag = ((owner + OWNER_TAG) as *const u32).read_unaligned();
        let mut second = 0u32;
        if tag > 1 {
            second = lf_checker_rt::callee_cdecl!(AUX_CALLEE, u32,);
            if second == 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(SLOT_FREE_CALLEE, u32, slot);
                let _: u32 = lf_checker_rt::callee_cdecl!(AUX_FREE_CALLEE, u32, first);
                return 0;
            }
        }
        let kind = ((slot + KIND_WORD) as *const u32).read_unaligned();
        let fold = (tag ^ kind) & KIND_MASK;
        ((slot + KIND_WORD) as *mut u32).write_unaligned(kind ^ fold);
        ((slot + 4) as *mut u32).write_unaligned(owner);
        let flags = (slot + FLAG_BYTE) as *mut u8;
        flags.write(flags.read() | LIVE_FLAG);
        (slot as *mut u32).write_unaligned(key);
        ((slot + 8) as *mut u32).write_unaligned(first);
        ((slot + 0x0c) as *mut u32).write_unaligned(second);
        slot
    }
});
