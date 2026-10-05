// original: 0x00B64340 veh_vtable_drop_14
/// Drop the reference at `[this+0x14]` through a virtual release call.
///
/// Returns after clearing the slot when it is null. Otherwise calls the detach
/// helper (stubbed, stdcall/1) with the slot address, re-reads the object and
/// invokes its virtual slot 0x114 (planted stub, thiscall/1) with argument 8,
/// then zeroes the slot. Thiscall, no stack words. No meaningful return.
export!(thiscall, rw_00b64340(this: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x14;
        const RELEASE: u32 = 0x114;
        let slot = this + SLOT;
        if (slot as *const u32).read_unaligned() == 0 {
            (slot as *mut u32).write_unaligned(0);
            return 0;
        }
        let _: u32 = callee_stdcall!(1, u32, slot);
        let obj = (slot as *const u32).read_unaligned();
        let vt = (obj as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((vt + RELEASE) as *const u32).read_unaligned()) as usize);
        let _ = f(obj, 8);
        (slot as *mut u32).write_unaligned(0);
        0
    }
});
