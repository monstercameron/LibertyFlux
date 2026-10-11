// original: 0x00CD9D20 CTaskSimpleBlendFromNM::CTaskSimpleBlendFromNM

/// Initialize one `CTaskSimpleBlendFromNM` constructor variant after its
/// base and common NM-task initializer calls. The five incoming stack words
/// are stored at offsets 0x24, 0x2C, 0x28, 0x38 and 0x20 respectively. If the
/// second or third argument equals `u32::MAX`, byte 0x3E is set to one; when
/// neither matches, that byte is left as it was after the scripted initializer
/// call.
///
/// Calling convention: thiscall with five 32-bit stack words. Both
/// constructor helpers are thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00cd9d20(this: u32, kind: u32, secondary_id: u32, owner_id: u32, duration: u32, blend: u32) -> u32 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const COMMON_INITIALIZER: u32 = 2;
    const KIND_FIELD: u32 = 0x24;
    const SECONDARY_ID_FIELD: u32 = 0x2C;
    const OWNER_ID_FIELD: u32 = 0x28;
    const DURATION_FIELD: u32 = 0x38;
    const BLEND_FIELD: u32 = 0x20;
    const SENTINEL_FLAG: u32 = 0x3E;
    const SENTINEL: u32 = u32::MAX;
    const VTABLE: u32 = 0x00ED_D68C;

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let _ = lf_checker_rt::callee_thiscall!(COMMON_INITIALIZER, u32, this);
        ((this + KIND_FIELD) as *mut u32).write_unaligned(kind);
        ((this + SECONDARY_ID_FIELD) as *mut u32).write_unaligned(secondary_id);
        ((this + OWNER_ID_FIELD) as *mut u32).write_unaligned(owner_id);
        ((this + DURATION_FIELD) as *mut u32).write_unaligned(duration);
        ((this + BLEND_FIELD) as *mut u32).write_unaligned(blend);
        if secondary_id == SENTINEL || owner_id == SENTINEL {
            ((this + SENTINEL_FLAG) as *mut u8).write(1);
        }
        this
    }
});
