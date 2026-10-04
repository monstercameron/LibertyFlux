// original: 0x00875340 add_dtor
/// Destroy an additive blend motion request.
///
/// Installs the additive-blend vtable, releases the child objects in slots
/// `0x198` and `0x194` through their vtables (slot 2) when non-null, clears
/// both slots, runs the source reset helper (stubbed), and tail-calls the
/// base teardown routine (stubbed), returning its answer.
export!(thiscall, rw_00875340(this: u32) -> u32 {
    unsafe {
        const SLOT_HI: usize = 0x198 / 4;
        const SLOT_LO: usize = 0x194 / 4;
        let base = this as *mut u32;
        base.write(relocated(0xFE813C));
        let hi = base.add(SLOT_HI).read();
        if hi != 0 {
            let vt = (hi as *const u32).read();
            let target = (vt as *const u32).add(2).read();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let _: u32 = release(hi);
        }
        let lo = base.add(SLOT_LO).read();
        base.add(SLOT_HI).write(0);
        if lo != 0 {
            let vt = (lo as *const u32).read();
            let target = (vt as *const u32).add(2).read();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let _: u32 = release(lo);
        }
        base.add(SLOT_LO).write(0);
        let _: u32 = callee_thiscall!(3, u32, this);
        callee_thiscall!(4, u32, this)
    }
});
