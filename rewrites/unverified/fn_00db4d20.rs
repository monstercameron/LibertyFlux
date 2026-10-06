// original: 0x00DB4D20 uimouse_verify_child

/// Verify a cursor child object against the shared name.
///
/// `this` is the cursor object. The key at `KEY` selects the child: a null
/// key answers false at once. Otherwise the tree lookup helper (scripted)
/// resolves the key against a global tree, slot 0 of the returned node's
/// virtual table is called with the node, and the shared-name helper
/// (scripted) is asked with its constant. The result is true when both
/// answers agree. The low byte of the return is compared (`al`: the
/// original sets only `al`).
///
/// Original: 0x00DB4D20 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00db4d20(this: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x200;
        const GLOBAL_TREE: u32 = 0x01981a4c;
        const NAME_REF: u32 = 0x00ef2974;
        const LOOKUP: u32 = 1;
        const SLOT_FN: u32 = 2;
        const SHARED: u32 = 3;

        let key = (this + KEY) as *const u32;
        let key = key.read_unaligned();
        if key == 0 {
            return 0;
        }
        let node: u32 = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            lf_checker_rt::relocated(GLOBAL_TREE),
            key
        );
        let vtable = (node as *const u32).read_unaligned();
        let slot: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((vtable as *const u32).read_unaligned() as usize);
        let value = slot(node);
        let shared: u32 =
            lf_checker_rt::callee_cdecl!(SHARED, u32, lf_checker_rt::relocated(NAME_REF));
        u32::from(value == shared)
    }
});
