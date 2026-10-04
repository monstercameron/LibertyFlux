// original: 0x009e9600 ped_state_in_window
/// True when the state word at `[this+0x21C]+0x12C` lies in
/// 3..=14. (thiscall; low byte is the value.)
lf_checker_rt::export!(thiscall, rw_009e9600(this_ptr: u32) -> u32 {
    unsafe {
        const LINK_OFF: u32 = 0x21C;
        const STATE_OFF: u32 = 0x12C;
        const LO: u32 = 3;
        const HI: u32 = 0xE;
        let linked = (this_ptr.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        let v = (linked.wrapping_add(STATE_OFF) as *const u32).read_unaligned();
        if v >= LO && v <= HI { 1 } else { 0 }
    }
});
