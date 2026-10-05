// original: 0x00879030 rage::crmtComposerOptimized::vf33

/// Dispatch one pending node through the composer's node virtual.
///
/// `this` points to a composer object. The node's handler (virtual slot
/// `+0x80` of the composer's own vtable) runs with `node` as its argument,
/// then the flag word at `done` is cleared and 1 is returned in AL.
///
/// Original: 0x00879030 (thiscall, two stack arguments; low byte of the
/// return only, the upper bytes keep the helper's answer).
lf_checker_rt::export!(thiscall, rw_00879030(this: u32, node: u32, done: u32) -> u32 {
    unsafe {
        const HANDLER_VT_SLOT: u32 = 0x80;
        let vtable = (this as *const u32).read_unaligned();
        let handler: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
            core::mem::transmute(
                ((vtable.wrapping_add(HANDLER_VT_SLOT)) as *const u32).read_unaligned()
                    as usize,
            )
        };
        handler(this, node);
        (done as *mut u32).write_unaligned(0);
        1
    }
});
