// original: 0x009B6280 script_any_position_component_ready

/// Return true when either of the manager's two script position components is
/// ready. The zero-argument cdecl routine first checks the object at `+0x424`;
/// a nonzero AL result returns one immediately. Otherwise it checks `+0x448`
/// and returns one for any nonzero AL value, or zero when both are clear.
lf_checker_rt::export!(cdecl, rw_009b6280() -> u32 {
    unsafe {
        const MANAGER_GLOBAL: u32 = 0x0118_d804;
        const FIRST_COMPONENT: u32 = 0x424;
        const SECOND_COMPONENT: u32 = 0x448;
        const FIRST_CHECK: u32 = 1;
        const SECOND_CHECK: u32 = 2;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let manager = read_u32(lf_checker_rt::relocated(MANAGER_GLOBAL));
        let first = lf_checker_rt::callee_thiscall!(
            FIRST_CHECK,
            u8,
            manager.wrapping_add(FIRST_COMPONENT)
        );
        if first != 0 {
            1
        } else {
            let second = lf_checker_rt::callee_thiscall!(
                SECOND_CHECK,
                u8,
                manager.wrapping_add(SECOND_COMPONENT)
            );
            u32::from(second != 0)
        }
    }
});
