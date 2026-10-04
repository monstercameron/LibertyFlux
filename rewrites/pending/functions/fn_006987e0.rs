// original: 0x006987e0 rage::crCreatureComponentSkeleton::vf7
/// Forward a skeleton request with a frame-built argument pair.
///
/// Builds the two-word struct (first argument, joint handle) on the stack
/// and passes its address in ECX with (handle, second argument, handle) on
/// the stack. Returns the callee answer. The struct address itself is
/// codegen-dependent and excluded from the call comparison.
export!(thiscall, rw_006987e0(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        let inner = *((this as *const u32).add(3));
        let handle = *((inner as *const u32).add(1));
        let pair = [arg1, handle];
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        f(&pair as *const u32 as u32, handle, arg2, handle)
    }
});
