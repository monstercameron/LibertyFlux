// original: 0x006341E0 frag_parse_bonetag_all

/// Ask the input object's virtual slot at `+0x14` to prepare its current
/// record, then insert the record key into the tree at `this + 0xD4`. The
/// first helper is called with the input's `+4` object and one word argument.
/// The insertion helper receives pointers to a local key cell and a second
/// local word, and its boolean result is returned in AL. The original leaves
/// that second local uninitialized; the proof fixes it to zero and records
/// that narrowed input explicitly.
lf_checker_rt::export!(thiscall, rw_006341E0(this: u32, input: u32) -> u32 {
    const PREPARE_RECORD: u32 = 1;
    const INSERT_RECORD: u32 = 2;
    const INPUT_OBJECT: u32 = 4;
    const TREE: u32 = 0xD4;

    unsafe {
        let interface = (input.wrapping_add(INPUT_OBJECT) as *const u32).read_unaligned();
        let _prepared = lf_checker_rt::callee_thiscall!(PREPARE_RECORD, u32, interface, 1u32);

        let record_key = (input as *const u32).read_unaligned();
        let mut key_cell = record_key;
        let mut unused_local = 0u32;
        let key_cell_pointer = core::ptr::addr_of_mut!(key_cell) as u32;
        let unused_local_pointer = core::ptr::addr_of_mut!(unused_local) as u32;
        let inserted = lf_checker_rt::callee_thiscall!(
            INSERT_RECORD,
            u8,
            this.wrapping_add(TREE),
            key_cell_pointer,
            unused_local_pointer,
        );
        u32::from(inserted)
    }
});
