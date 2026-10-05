// original: 0x009DE150 table_obj_release_forward (proposed)

/// Look up the object registered under `index` and tail-call its release routine.
///
/// A jump thunk: the shared object table maps the handle to an object
/// pointer, which becomes `this` for the target. The target only touches
/// `this` (it updates the object's word at `+0x44` and returns with no
/// stack cleanup), so the thunk is verified as a forwarding call through
/// the checker's tail-call interception: same table lookup, same target,
/// same `this`, same result.
///
/// Original: 0x009DE150 (cdecl, one stack argument, one outgoing tail call).
lf_checker_rt::export!(cdecl, rw_009DE150(index: u32) -> u32 {
    unsafe {
        const TABLE_VA: u32 = 0x01295CD8;
        const TARGET: u32 = 1;

        let obj = (lf_checker_rt::global::<u32>(TABLE_VA).wrapping_add(index as usize)
            as *const u32)
            .read_unaligned();
        lf_checker_rt::callee_thiscall!(TARGET, u32, obj)
    }
});
