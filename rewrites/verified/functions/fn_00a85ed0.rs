// original: 0x00A85ED0 CVehicleTaskInfo::vf5

/// Write the vehicle task's base fields, then serialize the optional payload for a positive signed count.
///
/// `this` is the task-info object and `stream` is the serializer context.
/// Callback results are scripted at their original dispatch point; the
/// helper calls preserve the observed argument order and return value.
lf_checker_rt::export!(thiscall, rw_00a85ed0(this: u32, stream: u32) -> u32 {
    unsafe {
        #[inline(always)] unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        const VTABLE_SLOT: u32 = 0x50;
        let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this, stream);
        let _set_flag = lf_checker_rt::callee_thiscall!(2, u32, stream, 1u32);
        let _set_kind = lf_checker_rt::callee_thiscall!(3, u32, stream, 5u32, 1u32);
        let vtable = rd32(this);
        let entry = rd32(vtable.wrapping_add(VTABLE_SLOT));
        let callback: unsafe extern "thiscall" fn(u32) -> u32 = core::mem::transmute(entry);
        let count = callback(this);
        if (count as i32) > 0 {
            let value = callback(this);
            lf_checker_rt::callee_thiscall!(5, u32, stream, value, 1u32)
        } else {
            count
        }
    }
});
