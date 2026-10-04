// original: 0x009e88e0 ped_sub_float_field
/// Float at `+0x144` of the sub-object from virtual slot `+0xD4`,
/// returned on the x87 stack. (thiscall.)
lf_checker_rt::export!(thiscall, rw_009e88e0(this_ptr: u32) -> f32 {
    unsafe {
        const SLOT: u32 = 0xD4;
        const FIELD_OFF: u32 = 0x144;
        let vt = (this_ptr as *const u32).read_unaligned();
        let slot = (vt.wrapping_add(SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        let sub = f(this_ptr);
        f32::from_bits((sub.wrapping_add(FIELD_OFF) as *const u32).read_unaligned())
    }
});
