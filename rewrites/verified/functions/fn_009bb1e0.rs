// original: 0x009BB1E0 CCamScriptInstruction_SetViewportPriority::vf2

/// Set the priority word on the viewport named by this instruction.
///
/// The viewport key is at `this + 0x08` and the priority is at `this + 0x0C`.
/// The handler asks the camera manager for the viewport, writes the priority
/// at `+0x544`, and returns the lookup result in EAX. A null lookup faults on
/// the field write, matching the original.
lf_checker_rt::export!(thiscall, rw_009bb1e0(this: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x0128_e400;
        const PRIORITY: u32 = 0x544;
        const LOOKUP: u32 = 1;
        let key = (this.wrapping_add(8) as *const u32).read_unaligned();
        let viewport = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            lf_checker_rt::relocated(MANAGER),
            key,
        );
        let value = (this.wrapping_add(0x0c) as *const u32).read_unaligned();
        (viewport.wrapping_add(PRIORITY) as *mut u32).write_unaligned(value);
        viewport
    }
});
