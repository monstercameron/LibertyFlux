// original: 0x00a888d0 render_object_init_chain
/// Run the three initialisation helpers and return `this`.
///
/// Calls the first helper on the object, the second on the sub-object at
/// `this + 0x9F0`, and the third on the object again (all intercepted),
/// then returns the object pointer.
export!(thiscall, rw_00a888d0(this_obj: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this_obj);
        let _: u32 = callee_thiscall!(2, u32, this_obj.wrapping_add(0x9f0));
        let _: u32 = callee_thiscall!(3, u32, this_obj);
        this_obj
    }
});
