// original: 0x00A33C80 matrix_release_to_placement_locked

/// Release the placement lock for the shared matrix slot, release this
/// entity's matrix to its placement, then clear the shared slot. The first
/// helper receives the relocated address of the lock word by value; the
/// second helper receives this entity through ECX. Both helper results are
/// ignored. The global write occurs after both calls.
lf_checker_rt::export!(thiscall, rw_00a33c80(this: u32) -> () {
    const LOCK_SLOT_VA: u32 = 0x012d_df1c;
    const RELEASE_LOCK: u32 = 1;
    const RELEASE_MATRIX: u32 = 2;

    unsafe {
        let lock_slot = lf_checker_rt::relocated(LOCK_SLOT_VA);
        let _ = lf_checker_rt::callee_cdecl!(RELEASE_LOCK, u32, lock_slot);
        let _ = lf_checker_rt::callee_thiscall!(RELEASE_MATRIX, u32, this);
        lf_checker_rt::global::<u32>(LOCK_SLOT_VA).write_unaligned(0);
    }
});
