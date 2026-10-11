// original: 0x009B6220 script_maximum_position_value

/// Return the greater of two script position values from the manager object.
/// The zero-argument cdecl function calls the float getter for manager fields
/// `+0x424` and `+0x448`. A strict greater-than selects the first value; equal
/// values and unordered comparisons select the second, preserving the chosen
/// value's exact `f32` bits in the ST0 return.
lf_checker_rt::export!(cdecl, rw_009b6220() -> f32 {
    unsafe {
        const MANAGER_GLOBAL: u32 = 0x0118_d804;
        const FIRST_FIELD: u32 = 0x424;
        const SECOND_FIELD: u32 = 0x448;
        const FIRST_GETTER: u32 = 1;
        const SECOND_GETTER: u32 = 2;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let manager = read_u32(lf_checker_rt::relocated(MANAGER_GLOBAL));
        let first = lf_checker_rt::callee_thiscall!(
            FIRST_GETTER,
            f32,
            manager.wrapping_add(FIRST_FIELD)
        );
        let second = lf_checker_rt::callee_thiscall!(
            SECOND_GETTER,
            f32,
            manager.wrapping_add(SECOND_FIELD)
        );
        if first > second { first } else { second }
    }
});
