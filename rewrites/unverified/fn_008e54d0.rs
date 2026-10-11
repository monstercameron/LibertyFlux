// original: 0x008E54D0 audio_frontend_ready_check

/// Check the frontend's global and entity readiness gates, then report audio readiness.
///
/// Several global dwords and bytes gate the checks. The entity must have zero
/// dwords at offsets `0x204` and `0x234`, and a zero byte at `0x401`. The first
/// helper's AL result must be zero. The next helper's nonzero AL result
/// succeeds immediately; otherwise a nonzero global byte at `0x11609F6` and a
/// nonzero final helper result are both required. All three outgoing calls
/// are scripted by the contract. This is a 32-bit thiscall method with no
/// stack arguments and a full EAX boolean result.
lf_checker_rt::export!(thiscall, rw_008e54d0(this: u32) -> u32 {
    unsafe {
        const FRONTEND_DISABLED: u32 = 0x011f_7060;
        const MODE_VALUE: u32 = 0x0120_88b4;
        const MODE_EXPECTED: u32 = 0x00f1_c040;
        const FRONTEND_MODE: u32 = 0x0103_7720;
        const INITIAL_GUARD: u32 = 0x0116_0c3d;
        const PRIMARY_READY: u32 = 0x0128_45c8;
        const SECONDARY_READY: u32 = 0x0115_dbfc;
        const FINAL_READY: u32 = 0x0116_09f6;
        const ENTITY_PENDING: usize = 0x204;
        const ENTITY_BUSY: usize = 0x234;
        const ENTITY_BLOCKED: usize = 0x401;

        if (lf_checker_rt::global::<u32>(FRONTEND_DISABLED)).read_unaligned() == 1 {
            return 0;
        }
        let mode_value = (lf_checker_rt::global::<u32>(MODE_VALUE)).read_unaligned();
        let expected_mode = (lf_checker_rt::global::<u32>(MODE_EXPECTED)).read_unaligned();
        if mode_value != expected_mode
            || (lf_checker_rt::global::<u32>(FRONTEND_MODE)).read_unaligned() == 0x12
            || (lf_checker_rt::global::<u8>(INITIAL_GUARD)).read() != 0
        {
            return 0;
        }

        let first_ready = lf_checker_rt::callee_thiscall!(1, u8, this);
        if first_ready != 0 {
            return 0;
        }
        if (this as usize as *const u32).add(ENTITY_PENDING / 4).read_unaligned() != 0
            || (this as usize as *const u32).add(ENTITY_BUSY / 4).read_unaligned() != 0
        {
            return 0;
        }
        let primary_ready = (lf_checker_rt::global::<u8>(PRIMARY_READY)).read();
        if primary_ready != first_ready
            && (lf_checker_rt::global::<u8>(SECONDARY_READY)).read() == first_ready
        {
            return 0;
        }
        if (this as usize as *const u8).add(ENTITY_BLOCKED).read() != 0 {
            return 0;
        }

        let quick_ready = lf_checker_rt::callee_cdecl!(2, u8,);
        if quick_ready != 0 {
            return 1;
        }
        if (lf_checker_rt::global::<u8>(FINAL_READY)).read() == 0 {
            return 0;
        }
        if lf_checker_rt::callee_cdecl!(3, u8,) != 0 {
            1
        } else {
            0
        }
    }
});
