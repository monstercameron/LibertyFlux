// original: 0x00B68580 veh_heli_aux_1
/// helicopter auxiliary update (channel 1): validate, register, reset output.
///
/// Returns at once when `[this+0xdc4]` is null, when byte `[this+0x1ef1]`
/// exceeds 0x7F (signed), or when the validator (stubbed, thiscall/1) rejects
/// the sign-extended byte. Otherwise registers (stubbed, thiscall/2) with
/// `(this, 1)` on the shared pool object (address passed through opaquely),
/// runs the channel reset (stubbed, thiscall/0) on `this+0x210` and the
/// applier (stubbed, thiscall/1), then zeroes `[this+0x1ef8]`. Thiscall, no
/// stack words. No meaningful return value.
export!(thiscall, rw_00b68580(this: u32) -> u32 {
    unsafe {
        const HANDLE: u32 = 0xdc4;
        const SEL: u32 = 0x1ef1;
        const POOL: u32 = 0x16D9F58;
        const CHAN: u32 = 0x210;
        const OUT: u32 = 0x1ef8;
        if ((this + HANDLE) as *const u32).read_unaligned() == 0 {
            return 0;
        }
        let sel = ((this + SEL) as *const u8).read();
        if sel > 0x7F {
            return 0;
        }
        let stale = ((this + HANDLE) as *const u32).read_unaligned();
        let ok: u32 = callee_thiscall!(1, u32, stale, (sel as i8) as i32 as u32);
        if (ok & 0xFF) != 0 {
            return 0;
        }
        let _: u32 = callee_thiscall!(2, u32, relocated(POOL), this, 1);
        let _: u32 = callee_thiscall!(4, u32, this + CHAN);
        let _: u32 = callee_thiscall!(3, u32, stale, (sel as i8) as i32 as u32);
        ((this + OUT) as *mut u32).write_unaligned(0);
        0
    }
});
