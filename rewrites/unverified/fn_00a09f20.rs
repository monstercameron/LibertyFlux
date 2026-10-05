// original: 0x00a09f20 mission_cleanup_entry_init (proposed)
/// Initialise one mission-cleanup table entry to the empty state.
///
/// `entry` points at a 0x18-byte record: byte 0 is the type tag, dword 4 the
/// handle, dwords 8..0x14 payload. The empty state is tag 0, handle -1 and
/// zero payload. Thiscall; the return register is untouched by the original.
lf_checker_rt::export!(thiscall, rw_00a09f20(entry: u32) -> u32 {
    unsafe {
        const EMPTY_HANDLE: u32 = 0xffff_ffff;
        ((entry + 0x04) as *mut u32).write_unaligned(EMPTY_HANDLE);
        ((entry + 0x00) as *mut u8).write(0);
        ((entry + 0x08) as *mut u32).write_unaligned(0);
        ((entry + 0x0c) as *mut u32).write_unaligned(0);
        ((entry + 0x10) as *mut u32).write_unaligned(0);
        ((entry + 0x14) as *mut u8).write(0);
        0
    }
});
