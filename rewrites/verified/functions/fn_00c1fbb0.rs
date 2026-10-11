// original: 0x00C1FBB0 streaming_release_entry_if_present

/// Release the optional entry pointer stored at receiver offset four. A
/// non-null entry is first passed as ECX to the resource release collaborator,
/// then as the sole cdecl argument to the cleanup collaborator. The receiver's
/// pointer field is cleared on either path. The method returns no defined
/// value and uses no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c1fbb0(this: u32) -> () {
    unsafe {
        const ENTRY_POINTER_OFFSET: u32 = 4;
        let entry_address = this.wrapping_add(ENTRY_POINTER_OFFSET);
        let entry = (entry_address as *const u32).read_unaligned();
        if entry != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, (), entry);
            let _ = lf_checker_rt::callee_cdecl!(2, (), entry);
        }
        (entry_address as *mut u32).write_unaligned(0);
    }
});
