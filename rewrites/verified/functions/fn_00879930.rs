// original: 0x00879930 rage::crmtComposerOptimized::vf4

/// Forward a call through the dispatch object's virtual slot `+0x10`.
///
/// The stack argument supplies the dispatch object. The original object pointer
/// arrives in ECX; the thunk places the dispatch object in ECX and forwards the
/// original object as the method's one stack argument. It tail-jumps through
/// that object's vtable and returns the selected method's EAX result.
///
/// Original calling convention: thiscall with one stack pointer argument.
lf_checker_rt::export!(thiscall, rw_00879930(this: u32, dispatch_object: u32) -> u32 {
    const VTABLE_OFFSET: u32 = 0;
    const METHOD_SLOT_OFFSET: u32 = 0x10;
    unsafe {
        let vtable = (dispatch_object.wrapping_add(VTABLE_OFFSET) as *const u32).read_unaligned();
        let target = (vtable.wrapping_add(METHOD_SLOT_OFFSET) as *const u32).read_unaligned();
        let method: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        method(dispatch_object, this)
    }
});
