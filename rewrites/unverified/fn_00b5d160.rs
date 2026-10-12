// original: 0x00b5d160 clear_state_1_or_3 (proposed)

/// If the state word at `+0x1c` is 1 or 3, reset it to 0. The stack
/// argument is not read. Returns the state word as found.
lf_checker_rt::export!(thiscall, rw_00b5d160(this: u32, _arg: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x1c;
        let p = this.wrapping_add(STATE) as *mut u32;
        let v = p.read_unaligned();
        if v == 1 || v == 3 {
            p.write_unaligned(0);
        }
        v
    }
});
