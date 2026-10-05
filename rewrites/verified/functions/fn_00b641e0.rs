// original: 0x00B641E0 veh_init_048
/// Initialise three slots of a small object: `[this]=0`, clamp call, `[this+8]=0`.
///
/// Zeroes the first dword, calls the u16 clamp setter (stubbed, thiscall/1)
/// with argument 0, zeroes the byte at +8, and returns `this`. Thiscall, no
/// stack words; entry registers except ECX are ignored.
export!(thiscall, rw_00b641e0(this: u32) -> u32 {
    unsafe {
        (this as *mut u32).write_unaligned(0);
        let _: u32 = callee_thiscall!(1, u32, this, 0);
        ((this + 8) as *mut u8).write(0);
        this
    }
});
