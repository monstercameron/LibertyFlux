// original: 0x00C55090 initialize_owned_reference_record (proposed)
/// Initialize a reference-owning record, store the supplied pointer at
/// `this + 0x0c`, and release it through the optional helper when non-null.
/// The base initializer runs first; the helper receives the stored object in
/// ECX and the address of the field as its only stack argument. Both callees
/// are scripted by the proof contract. Thiscall removes the one input word
/// and the method returns `this` in EAX.
lf_checker_rt::export!(thiscall, rw_c55090(this: u32, owned_object: u32) -> u32 {
    unsafe {
        let type_tag = lf_checker_rt::relocated(0x00ec_ae04);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        let field = this.wrapping_add(0x0c);
        (this as *mut u32).write_unaligned(type_tag);
        (field as *mut u32).write_unaligned(owned_object);
        if owned_object != 0 {
            let _ = lf_checker_rt::callee_thiscall!(2, u32, owned_object, field);
        }
        this
    }
});
