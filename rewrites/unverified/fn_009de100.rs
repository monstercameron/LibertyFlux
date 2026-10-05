// original: 0x009DE100 shared_table_field44 (proposed)

/// Return the word at `+0x44` of the object registered under `index`.
///
/// The shared object table maps a small integer handle to an object pointer;
/// this getter reads one field out of the looked-up object.
///
/// Original: 0x009DE100 (cdecl, one stack argument, no outgoing calls).
lf_checker_rt::export!(cdecl, rw_009DE100(index: u32) -> u32 {
    unsafe {
        const TABLE_VA: u32 = 0x01295CD8;
        const FIELD_OFF: u32 = 0x44;

        let obj = (lf_checker_rt::global::<u32>(TABLE_VA).wrapping_add(index as usize)
            as *const u32)
            .read_unaligned();
        ((obj as *const u8).wrapping_add(FIELD_OFF) as *const u32).read_unaligned()
    }
});
