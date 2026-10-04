// original: 0x009e7490 ped_sub_byte_field
/// Byte at `+0x142` of the sub-object from virtual slot `+0xD4`.
/// (thiscall; low byte is the value.)
lf_checker_rt::export!(thiscall, rw_009e7490(this_ptr: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0xD4;
        const FIELD_OFF: u32 = 0x142;
        let vt = (this_ptr as *const u32).read_unaligned();
        let slot = (vt.wrapping_add(SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        let sub = f(this_ptr);
        (sub.wrapping_add(FIELD_OFF) as *const u8).read() as u32
    }
});
