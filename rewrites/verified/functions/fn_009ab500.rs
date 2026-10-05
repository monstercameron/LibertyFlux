// original: 0x009AB500 audExplosionAudioEntity::vf0 (symbols)

/// Deleting destructor of `audExplosionAudioEntity` (vector slot 0).
///
/// Writes the class vtable pointer into `this`, runs the destructor body
/// callee on it, and when bit 0 of `flags` is set frees the object through
/// the allocator callee. Returns `this`.
/// Original: 0x009AB500 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_009AB500(this: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00e83134;
        const DTOR_BODY: u32 = 1;
        const OPERATOR_DELETE: u32 = 2;
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let _: u32 = lf_checker_rt::callee_thiscall!(DTOR_BODY, u32, this);
        if flags & 1 != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(OPERATOR_DELETE, u32, this);
        }
        this
    }
});
