// original: 0x009FF2E0 frag_node_move_to_obj (proposed)

/// Move `node` out of its current list into `obj`'s list at `+0x18`.
///
/// Unlinks `node` through the shared unlink callee, then inserts it through
/// the shared insert callee with the list head at `obj + 0x18`. Returns
/// whatever the insert callee returned.
///
/// Original: 0x009FF2E0 (thiscall, `obj` in `ecx`, `node` one stack word).
lf_checker_rt::export!(thiscall, rw_009FF2E0(obj: u32, node: u32) -> u32 {
    unsafe {
        const LIST_OFF: u32 = 0x18;
        lf_checker_rt::callee_thiscall!(1, u32, node);
        lf_checker_rt::callee_thiscall!(2, u32, obj + LIST_OFF, node)
    }
});
