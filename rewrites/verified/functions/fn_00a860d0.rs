// original: 0x00A860D0 CComplexJumpTaskInfo::vf4

/// Forward this task-info record's serialized values to its stream in the original order.
///
/// `this` identifies the class-specific record and `stream` is the serializer context.
/// This wrapper reads record offsets 0x18, 0x19, 0x1A and delegates each value or
/// tag through the intercepted serializer calls. It has one stream argument and uses
/// thiscall; the final helper result remains in EAX.
lf_checker_rt::export!(thiscall, rw_00a860d0(this: u32, stream: u32) -> u32 {
    unsafe {
        #[inline(always)] unsafe fn rd8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)] unsafe fn rd16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)] unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        const OFF_18: u32 = 0x18;
        const OFF_19: u32 = 0x19;
        const OFF_1A: u32 = 0x1A;
        let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, this, stream);
        let call_result_1 = lf_checker_rt::callee_thiscall!(2, u32, stream, u32::from(rd8((this).wrapping_add(OFF_18))));
        let call_result_2 = lf_checker_rt::callee_thiscall!(2, u32, stream, u32::from(rd8((this).wrapping_add(OFF_19))));
        let call_result_3 = lf_checker_rt::callee_thiscall!(2, u32, stream, u32::from(rd8((this).wrapping_add(OFF_1A))));
        call_result_3
    }
});
