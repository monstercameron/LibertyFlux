// original: 0x00A85E80 CComplexOpenVehicleDoorTaskInfo::vf5

/// Run the base serializer, then serialize the optional door value only when its callback requests it.
///
/// `this` is the task-info object and `stream` is the serializer context.
/// Callback results are scripted at their original dispatch point; the
/// helper calls preserve the observed argument order and return value.
lf_checker_rt::export!(thiscall, rw_00a85e80(this: u32, stream: u32) -> u32 {
    unsafe {
        #[inline(always)] unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        const VTABLE_SLOT: u32 = 0x44;
        let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this, stream);
        let vtable = rd32(this);
        let entry = rd32(vtable.wrapping_add(VTABLE_SLOT));
        let callback: unsafe extern "thiscall" fn(u32) -> u32 = core::mem::transmute(entry);
        let should_write = callback(this);
        if should_write != 0 {
            let value = callback(this);
            lf_checker_rt::callee_thiscall!(3, u32, stream, value, 1u32)
        } else {
            0
        }
    }
});
