// original: 0x008ddf20 CShaderFx_PushForcedTechnique::vf1

/// Push the current technique and force this object's one.
///
/// Reads the current technique (callee 1), records it on the technique
/// stack for the current context (`CTX_INDEX` global into `STACK_TOPS`,
/// value into `STACK_VALUES`, top incremented), then applies the technique
/// stored at `this + 8` (callee 2). Thiscall, no stack arguments, two
/// outgoing calls.
lf_checker_rt::export!(thiscall, rw_008ddf20(this: u32) -> u32 {
    unsafe {
        const CTX_INDEX: u32 = 0x011d_6f0c;
        const STACK_TOPS: u32 = 0x011d_4e98;
        const STACK_VALUES: u32 = 0x011d_4e78;
        const FORCED_OFF: u32 = 8;
        const CALLEE_CURRENT: u32 = 1;
        const CALLEE_APPLY: u32 = 2;
        let current = lf_checker_rt::callee_cdecl!(CALLEE_CURRENT, u32,);
        let idx = lf_checker_rt::global::<u32>(CTX_INDEX).read_unaligned();
        let tops = lf_checker_rt::relocated(STACK_TOPS);
        let slot = (tops + idx.wrapping_mul(4)) as *mut u32;
        let top = slot.read_unaligned().wrapping_add(1);
        slot.write_unaligned(top);
        let values = lf_checker_rt::relocated(STACK_VALUES);
        let at = top.wrapping_add(idx.wrapping_mul(2));
        ((values + at.wrapping_mul(4)) as *mut u32).write_unaligned(current);
        let forced = ((this + FORCED_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(CALLEE_APPLY, u32, forced);
        0
    }
});
