// original: 0x008ddef0 CShaderFx_PopForcedTechnique::vf1

/// Pop the forced-technique stack and apply the revealed technique.
///
/// The stack pointer for the current context (`CTX_INDEX` global) is read
/// from `STACK_TOPS`, the technique recorded below it in `STACK_VALUES` is
/// passed to the applier (callee 1), and the top is decremented. Thiscall,
/// no stack arguments, one outgoing call.
lf_checker_rt::export!(thiscall, rw_008ddef0(this: u32) -> u32 {
    unsafe {
        const CTX_INDEX: u32 = 0x011d_6f0c;
        const STACK_TOPS: u32 = 0x011d_4e98;
        const STACK_VALUES: u32 = 0x011d_4e78;
        const CALLEE_APPLY: u32 = 1;
        let _ = this;
        let idx = lf_checker_rt::global::<u32>(CTX_INDEX).read_unaligned();
        let tops = lf_checker_rt::relocated(STACK_TOPS);
        let top = ((tops + idx.wrapping_mul(4)) as *const u32).read_unaligned();
        let values = lf_checker_rt::relocated(STACK_VALUES);
        let slot = top.wrapping_add(idx.wrapping_mul(2));
        let technique =
            ((values + slot.wrapping_mul(4)) as *const u32).read_unaligned();
        ((tops + idx.wrapping_mul(4)) as *mut u32)
            .write_unaligned(top.wrapping_sub(1));
        lf_checker_rt::callee_cdecl!(CALLEE_APPLY, u32, technique);
        0
    }
});
