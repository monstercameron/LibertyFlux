// original: 0x00A753D0 clear_owned_task_fields

/// Releases the optional object stored at `this + 0x30`, clears that pointer,
/// then clears the word at `this + 0x38`.
///
/// The cleanup helper receives the stored object in ECX and the address of
/// the pointer field as its stack argument. Its return value is not semantic.
lf_checker_rt::export!(thiscall, rw_00a753d0(this: u32) -> u32 {
    unsafe {
        const RELEASE_FIELD: u32 = 1;
        const OWNED_OFFSET: u32 = 0x30;
        const STATE_OFFSET: u32 = 0x38;

        let owned = ((this + OWNED_OFFSET) as *const u32).read_unaligned();
        if owned != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_FIELD, u32, owned, this + OWNED_OFFSET);
            ((this + OWNED_OFFSET) as *mut u32).write_unaligned(0);
        }
        ((this + STATE_OFFSET) as *mut u32).write_unaligned(0);
        0
    }
});
