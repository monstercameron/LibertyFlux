// original: 0x009DBCA0 table_obj_guarded_release (proposed)

/// Conditionally detach, release and unregister a handle's object.
///
/// A null payload returns 0 at once. When the payload's word at `+0xC` is
/// non-zero a detach call runs first with the index and a global tag. Then
/// the object registered under `index` gets its vtable slot-6 method called
/// with the payload, the index is unregistered, and 1 is returned (in the
/// low byte; the upper bytes are whatever the unregister call returned).
///
/// Original: 0x009DBCA0 (cdecl, two stack arguments, three outgoing calls).
lf_checker_rt::export!(cdecl, rw_009DBCA0(index: u32, payload: u32) -> u32 {
    unsafe {
        const TABLE_VA: u32 = 0x01295CD8;
        const TAG_VA: u32 = 0x012B4138;
        const REGISTRY_VA: u32 = 0x016DCEB8;
        const DETACH: u32 = 1;
        const UNREGISTER: u32 = 3;

        if payload == 0 {
            return 0;
        }
        let obj = (lf_checker_rt::global::<u32>(TABLE_VA).wrapping_add(index as usize)
            as *const u32)
            .read_unaligned();
        let gate = ((payload as *const u8).wrapping_add(0x0C) as *const u32).read_unaligned();
        if gate != 0 {
            let tag = (lf_checker_rt::global::<u32>(TAG_VA) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_cdecl!(DETACH, u32, index, tag);
        }
        let vtable = (obj as *const u32).read_unaligned();
        let release: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            (((vtable as *const u32).wrapping_add(6)).read_unaligned()) as usize,
        );
        release(obj, payload);
        let ans: u32 = lf_checker_rt::callee_thiscall!(
            UNREGISTER, u32, lf_checker_rt::relocated(REGISTRY_VA), index
        );
        (ans & 0xFFFF_FF00) | 1
    }
});
