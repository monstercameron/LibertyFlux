// original: 0x005D3420 CTaskComplexCarDriveMissionFleeScene::vf8

/// Forward one argument to the child task's virtual slot 8.
///
/// The child pointer is stored at `this + 8`. For a live child, the function
/// loads the 32-bit vtable pointer, dispatches slot 8 at byte offset `0x20`
/// with the child as `this`, and returns that method's EAX result. The original
/// tail-jumps to the selected thiscall method; this rewrite makes the
/// equivalent intercepted call. If the child is null, the original leaves EAX
/// unchanged. This proof fixes incoming EAX to zero, so returning zero matches
/// that branch; nonzero EAX carry-through is not covered. The one stack
/// argument is removed by the target method.
lf_checker_rt::export!(thiscall, rw_005d3420(this: u32, argument: u32) -> u32 {
    unsafe {
        const CHILD_POINTER: u32 = 0x08;
        const VTABLE_SLOT8: u32 = 8;
        const VTABLE_ENTRY_BYTES: u32 = 4;

        let child = ((this.wrapping_add(CHILD_POINTER)) as *const u32).read_unaligned();
        if child == 0 {
            return 0;
        }

        let vtable = (child as *const u32).read_unaligned();
        let method_offset = VTABLE_SLOT8 * VTABLE_ENTRY_BYTES;
        let method = ((vtable.wrapping_add(method_offset)) as *const u32).read_unaligned();
        let invoke: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(method as usize);
        invoke(child, argument)
    }
});
