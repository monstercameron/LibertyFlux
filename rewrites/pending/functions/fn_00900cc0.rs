// original: 0x00900cc0 init_tagged_object_ex
/// Initialise a tagged object with one extra pointee field.
///
/// Same shape as [`rw_00900c80`] with a different final vtable and a third
/// argument whose pointee lands one slot further on. Returns `this`.
export!(thiscall, rw_00900cc0(this: u32, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        const VT_INTERIM: u32 = 0xE7E048;
        const VT_FINAL: u32 = 0xE84C5C;
        const COUNTER: u32 = 0x10327a0;
        let old = (this as *const u32).add(1).read();
        (this as *mut u32).write(relocated(VT_INTERIM));
        let g = *global::<u32>(COUNTER);
        (this as *mut u32).add(1).write(old ^ ((old ^ g) & 0x3fff));
        *global::<u32>(COUNTER) = g.wrapping_add(1);
        (this as *mut u32).add(2).write(a);
        (this as *mut u32).write(relocated(VT_FINAL));
        let bv = (b as *const u32).read();
        (this as *mut u32).add(3).write(bv);
        let cv = (c as *const u32).read();
        (this as *mut u32).add(4).write(cv);
        this
    }
});
