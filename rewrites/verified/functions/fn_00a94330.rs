// original: 0x00a94330 stream_entry_get_location (proposed)

/// Locate a stream entry's data: 1 with outputs written, 0 when empty.
///
/// An entry is empty when its size word at `+0x08` is clear above the low
/// two bits and bit 11 of its flag word at `+0x0e` is clear; the result is
/// then 0 and nothing is written. Otherwise the data address comes from the
/// address callee into `out_data`, and `out_blocks` takes
/// `ceil((size >> 2) / 2048)`. Only `al` carries the result.
///
/// Original: thiscall, two stack arguments (out pointers).
/// One callee (thiscall, no arguments).
lf_checker_rt::export!(thiscall, rw_00a94330(ent: u32, out_data: u32, out_blocks: u32) -> u8 {
    unsafe {
        const ENT_SIZE: u32 = 0x08;
        const ENT_FLAGS: u32 = 0x0e;
        const SIZE_MASK: u32 = 0xffff_fffc;
        const PRESENT_BIT: u32 = 11;
        const ROUND_UP: u32 = 0x7ff;
        const DATA_ADDRESS: u32 = 0;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let size = rd32(ent.wrapping_add(ENT_SIZE));
        if size & SIZE_MASK == 0
            && (rd16(ent.wrapping_add(ENT_FLAGS)) >> PRESENT_BIT) & 1 == 0
        {
            return 0;
        }
        let data: u32 = lf_checker_rt::callee_thiscall!(DATA_ADDRESS, u32, ent);
        wr32(out_data, data);
        wr32(out_blocks, (size >> 2).wrapping_add(ROUND_UP) >> 11);
        1
    }
});
