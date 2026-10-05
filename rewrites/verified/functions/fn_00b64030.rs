// original: 0x00B64030 veh_init_full
/// Initialise a vehicle sub-object: handle, capped limit, two resets, tag.
///
/// Stores `a0` at `[this+0x18]`; calls the limit setter (stubbed, thiscall/1)
/// with `min(a1 & 0xFFFF, 0x7FFF)`; calls two reset routines (stubbed, each
/// thiscall/1) with 0; zeroes `[this+0x1c]` and `[this+0x20]`, stores the low
/// byte of `a2` at `[this+0x24]`; calls the tag routine (stubbed, thiscall/1)
/// with `a0` and returns its answer opaquely. Thiscall, three stack words.
export!(thiscall, rw_00b64030(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const HANDLE: u32 = 0x18;
        const LIMIT: u32 = 0x7FFF;
        ((this + HANDLE) as *mut u32).write_unaligned(a0);
        let lo = a1 & 0xFFFF;
        let capped = if LIMIT < lo { LIMIT } else { lo };
        let _: u32 = callee_thiscall!(1, u32, this, capped);
        let _: u32 = callee_thiscall!(2, u32, this, 0);
        let _: u32 = callee_thiscall!(3, u32, this, 0);
        ((this + 0x1c) as *mut u32).write_unaligned(0);
        ((this + 0x20) as *mut u32).write_unaligned(0);
        ((this + 0x24) as *mut u8).write((a2 & 0xFF) as u8);
        callee_thiscall!(4, u32, this, a0)
    }
});
