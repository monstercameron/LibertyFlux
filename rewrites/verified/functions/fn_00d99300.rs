// original: 0x00D99300 CBoat::vf0

/// Run the boat destructor helper, then release the boat's pool slot when the
/// low bit of the scalar-delete flag is set. Return the original object pointer
/// regardless of the flag. The pool pointer comes from the relocated global
/// slot, and the call order is destructor followed by optional release.
///
/// This is a 32-bit thiscall method with one stack flag. The destructor helper
/// takes this in ECX; the release helper takes the pool in ECX and this as its
/// single stack argument.
lf_checker_rt::export!(thiscall, rw_00D99300(this: u32, flags: u32) -> u32 {
    const DESTRUCTOR_CALL: u32 = 4;
    const POOL_RELEASE_CALL: u32 = 5;
    const POOL_POINTER_VA: u32 = 0x012E_22A4;
    let _ = lf_checker_rt::callee_thiscall!(DESTRUCTOR_CALL, u32, this);
    if flags & 1 != 0 {
        let pool = unsafe { lf_checker_rt::global::<u32>(POOL_POINTER_VA).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(POOL_RELEASE_CALL, u32, pool, this);
    }
    this
});
