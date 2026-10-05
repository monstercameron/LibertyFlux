// original: 0x00a91840 stream_walk_lists_step (proposed)

/// Walk each object's callback list, stepping to the next object until done.
///
/// For the current object (starting with `this`), every node of the list at
/// `obj+0x10` (each node a data word followed by the next pointer) is passed
/// after `arg0` to the caller-supplied `callback` (cdecl/2, arg0 first).
/// Callee 2
/// (thiscall, `this`=object, arg `arg0`) then names the next step: -1 ends
/// the walk and is returned, otherwise the object at
/// `obj+idx*4+0x14` becomes current, and a null there ends the walk
/// returning the last step value.
///
/// Thiscall. The proof plants the recorder stub's address as the callback.
lf_checker_rt::export!(thiscall, rw_00a91840(this: u32, arg0: u32, callback: u32) -> u32 {
    unsafe {
        const LIST_OFF: u32 = 0x10;
        const NEXT_OFF: u32 = 0x14;
        const DONE: u32 = 0xffffffff;
        let fire: extern "cdecl" fn(u32, u32) -> u32 = core::mem::transmute(callback);
        let mut obj = this;
        loop {
            let mut node = ((obj + LIST_OFF) as *const u32).read_unaligned();
            while node != 0 {
                let data = (node as *const u32).read_unaligned();
                fire(arg0, data);
                node = ((node + 4) as *const u32).read_unaligned();
            }
            let idx = lf_checker_rt::callee_thiscall!(2, u32, obj, arg0);
            if idx == DONE {
                return DONE;
            }
            obj = ((obj + idx.wrapping_mul(4) + NEXT_OFF) as *const u32).read_unaligned();
            if obj == 0 {
                return idx;
            }
        }
    }
});
