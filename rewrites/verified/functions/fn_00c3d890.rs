// original: 0x00c3d890 train_notify_then_forward_tail (proposed)
/// Notify through the object's slot-2 virtual and tail into the forwarder.
///
/// `obj` points to an object whose first word is its virtual table,
/// `arg` is passed on. Calls the slot at `vtable+8` (thiscall: this=obj,
/// args arg, 0, 0) through the planted stub id 1, exactly like the
/// original's indirect call, then forwards obj to the tail callee id 2
/// (thiscall, no stack words) the way the original's tail jump does.
/// Returns the tail callee's answer.
///
/// Original: 0x00c3d890 (cdecl, two stack words; ends in a tail jump).
lf_checker_rt::export!(cdecl, rw_00c3d890(obj: u32, arg: u32) -> u32 {
    unsafe {
        const VSLOT: u32 = 8;
        const TAIL: u32 = 2;
        let vt = (obj as *const u32).read_unaligned();
        let slot = ((vt + VSLOT) as *const u32).read_unaligned();
        let vcall: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let _: u32 = vcall(obj, arg, 0, 0);
        lf_checker_rt::callee_thiscall!(TAIL, u32, obj)
    }
});
