// original: 0x00b55bc0 resolve_then_find_node
/// Resolve a key pair through the helper, then find its node in the list.
///
/// The helper turns (a, b) into a single key which is looked up with the
/// +0x44-field finder (fn_00b55be0) on the same object; that finder's answer
/// is returned.
export!(thiscall, rw_00b55bc0(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        let key: u32 = callee_cdecl!(1, u32, a, b);
        callee_thiscall!(2, u32, this, key)
    }
});
