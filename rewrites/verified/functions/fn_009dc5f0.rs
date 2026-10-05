// original: 0x009DC5F0 shared_table_guarded_lookup (proposed)

/// Look up the object registered under `index` and read through its links.
///
/// When the object's word at `+8` is non-null it points at a descriptor and
/// the word at `+0xB4` of that descriptor is returned. Otherwise the
/// object's word at `+0xC` is tried: a null there returns 0, a non-null one
/// is dereferenced and returned.
///
/// Original: 0x009DC5F0 (cdecl, one stack argument, no outgoing calls).
lf_checker_rt::export!(cdecl, rw_009DC5F0(index: u32) -> u32 {
    unsafe {
        const TABLE_VA: u32 = 0x01295CD8;
        const LINK_OFF: usize = 0x08;
        const FALLBACK_OFF: usize = 0x0C;
        const DESCRIPTOR_OFF: usize = 0xB4;

        let obj = (lf_checker_rt::global::<u32>(TABLE_VA).wrapping_add(index as usize)
            as *const u32)
            .read_unaligned();
        let link = ((obj as *const u8).wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        if link != 0 {
            return ((link as *const u8).wrapping_add(DESCRIPTOR_OFF) as *const u32)
                .read_unaligned();
        }
        let fallback =
            ((obj as *const u8).wrapping_add(FALLBACK_OFF) as *const u32).read_unaligned();
        if fallback == 0 {
            return 0;
        }
        (fallback as *const u32).read_unaligned()
    }
});
