// original: 0x00BCCFA0 physics_request_object_processing
/// Resolve a physics object, clamp a signed 32-bit limit to 0xffff, and pass
/// a mode plus the limit to a helper on the object's subobject at byte offset
/// 0x10d0. Negative inputs, including 0x80000000 and 0xffffffff, stay
/// unchanged; positive inputs above 0xffff clamp to 0xffff. The cdecl inputs
/// are a context handle, the limit word, a mode flag, and a byte flag that
/// does not change the computed mode. A nonzero low byte of the mode flag
/// selects mode 3, otherwise mode 0. A null resolved object returns zero;
/// otherwise the helper's EAX result is returned. The contract covers values
/// below, at, and above the signed clamp boundary and checks AL.
lf_checker_rt::export!(
    cdecl,
    rw_00BCCFA0(
        context_handle: u32,
        limit_input: u32,
        mode_flag: u32,
        unused_flag_word: u32,
    ) -> u32 {
        const PHYSICS_MANAGER_SLOT: u32 = 0x012e_22a4;
        const MAX_LIMIT: u32 = 0xffff;
        const REQUEST_SUBOBJECT_OFFSET: u32 = 0x10d0;
        const BYTE_MASK: u32 = 0xff;
        let manager = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<u32>(PHYSICS_MANAGER_SLOT))
        };
        let object = lf_checker_rt::callee_thiscall!(1, u32, manager, context_handle);
        let signed_limit = limit_input as i32;
        let clamped_limit = if signed_limit > MAX_LIMIT as i32 {
            MAX_LIMIT
        } else {
            limit_input
        };
        let mode = if mode_flag & BYTE_MASK != 0 { 3 } else { 0 };
        let _ignored_flag = unused_flag_word & BYTE_MASK;
        if object == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            2,
            u32,
            object.wrapping_add(REQUEST_SUBOBJECT_OFFSET),
            mode,
            clamped_limit,
        )
    }
);
