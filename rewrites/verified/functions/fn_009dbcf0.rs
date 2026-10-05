// original: 0x009DBCF0 table_obj_release_and_unregister (proposed)

/// Release a handle's object through its vtable slot 9, then unregister it.
///
/// A null payload returns 0 at once. Otherwise the object registered under
/// `index` gets its slot-9 method called with the payload, the index is
/// unregistered from the shared table, and 1 is returned (in the low byte;
/// the upper bytes are whatever the unregister call returned).
///
/// Original: 0x009DBCF0 (cdecl, two stack arguments, two outgoing calls).
lf_checker_rt::export!(cdecl, rw_009DBCF0(index: u32, payload: u32) -> u32 {
    unsafe {
        const TABLE_VA: u32 = 0x01295CD8;
        const REGISTRY_VA: u32 = 0x016DCEB8;
        const UNREGISTER: u32 = 2;

        if payload == 0 {
            return 0;
        }
        let obj = (lf_checker_rt::global::<u32>(TABLE_VA).wrapping_add(index as usize)
            as *const u32)
            .read_unaligned();
        let vtable = (obj as *const u32).read_unaligned();
        let release: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            (((vtable as *const u32).wrapping_add(9)).read_unaligned()) as usize,
        );
        release(obj, payload);
        let ans: u32 = lf_checker_rt::callee_thiscall!(
            UNREGISTER, u32, lf_checker_rt::relocated(REGISTRY_VA), index
        );
        (ans & 0xFFFF_FF00) | 1
    }
});
