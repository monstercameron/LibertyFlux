// original: 0x00A84780 CTargetTaskInfo::append_target_info

/// Read the target pointer at +0x14 and its name pointer at +0x6C. If a name pointer exists, pass it through the target-name helper and log the returned string. Otherwise sign-extend the 16-bit fallback id at +0x18 and log it. The thiscall method takes the logger context as its single stack argument.
lf_checker_rt::export!(thiscall, rw_00a84780(this: u32, logger_context: u32) -> () {
    const TARGET_POINTER_OFFSET: u32 = 0x14;
    const TARGET_NAME_OFFSET: u32 = 0x6C;
    const FALLBACK_ID_OFFSET: u32 = 0x18;
    const FORMAT_TARGET_NAME: u32 = 0x00EA06E0;
    const FORMAT_TARGET_ID: u32 = 0x00EA06EC;
    #[inline(always)]
    fn read_u16(base: u32, offset: u32) -> u16 {
        unsafe { core::ptr::read_unaligned(base.wrapping_add(offset) as *const u16) }
    }
    #[inline(always)]
    fn read_u32(base: u32, offset: u32) -> u32 {
        unsafe { core::ptr::read_unaligned(base.wrapping_add(offset) as *const u32) }
    }
    let target = read_u32(this, TARGET_POINTER_OFFSET);
    if target != 0 {
        let target_name = read_u32(target, TARGET_NAME_OFFSET);
        if target_name != 0 {
            let name = lf_checker_rt::callee_thiscall!(1, u32, target_name);
            let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_TARGET_NAME), name);
            return;
        }
    }
    let signed_target_id = read_u16(this, FALLBACK_ID_OFFSET) as i16 as i32 as u32;
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_TARGET_ID), signed_target_id);
});
