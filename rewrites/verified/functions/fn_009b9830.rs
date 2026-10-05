// original: 0x009B9830 CCamScriptInstruction_InheritRoll::vf2

/// Execute the InheritRoll script instruction: look the cam up by the
/// index at `this+0x08` (callee 1); if found, query its kind through the
/// virtual slot at table+0x28 (callee 2, reached by the same
/// load-and-call through the fabricated object on both sides); if the
/// kind is 0x0e (roll-capable), forward the operand at `this+0x0c` to the
/// roll setter (callee 3).
///
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9830(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0128_E400;
        const FIELD_INDEX: u32 = 0x08;
        const FIELD_VALUE: u32 = 0x0c;
        const VTABLE_SLOT_KIND: u32 = 0x28;
        const KIND_ROLL: u32 = 0x0e;
        let index = ((this + FIELD_INDEX) as *const u32).read_unaligned();
        let cam: u32 = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(MGR), index);
        if cam != 0 {
            let table = (cam as *const u32).read_unaligned();
            let slot = (table + VTABLE_SLOT_KIND) as *const u32;
            let query: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot.read_unaligned() as usize);
            if query(cam) == KIND_ROLL {
                let value =
                    ((this + FIELD_VALUE) as *const u32).read_unaligned();
                let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, cam, value);
            }
        }
        0
    }
});
