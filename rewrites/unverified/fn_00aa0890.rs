// original: 0x00AA0890 set_loader_entry_flags_11
/// Sets the selected loader entry's flags by OR-ing the fixed mask
/// 0x11 into the dword at offset 0x254, then returns the
/// entry pointer. The single pointer argument is stack-passed.
lf_checker_rt::export!(stdcall, rw_00AA0890(entry: u32) -> u32 {
    unsafe {
        let flags = (entry.wrapping_add(0x254) as *const u32).read_unaligned();
        (entry.wrapping_add(0x254) as *mut u32).write_unaligned(flags | 0x11);
        entry
    }
});
