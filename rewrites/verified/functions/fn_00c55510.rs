// original: 0x00C55510 release_world_pool_record_reference (proposed)
/// Release the optional object stored at `this + 4`, passing the object in
/// ECX and the field address as the helper's stack argument, then clear the
/// field. The call is intercepted and its one-word pointed-to input is
/// snapshotted by the proof contract. The method has no assigned return value;
/// its `this` pointer is supplied in ECX and it has no stack arguments.
lf_checker_rt::export!(thiscall, rw_c55510(this: u32) -> u32 {
    unsafe {
        const OWNED_OBJECT: u32 = 4;
        let field = this.wrapping_add(OWNED_OBJECT);
        let object = (field as *const u32).read_unaligned();
        if object != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, object, field);
        }
        (field as *mut u32).write_unaligned(0);
        0
    }
});
