// original: 0x00b58ac0 ped_factory_init (proposed)

/// Initialise a ped factory object: install its virtual table, run six
/// chained initialisers (all scripted under one callee id) with `this`,
/// then set byte `+0x19` of the object the global at 0x0171f9c0 points to.
/// Returns `this`.
lf_checker_rt::export!(thiscall, rw_00b58ac0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ea_f77c;
        const FLAG_BYTE: u32 = 0x19;
        const SHARED: u32 = 0x0171_f9c0;
        (this as *mut u32).write_unaligned(VTABLE);
        lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(1, u32, this);
        let shared = lf_checker_rt::global::<u32>(SHARED).read_unaligned();
        (shared.wrapping_add(FLAG_BYTE) as *mut u8).write(1);
        this
    }
});
