// original: 0x009DE110 registry_find_or_alloc (proposed)

/// Find the object for a key, allocating and flagging one on a miss.
///
/// The lookup writes its out-word to a dead stack slot (address skipped,
/// value scripted and returned). On a hit that word is the result. On a
/// miss a fresh pool slot is allocated for the key, a registry fixup runs,
/// the slot's flag word at `+0x40` gains bit 0x800000, and the global index
/// minus one is returned.
///
/// Original: 0x009DE110 (cdecl, one stack argument, three outgoing calls).
lf_checker_rt::export!(cdecl, rw_009DE110(key: u32) -> u32 {
    unsafe {
        const INDEX_VA: u32 = 0x012B415C;
        const FLAG_OFF: usize = 0x40;
        const FLAG_BIT: u32 = 0x800000;
        const LOOKUP: u32 = 1;
        const ALLOC: u32 = 2;
        const FIXUP: u32 = 3;

        let mut slot = 0u32;
        let found: u32 = lf_checker_rt::callee_cdecl!(
            LOOKUP, u32, &mut slot as *mut u32 as u32, key
        );
        if found != 0 {
            return slot;
        }
        let obj: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, key);
        let _: u32 = lf_checker_rt::callee_cdecl!(FIXUP, u32, );
        let flags =
            ((obj as *const u8).wrapping_add(FLAG_OFF) as *const u32).read_unaligned();
        ((obj as *mut u8).wrapping_add(FLAG_OFF) as *mut u32)
            .write_unaligned(flags | FLAG_BIT);
        let index = (lf_checker_rt::global::<u32>(INDEX_VA) as *const u32).read_unaligned();
        index.wrapping_sub(1)
    }
});
