// original: 0x009DD660 guarded_obj_use_unregister (proposed)

/// Use a handle's object under a guard, then unregister the handle.
///
/// A guard call runs first, then the object's signed key word at `+0x48` is
/// validated. A search call runs with a relocated tag, two constant
/// parameters, a zeroed stack out-slot and, as its second argument, whatever
/// the caller left in ebx (unreadable from Rust, skipped in the comparison).
/// A zero out-word fails: the guard is released and 0 returned. Otherwise
/// the object gets its vtable slot-9 method called with the out-word, the
/// handle is unregistered, the guard is released and 1 is returned (in the
/// low byte; the upper bytes are the guard-release answer).
///
/// Original: 0x009DD660 (cdecl, one stack argument, six outgoing calls).
lf_checker_rt::export!(cdecl, rw_009DD660(index: u32) -> u32 {
    unsafe {
        const TABLE_VA: u32 = 0x01295CD8;
        const KEY_OFF: usize = 0x48;
        const SEARCH_TAG_VA: u32 = 0x00E97310;
        const REGISTRY_VA: u32 = 0x016DCEB8;
        const GUARD_ENTER: u32 = 1;
        const VALIDATE: u32 = 2;
        const SEARCH: u32 = 3;
        const UNREGISTER: u32 = 5;
        const GUARD_EXIT: u32 = 6;

        let obj = (lf_checker_rt::global::<u32>(TABLE_VA).wrapping_add(index as usize)
            as *const u32)
            .read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(GUARD_ENTER, u32,);
        let key =
            ((obj as *const u8).wrapping_add(KEY_OFF) as *const i16).read_unaligned() as i32
                as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(VALIDATE, u32, key);
        let mut slot = 0u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            SEARCH, u32,
            &mut slot as *mut u32 as u32,
            0u32,
            lf_checker_rt::relocated(SEARCH_TAG_VA),
            0x70u32,
            0u32
        );
        let out = slot;
        if out == 0 {
            let ans: u32 = lf_checker_rt::callee_cdecl!(GUARD_EXIT, u32,);
            return ans & 0xFFFF_FF00;
        }
        let vtable = (obj as *const u32).read_unaligned();
        let use_obj: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            (((vtable as *const u32).wrapping_add(9)).read_unaligned()) as usize,
        );
        use_obj(obj, out);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            UNREGISTER, u32,
            lf_checker_rt::relocated(REGISTRY_VA),
            index
        );
        let ans: u32 = lf_checker_rt::callee_cdecl!(GUARD_EXIT, u32,);
        (ans & 0xFFFF_FF00) | 1
    }
});
