// original: 0x00b4e150 CDummyPed::vf0

/// Deleting destructor of a dummy ped.
///
/// Runs the destructor body (callee 1, thiscall on `this` with no
/// arguments); when bit 0 of `free_flag` is set, the object is returned to
/// the ped pool (callee 2, thiscall on the pool object read from game
/// address `PED_POOL`, passed `this`). Returns `this` either way.
///
/// Original: 0x00b4e150 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b4e150(this: u32, free_flag: u32) -> u32 {
    unsafe {
        const DTOR: u32 = 1;
        const POOL_FREE: u32 = 2;
        const PED_POOL: u32 = 0x018b6f10;
        const FREE_BIT: u32 = 1;
        lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if free_flag & FREE_BIT != 0 {
            let pool = lf_checker_rt::global::<u32>(PED_POOL).read_unaligned();
            lf_checker_rt::callee_thiscall!(POOL_FREE, u32, pool, this);
        }
        this
    }
});
