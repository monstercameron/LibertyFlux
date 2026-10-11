// original: 0x005D3450 CTaskComplexCarDriveMissionFleeScene::vf7

/// Dispatch to the child's virtual slot 7 with a zero argument.
///
/// The receiver stores its child pointer at byte offset 8. A null child
/// returns zero. Otherwise the function loads the child's vtable, selects slot
/// 7 at byte offset 28, and tail-dispatches with the child as `this` and a
/// stack argument forced to zero. The selected thiscall method removes that
/// argument; its EAX result is returned unchanged. The incoming stack word is
/// deliberately ignored by the original.
lf_checker_rt::export!(thiscall, rw_005d3450(this: u32, incoming_word: u32) -> u32 {
    unsafe {
        const CHILD_POINTER: u32 = 0x08;
        const VTABLE_SLOT7: u32 = 7;
        const VTABLE_ENTRY_BYTES: u32 = 4;
        let _ = incoming_word;

        let child = ((this.wrapping_add(CHILD_POINTER)) as *const u32).read_unaligned();
        if child == 0 {
            return 0;
        }

        let vtable = (child as *const u32).read_unaligned();
        let method_offset = VTABLE_SLOT7 * VTABLE_ENTRY_BYTES;
        let method = ((vtable.wrapping_add(method_offset)) as *const u32).read_unaligned();
        let invoke: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(method as usize);
        invoke(child, 0)
    }
});
