// original: 0x00900c80 init_tagged_object
/// Initialise a tagged object: two-stage vtable install plus a scrambled tag.
///
/// Stores the interim vtable, folds the global counter into the tag word with
/// a 14-bit mask, bumps the counter, stores the plain argument, installs the
/// final vtable and copies the pointee of `b`. Returns `this`.
export!(thiscall, rw_00900c80(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const VT_INTERIM: u32 = 0xE7E048;
        const VT_FINAL: u32 = 0xE84CB0;
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
        this
    }
});
