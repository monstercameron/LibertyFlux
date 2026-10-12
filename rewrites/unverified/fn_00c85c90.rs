// original: 0x00C85C90 phArchetypeGta::~phArchetypeGta

/// Install the archetype's vtable in the receiver and tail-call the base
/// fragment routine. The vtable pointer is stored at offset zero; the base
/// routine determines the returned value.
lf_checker_rt::export!(thiscall, rw_00c85c90(this: u32) -> u32 {
    const ARCHETYPE_VTABLE: u32 = 0x00ED_6364;
    const DESTROY_BASE: u32 = 1;

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(ARCHETYPE_VTABLE));
    }
    lf_checker_rt::callee_thiscall!(DESTROY_BASE, u32, this)
});
