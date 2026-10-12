// original: 0x00A861B0 CComplexGunTaskInfo::vf4

/// Forward this task-info record's serialized values to its stream in the original order.
///
/// `this` identifies the class-specific record and `stream` is the serializer context.
/// This wrapper reads record offsets 0x40, 0x44, 0x45, 0x46, 0x48 and delegates each value or
/// tag through the intercepted serializer calls. It has one stream argument and uses
/// thiscall; the final helper result remains in EAX.
lf_checker_rt::export!(thiscall, rw_00a861b0(this: u32, stream: u32) -> u32 {
    unsafe {
        #[inline(always)] unsafe fn rd8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)] unsafe fn rd16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)] unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        const OFF_40: u32 = 0x40;
        const OFF_44: u32 = 0x44;
        const OFF_45: u32 = 0x45;
        const OFF_46: u32 = 0x46;
        const OFF_48: u32 = 0x48;
        const LIT_00000002: u32 = 0x00000002;
        const LIT_00000003: u32 = 0x00000003;
        let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, this, stream);
        let call_result_1 = lf_checker_rt::callee_thiscall!(2, u32, stream, rd32((this).wrapping_add(OFF_40)), LIT_00000003);
        let call_result_2 = lf_checker_rt::callee_thiscall!(3, u32, stream, u32::from(rd8((this).wrapping_add(OFF_44))));
        let call_result_3 = lf_checker_rt::callee_thiscall!(3, u32, stream, u32::from(rd8((this).wrapping_add(OFF_45))));
        let call_result_4 = lf_checker_rt::callee_thiscall!(3, u32, stream, u32::from(rd8((this).wrapping_add(OFF_46))));
        let call_result_5 = lf_checker_rt::callee_thiscall!(2, u32, stream, rd32((this).wrapping_add(OFF_48)), LIT_00000002);
        call_result_5
    }
});
