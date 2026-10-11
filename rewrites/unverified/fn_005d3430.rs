// original: 0x005D3430 CTaskComplexCarDriveMissionFleeScene::vf13

/// Forward two stack arguments to the child's virtual slot 13.
///
/// The receiver stores its child pointer at byte offset 8. The first stack
/// argument is forwarded second; the second stack argument is a raw f32 word
/// that the original loads into XMM0 and forwards first on the callee stack.
/// Slot 13 is at byte offset 52. The selected thiscall method receives the
/// child as `this`, the f32 word in both XMM0 and stack argument zero, then the
/// first incoming stack word as stack argument one. Its EAX result is returned
/// unchanged, and it removes both outgoing stack arguments.
lf_checker_rt::export!(thiscall, rw_005d3430(this: u32, forwarded_word: u32, float_bits: u32) -> u32 {
    unsafe {
        const CHILD_POINTER: u32 = 0x08;
        const VTABLE_SLOT13: u32 = 13;
        const VTABLE_ENTRY_BYTES: u32 = 4;

        let child = ((this.wrapping_add(CHILD_POINTER)) as *const u32).read_unaligned();
        let vtable = (child as *const u32).read_unaligned();
        let method_offset = VTABLE_SLOT13 * VTABLE_ENTRY_BYTES;
        let method = ((vtable.wrapping_add(method_offset)) as *const u32).read_unaligned();
        let invoke: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(method as usize);
        invoke(child, float_bits, forwarded_word)
    }
});
