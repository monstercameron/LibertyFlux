// original: 0x00A85B80 CScenarioTaskInfo::vf5

/// Forward the scenario task's callback result to its stream with the fixed task tag.
///
/// `this` is the task-info object and `stream` is the serializer context.
/// Callback results are scripted at their original dispatch point; the
/// helper calls preserve the observed argument order and return value.
lf_checker_rt::export!(thiscall, rw_00a85b80(this: u32, stream: u32) -> u32 {
    unsafe {
        #[inline(always)] unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        const VTABLE_SLOT: u32 = 0x20;
        let vtable = rd32(this);
        let entry = rd32(vtable.wrapping_add(VTABLE_SLOT));
        let callback: unsafe extern "thiscall" fn(u32) -> u32 = core::mem::transmute(entry);
        let callback_result = callback(this);
        lf_checker_rt::callee_thiscall!(2, u32, stream, callback_result, 1u32)
    }
});
