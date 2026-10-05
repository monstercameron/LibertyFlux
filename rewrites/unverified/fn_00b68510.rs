// original: 0x00B68510 veh_heli_aux_0
/// helicopter auxiliary update (channel 0): validate, register, reset flags.
///
/// Returns at once when `[this+0xdc4]` is null, when byte `[this+0x1ef0]`
/// exceeds 0x7F (signed), or when the validator (stubbed, thiscall/1) rejects
/// the sign-extended byte. Otherwise registers (stubbed, thiscall/2) with
/// `(this, 0)` on the shared pool object (address passed through opaquely),
/// runs the applier (stubbed, thiscall/1) and the channel reset (stubbed,
/// thiscall/0) on `this+0x210`, then zeroes `[this+0x1ef4]` and `[this+0x1ed4]`
/// and clears bit 3 of `[this+0xf14]`. Thiscall, no stack words.
export!(thiscall, rw_00b68510(this: u32) -> u32 {
    unsafe {
        const HANDLE: u32 = 0xdc4;
        const SEL: u32 = 0x1ef0;
        const POOL: u32 = 0x16D9F58;
        const CHAN: u32 = 0x210;
        const OUT0: u32 = 0x1ef4;
        const OUT1: u32 = 0x1ed4;
        const FLAGS: u32 = 0xf14;
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
        let _: u32 = callee_thiscall!(2, u32, relocated(POOL), this, 0);
        let _: u32 = callee_thiscall!(3, u32, stale, (sel as i8) as i32 as u32);
        ((this + OUT0) as *mut u32).write_unaligned(0);
        let _: u32 = callee_thiscall!(4, u32, this + CHAN);
        let f = ((this + FLAGS) as *const u8).read();
        ((this + FLAGS) as *mut u8).write(f & 0xf7);
        ((this + OUT1) as *mut u32).write_unaligned(0);
        0
    }
});
