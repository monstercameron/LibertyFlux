// original: 0x009e98a0 ped_mode_special
/// True when the mode word at `[this+0x21C]+0x12C` is 2 and the
/// sign-extended word at `+0x2E` differs from the shared id global.
/// (thiscall; low byte is the value.)
lf_checker_rt::export!(thiscall, rw_009e98a0(this_ptr: u32) -> u32 {
    unsafe {
        const LINK_OFF: u32 = 0x21C;
        const MODE_OFF: u32 = 0x12C;
        const TAG_OFF: u32 = 0x2E;
        const WANT_MODE: u32 = 2;
        const SHARED_ID: u32 = 0x12FA650;
        let linked = (this_ptr.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        if (linked.wrapping_add(MODE_OFF) as *const u32).read_unaligned() != WANT_MODE {
            return 0;
        }
        let tag = (this_ptr.wrapping_add(TAG_OFF) as *const i16).read_unaligned() as i32 as u32;
        if tag == lf_checker_rt::global::<u32>(SHARED_ID).read_unaligned() {
            0
        } else {
            1
        }
    }
});
