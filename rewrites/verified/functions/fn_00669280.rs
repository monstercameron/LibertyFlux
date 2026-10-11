// original: 0x00669280 rage::ptxEventEffect::vf13

/// Forward one adjusted record pointer to the object stored at offset `0x110`
/// in the second stack argument. The x86 `thiscall` receiver is unused. When
/// that field is zero, return the owner pointer already in EAX; otherwise return the
/// callee's EAX unchanged. The helper call is intercepted by the checker.
const VF13_TARGET_POINTER_OFFSET: u32 = 0x110;
const VF13_RECORD_DATA_OFFSET: u32 = 0x00a0;
const VF13_TARGET_CALL_ID: u32 = 1;

lf_checker_rt::export!(thiscall, rw_00669280(_receiver: u32, record: u32, owner: u32) -> u32 {
    let target = unsafe {
        (owner.wrapping_add(VF13_TARGET_POINTER_OFFSET) as *const u32).read_unaligned()
    };
    if target == 0 {
        return owner;
    }
    let data = record.wrapping_add(VF13_RECORD_DATA_OFFSET);
    unsafe { lf_checker_rt::callee_thiscall!(VF13_TARGET_CALL_ID, u32, target, data) }
});
