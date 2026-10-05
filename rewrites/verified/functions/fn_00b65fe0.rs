// original: 0x00B65FE0 veh_swap_slot_e8
/// Swap the reference at `[this+0xe8]` for `a0`, notifying on change.
///
/// Detaches the old reference (stubbed, stdcall/1) when non-null, stores `a0`,
/// attaches it (stubbed, thiscall/1) when non-null. When the value changed,
/// clears `[this+0x100]`; when it changed to a non-null object, also invokes
/// the object's virtual slot 0x9c (planted stub, thiscall/1) with `this+0xf0`.
/// Thiscall, one stack word. No meaningful return value.
export!(thiscall, rw_00b65fe0(this: u32, a0: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0xe8;
        const FLAG: u32 = 0x100;
        const OUT: u32 = 0xf0;
        const NOTIFY: u32 = 0x9c;
        let slot = this + SLOT;
        let old = (slot as *const u32).read_unaligned();
        let changed = a0 != old;
        if old != 0 {
            let _: u32 = callee_stdcall!(1, u32, slot);
        }
        (slot as *mut u32).write_unaligned(a0);
        if a0 != 0 {
            let _: u32 = callee_thiscall!(2, u32, a0, slot);
        }
        if !changed {
            return 0;
        }
        ((this + FLAG) as *mut u32).write_unaligned(0);
        if a0 == 0 {
            return 0;
        }
        let vt = (a0 as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((vt + NOTIFY) as *const u32).read_unaligned()) as usize);
        let _ = f(a0, this + OUT);
        0
    }
});
