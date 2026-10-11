// original: 0x009BB200 CCamScriptInstruction_SetViewportShape::vf2

/// Set the shape word on the viewport matching this instruction's key.
///
/// The key is read from `this + 0x08` and the new shape from `this + 0x0C`.
/// A global pointer/count pair describes the viewport list. The first object
/// whose identifier at `+0x53C` matches receives the shape word at `+0x554`;
/// an empty list or missing key is a no-op. The original is a void thiscall
/// method and leaves EAX as incidental loop state.
lf_checker_rt::export!(thiscall, rw_009bb200(this: u32) -> () {
    unsafe {
        const VIEWPORTS_VA: u32 = 0x0118_d7f0;
        const VIEWPORT_COUNT_VA: u32 = 0x0118_d7f4;
        const VIEWPORT_ID: u32 = 0x53c;
        const VIEWPORT_SHAPE: u32 = 0x554;

        let wanted = (this.wrapping_add(8) as *const u32).read_unaligned();
        let shape = (this.wrapping_add(0x0c) as *const u32).read_unaligned();
        let list = lf_checker_rt::global::<u32>(VIEWPORTS_VA).read_unaligned();
        let count = (lf_checker_rt::global::<u16>(VIEWPORT_COUNT_VA)).read_unaligned();
        for index in 0..u32::from(count) {
            let viewport = ((list.wrapping_add(index * 4)) as *const u32).read_unaligned();
            let identifier = (viewport.wrapping_add(VIEWPORT_ID) as *const u32).read_unaligned();
            if identifier == wanted {
                (viewport.wrapping_add(VIEWPORT_SHAPE) as *mut u32).write_unaligned(shape);
                break;
            }
        }
    }
});
