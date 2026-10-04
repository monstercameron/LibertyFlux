// original: 0x00945fb0 init_slot_pair
/// Run the slot worker over both slots of a dual-slot object.
///
/// Two counted calls with the slot pointers, returning the second call's
/// answer (what the original leaves in `eax`).
lf_checker_rt::export!(thiscall, rw_00945fb0(this_ptr: u32) -> u32 {
    unsafe {
        let mut r: u32 = 0;
        let mut s = this_ptr;
        let mut i = 0;
        while i < 2 {
            r = lf_checker_rt::callee_thiscall!(1, u32, s);
            s = s.wrapping_add(0xbd0);
            i += 1;
        }
        r
    }
});
