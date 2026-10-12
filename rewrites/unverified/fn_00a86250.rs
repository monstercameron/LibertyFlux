// original: 0x00A86250 CComplexNewUseCoverInfo::vf4

/// Forward this task-info record's serialized values to its stream in the original order.
///
/// `this` identifies the class-specific record and `stream` is the serializer context.
/// This wrapper reads record offsets 0x18, 0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21 and delegates each value or
/// tag through the intercepted serializer calls. It has one stream argument and uses
/// thiscall; the final helper result remains in EAX.
lf_checker_rt::export!(thiscall, rw_00a86250(this: u32, stream: u32) -> u32 {
    unsafe {
        #[inline(always)] unsafe fn rd8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)] unsafe fn rd16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)] unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        const OFF_18: u32 = 0x18;
        const OFF_1C: u32 = 0x1C;
        const OFF_1D: u32 = 0x1D;
        const OFF_1E: u32 = 0x1E;
        const OFF_1F: u32 = 0x1F;
        const OFF_20: u32 = 0x20;
        const OFF_21: u32 = 0x21;
        const LIT_00000004: u32 = 0x00000004;
        const LIT_00000007: u32 = 0x00000007;
        let call_result_0 = lf_checker_rt::callee_thiscall!(1, u32, this, stream);
        let call_result_1 = lf_checker_rt::callee_thiscall!(2, u32, (stream).wrapping_add(LIT_00000004), rd32((this).wrapping_add(OFF_18)), LIT_00000007);
        let call_result_2 = lf_checker_rt::callee_thiscall!(3, u32, (stream).wrapping_add(LIT_00000004), u32::from(rd8((this).wrapping_add(OFF_1C))));
        let call_result_3 = lf_checker_rt::callee_thiscall!(3, u32, (stream).wrapping_add(LIT_00000004), u32::from(rd8((this).wrapping_add(OFF_1D))));
        let call_result_4 = lf_checker_rt::callee_thiscall!(3, u32, (stream).wrapping_add(LIT_00000004), u32::from(rd8((this).wrapping_add(OFF_1E))));
        let call_result_5 = lf_checker_rt::callee_thiscall!(3, u32, (stream).wrapping_add(LIT_00000004), u32::from(rd8((this).wrapping_add(OFF_1F))));
        let call_result_6 = lf_checker_rt::callee_thiscall!(3, u32, (stream).wrapping_add(LIT_00000004), u32::from(rd8((this).wrapping_add(OFF_20))));
        let call_result_7 = lf_checker_rt::callee_thiscall!(3, u32, (stream).wrapping_add(LIT_00000004), u32::from(rd8((this).wrapping_add(OFF_21))));
        call_result_7
    }
});
