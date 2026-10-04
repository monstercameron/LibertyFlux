// original: 0x00e5db50 net_list_prepend_02
/// Prepend this module's static node to the shared registration list.
///
/// The node's next slot takes the previous head, the head takes the node,
/// and the previous head is returned.
lf_checker_rt::export!(cdecl, rw_00e5db50() -> u32 {
    unsafe {
        let head = lf_checker_rt::global::<u32>(0x017ad16c);
        let prev = *head;
        *lf_checker_rt::global::<u32>(0x0110e6f4) = prev;
        *head = lf_checker_rt::relocated(0x0110e6e4);
        prev
    }
});
