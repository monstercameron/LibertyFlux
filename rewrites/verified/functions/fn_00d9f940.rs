// original: 0x00D9F940 stream_key_missing
/// Uses the state helper to select one of two counted key lists and returns zero when the requested key appears in the selected list, or one when it does not.
lf_checker_rt::export!(thiscall, rw_s675_00d9f940(this: u32, requested_key: u32) -> u32 {
    let mode: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
    let (table_offset, count_offset) = if mode == 1 { (0x30_u32, 0x34_u32) } else { (0x14_u32, 0x18_u32) };
    let count = unsafe { (this.wrapping_add(count_offset) as *const u16).read_unaligned() } as u32;
    let entries = unsafe { (this.wrapping_add(table_offset) as *const u32).read_unaligned() };
    for index in 0..count {
        let entry = unsafe { (entries.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned() };
        if entry == requested_key {
            return 0;
        }
    }
    1
});
