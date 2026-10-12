// original: 0x00b59040 vehicle_factory_teardown (proposed)

/// Tear down a vehicle factory object: install the table, and when the
/// global slot at 0x012e22a4 holds a non-null object, shut it down (callee
/// 1) and release it (callee 2, cdecl), then clear the slot. The null path
/// returns the incoming accumulator unchanged, so the contract fixes it to
/// 0; the taken path returns the allocator's answer.
lf_checker_rt::export!(thiscall, rw_00b59040(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00eb_0030;
        const SLOT: u32 = 0x012e_22a4;
        (this as *mut u32).write_unaligned(VTABLE);
        let slot = lf_checker_rt::global::<u32>(SLOT);
        let held = slot.read_unaligned();
        let out = if held != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, held);
            lf_checker_rt::callee_cdecl!(2, u32, held)
        } else {
            0
        };
        slot.write_unaligned(0);
        out
    }
});
