// original: 0x00CEAB10 CBike::~CBike__deleting

/// Run the bike destructor and release its slot when the low flag bit is set.
///
/// The thiscall wrapper forwards the object to its destructor, reads the
/// release flag as a byte, and conditionally calls the pool release method
/// with the global pool object and this object. It always returns this and
/// removes the one stack argument.
lf_checker_rt::export!(thiscall, rw_fn_00ceab10(this: u32, flags: u32) -> u32 {
    const BIKE_DESTRUCTOR: u32 = 1;
    const POOL_RELEASE: u32 = 2;
    const POOL_VA: u32 = 0x012e_22a4;

    let _ = lf_checker_rt::callee_thiscall!(BIKE_DESTRUCTOR, u32, this);
    if (flags as u8) & 1 != 0 {
        let pool = unsafe { lf_checker_rt::global::<u32>(POOL_VA).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(POOL_RELEASE, u32, pool, this);
    }
    this
});
