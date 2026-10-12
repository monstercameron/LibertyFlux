// original: 0x00CF21E0 playstat_derive_name_id

/// Copy a bounded stack-derived name window into a record and store its ID.
///
/// The thiscall inputs are the record pointer and two stack words. The first
/// word is stored at record offset `0x30` and returned in EAX. A 32-byte copy
/// helper receives the destination at `this+0x10`, a pointer to the caller's
/// second argument slot, and the length 32. The proof snapshots that source
/// window at the call, including the defined stack-fill words after it. The
/// wrapper removes both stack arguments.
lf_checker_rt::export!(thiscall, rw_fn_00cf21e0(this: u32, name_id: u32, source_word: u32) -> u32 {
    const NAME_FIELD: usize = 0x10;
    const NAME_ID_FIELD: usize = 0x30;
    const COPY_BYTES: u32 = 0x20;
    const COPY_CALLEE: u32 = 1;
    let source_window = [source_word, 0, 0, 0, 0, 0, 0, 0];
    let source = source_window.as_ptr() as usize as u32;
    let destination = this.wrapping_add(NAME_FIELD as u32);
    let _ = lf_checker_rt::callee_cdecl!(COPY_CALLEE, u32, destination, source, COPY_BYTES);
    unsafe {
            (this.wrapping_add(NAME_ID_FIELD as u32) as *mut u32).write_unaligned(name_id);
    }
    name_id
});
