// original: 0x008f8410 input_list_clear_tail (proposed)

/// Clear the element list, then tail-dispatch to the teardown callee.
///
/// `obj` is the input device object. The function calls the clear callee
/// (thiscall, ECX = `obj`, no stack words) and then tail-jumps to the
/// teardown callee with ECX = `obj + 0x38`, so the teardown answer is the
/// return value and no return address of this function remains. The stack
/// is balanced at the jump (one push, one pop), which the checker's tail
/// patching requires. The rewrite performs the same two calls in order.
///
/// Thiscall: object in ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f8410(obj: u32) -> u32 {
    const C_CLEAR: u32 = 1;
    const C_TEARDOWN: u32 = 2;
    const SUB_OFF: u32 = 0x38;
    let _: u32 = lf_checker_rt::callee_thiscall!(C_CLEAR, u32, obj);
    lf_checker_rt::callee_thiscall!(C_TEARDOWN, u32, obj.wrapping_add(SUB_OFF))
});
