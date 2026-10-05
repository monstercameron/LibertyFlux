// original: 0x00B65CB0 veh_guarded_forward
/// Forward two arguments under a re-entrancy flag at `[this+0x109]`.
///
/// Sets the flag byte to 1, calls the guarded routine (stubbed, thiscall/3)
/// with `(a0, a1, 0)`, resets the flag to 0. Thiscall, two stack words; entry
/// registers except ECX are ignored. No meaningful return value.
export!(thiscall, rw_00b65cb0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x109;
        ((this + FLAG) as *mut u8).write(1);
        let r: u32 = callee_thiscall!(1, u32, this, a0, a1, 0);
        ((this + FLAG) as *mut u8).write(0);
        r
    }
});
