// original: 0x00a122c0 null_guarded_thiscall (proposed)
/// Notify the target object unless it is null.
///
/// Does nothing when `target` is null. Otherwise calls the notify callee on
/// `target` with (`value`, 0). No return value is set: the exit register
/// holds the callee's result on the taken path and the entry value on the
/// null path, so the contract does not compare it. Cdecl, two arguments.
export!(cdecl, rw_00a122c0(value: u32, target: u32) -> u32 {
    unsafe {
        const NOTIFY: u32 = 1;
        if target == 0 {
            0
        } else {
            callee_thiscall!(NOTIFY, u32, target, value, 0)
        }
    }
});
