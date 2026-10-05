// original: 0x00B66040 veh_adopt_slot_18
/// Adopt `a0` into `[this+0x18]` and run the attach sequence.
///
/// Clears the slot through the drop helper (stubbed, thiscall/1), runs the
/// reset helper (stubbed, thiscall/0), stores `a0`. Returns when `a0` is null.
/// Otherwise attaches (stubbed, thiscall/1) with (a0, slot), reads the auxiliary handle from
/// `[a0+0x1bc]` when `(byte[a0+0x1e2] & 0xF) >= 2`, invokes virtual slots 0x114
/// (argument 8), 0xb0 and 0x30 (argument 0) on `a0` (planted stubs), and when
/// the auxiliary handle is non-null calls the registrar (stubbed, thiscall/5).
/// Thiscall, one stack word. No meaningful return value.
export!(thiscall, rw_00b66040(this: u32, a0: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x18;
        const AUX: u32 = 0x1bc;
        const KIND: u32 = 0x1e2;
        let slot = this + SLOT;
        let _: u32 = callee_thiscall!(1, u32, this, slot);
        let _: u32 = callee_thiscall!(2, u32, this);
        (slot as *mut u32).write_unaligned(a0);
        if a0 == 0 {
            return 0;
        }
        let _: u32 = callee_thiscall!(3, u32, a0, slot);
        let obj = (slot as *const u32).read_unaligned();
        let aux = if (((obj + KIND) as *const u8).read() & 0xF) < 2 {
            0
        } else {
            ((obj + AUX) as *const u32).read_unaligned()
        };
        let vt = (obj as *const u32).read_unaligned();
        let f1: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((vt + 0x114) as *const u32).read_unaligned()) as usize);
        let _ = f1(obj, 8);
        let vt = (obj as *const u32).read_unaligned();
        let f2: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((vt + 0xb0) as *const u32).read_unaligned()) as usize);
        let _ = f2(obj);
        let vt = (obj as *const u32).read_unaligned();
        let f3: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((vt + 0x30) as *const u32).read_unaligned()) as usize);
        let _ = f3(obj, 0);
        if aux == 0 {
            return 0;
        }
        let obj = (slot as *const u32).read_unaligned();
        let _: u32 = callee_thiscall!(7, u32, obj, aux, 0, 0x102, 0, 0);
        0
    }
});
