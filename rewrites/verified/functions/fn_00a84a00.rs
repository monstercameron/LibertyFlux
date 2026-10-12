// original: 0x00A84A00 CComplexFallAndGetUpTaskInfo::deserialize

/// Deserializes the base task-info state, then reads one typed value from the
/// stream into the derived object's 32-bit slot at byte offset 0x18. Both
/// helper calls are preserved in order through checker stand-ins. The
/// thiscall entry receives the object in ECX and consumes one stream pointer
/// from the stack. The last helper's 32-bit EAX result passes through.
lf_checker_rt::export!(thiscall, rw_00A84A00(task_info: u32, stream: u32) -> u32 {
    const BASE_DESERIALIZE_CALLEE: u32 = 1;
    const READ_TYPED_VALUE_CALLEE: u32 = 2;
    const DERIVED_VALUE_OFFSET: u32 = 0x18;
    const DERIVED_VALUE_TYPE: u32 = 2;

    let _ = lf_checker_rt::callee_thiscall!(
        BASE_DESERIALIZE_CALLEE,
        u32,
        task_info,
        stream,
    );
    let read_result = lf_checker_rt::callee_thiscall!(
        READ_TYPED_VALUE_CALLEE,
        u32,
        stream,
        task_info.wrapping_add(DERIVED_VALUE_OFFSET),
        DERIVED_VALUE_TYPE,
    );
    read_result
});
