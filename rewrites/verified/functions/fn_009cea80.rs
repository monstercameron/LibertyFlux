// original: 0x009CEA80 input_hook_refresh_dispatch (proposed)

/// Refresh the input-hook state machine, optionally rebuilding the hook.
///
/// `arg` carries one meaningful byte: zero takes the rebuild path, nonzero
/// the refresh path. Every shared-state touch is guarded by the global
/// critical section when the enabled flag is set (enter/leave pairs around
/// each block). The worker object comes from the context callee (id 1); its
/// link field selects between unhooking the old hook, querying the OS
/// version and installing a fresh low-level keyboard hook, or skipping
/// ahead. Three display/input queries run on the refresh path; three
/// conditional updates run on the rebuild path when their cached words have
/// bit 0 clear. Both exits run the cookie check, which preserves registers.
///
/// All operating-system calls go through import slots, loaded and called
/// exactly like the original so both sides land on the same checker stubs.
/// Absolute addresses (the critical-section pointer, the hook procedure)
/// are relocated, never hard-coded.
///
/// Original: 0x009CEA80 (cdecl, one word-sized stack argument, low byte used).
lf_checker_rt::export!(cdecl, rw_009CEA80(arg: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x103AD58;
        const CS_PTR: u32 = 0x129588C;
        const IAT_ENTER: u32 = 0xE731CC;
        const IAT_LEAVE: u32 = 0xE731C8;
        const IAT_SPI: u32 = 0xE73424;
        const IAT_UNHOOK: u32 = 0xE73430;
        const IAT_VEREX: u32 = 0xE73278;
        const IAT_MODH: u32 = 0xE73274;
        const IAT_HOOKEX: u32 = 0xE7342C;
        const HOOK_PROC: u32 = 0x9D09F0;
        const HOOK_ID: u32 = 0xD;
        const CAL_CTX: u32 = 1;
        const CAL_BUILD_A: u32 = 9;
        const CAL_BUILD_B: u32 = 10;
        const CAL_COOKIE: u32 = 11;
        const LINK: u32 = 0x2BC;
        const STAMP: u32 = 0x2C2;
        const F_A: u32 = 0x2C4;
        const F_B: u32 = 0x2C8;
        const F_C: u32 = 0x2CC;
        const F_D: u32 = 0x2D0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn flag_on() -> bool {
            unsafe { rd8(lf_checker_rt::relocated(FLAG)) != 0 }
        }
        #[inline(always)]
        unsafe fn enter_cs() {
            unsafe {
                let f: extern "stdcall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(lf_checker_rt::relocated(IAT_ENTER)) as usize);
                f(lf_checker_rt::relocated(CS_PTR));
            }
        }
        #[inline(always)]
        unsafe fn leave_cs() {
            unsafe {
                let f: extern "stdcall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(lf_checker_rt::relocated(IAT_LEAVE)) as usize);
                f(lf_checker_rt::relocated(CS_PTR));
            }
        }
        #[inline(always)]
        unsafe fn spi(act: u32, param: u32, out: u32) -> u32 {
            unsafe {
                let f: extern "stdcall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(lf_checker_rt::relocated(IAT_SPI)) as usize);
                f(act, param, out, 0)
            }
        }
        #[inline(always)]
        unsafe fn ctx() -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(CAL_CTX, u32,) }
        }
        #[inline(always)]
        unsafe fn cookie() {
            unsafe {
                let _: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            }
        }

        let al = (arg & 0xFF) as u8;
        // Entry block: stamp the context byte under the guard.
        let mut esi = ctx();
        if flag_on() {
            enter_cs();
        }
        wr8(esi.wrapping_add(STAMP), al);
        if flag_on() {
            leave_cs();
        }
        if al != 0 {
            // Refresh path.
            esi = ctx();
            if flag_on() {
                enter_cs();
            }
            let w0 = rd32(esi.wrapping_add(F_A));
            let _w1 = rd32(esi.wrapping_add(F_B));
            if flag_on() {
                leave_cs();
            }
            esi = ctx();
            if flag_on() {
                enter_cs();
            }
            let w2 = rd32(esi.wrapping_add(F_C));
            let _w3 = rd32(esi.wrapping_add(F_D));
            if flag_on() {
                leave_cs();
            }
            let mut build_slot = [0u32; 2];
            let bctx = ctx();
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CAL_BUILD_A,
                u32,
                bctx,
                build_slot.as_mut_ptr() as u32
            );
            let mut q0 = [w0; 1];
            let mut q1 = [w2; 1];
            let mut q2 = [build_slot[0]; 1];
            spi(0x3B, 8, q0.as_mut_ptr() as u32);
            spi(0x35, 8, q1.as_mut_ptr() as u32);
            spi(0x33, 0x18, q2.as_mut_ptr() as u32);
            let t5 = ctx();
            esi = t5;
            if flag_on() {
                enter_cs();
            }
            esi = rd32(esi.wrapping_add(LINK));
            if flag_on() {
                leave_cs();
            }
            if esi == 0 {
                cookie();
                return if flag_on() { 0 } else { t5 };
            }
            esi = ctx();
            if flag_on() {
                enter_cs();
            }
            esi = rd32(esi.wrapping_add(LINK));
            if flag_on() {
                leave_cs();
            }
            let unhook: extern "stdcall" fn(u32) -> u32 =
                core::mem::transmute(rd32(lf_checker_rt::relocated(IAT_UNHOOK)) as usize);
            unhook(esi);
            esi = ctx();
            if flag_on() {
                enter_cs();
            }
            let retv = esi;
            wr32(esi.wrapping_add(LINK), 0);
            if !flag_on() {
                cookie();
                return retv;
            }
            leave_cs();
            cookie();
            return 0;
        }
        // Rebuild path.
        esi = ctx();
        if flag_on() {
            enter_cs();
        }
        esi = rd32(esi.wrapping_add(LINK));
        if flag_on() {
            leave_cs();
        }
        if esi == 0 {
            let mut ver = [0u32; 5];
            ver[0] = 0x94;
            let verex: extern "stdcall" fn(u32) -> u32 =
                core::mem::transmute(rd32(lf_checker_rt::relocated(IAT_VEREX)) as usize);
            verex(ver.as_mut_ptr() as u32);
            if ver[4] == 2 && ver[1] > 4 {
                let modh: extern "stdcall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(lf_checker_rt::relocated(IAT_MODH)) as usize);
                let h = modh(0);
                let hookex: extern "stdcall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(lf_checker_rt::relocated(IAT_HOOKEX)) as usize);
                let hh = hookex(HOOK_ID, lf_checker_rt::relocated(HOOK_PROC), h, 0);
                let bctx = ctx();
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_BUILD_B, u32, bctx, hh);
            }
        }
        esi = ctx();
        if flag_on() {
            enter_cs();
        }
        let w0 = rd32(esi.wrapping_add(F_A));
        let w1 = rd32(esi.wrapping_add(F_B));
        if flag_on() {
            leave_cs();
        }
        if (w1 & 1) == 0 {
            let mut slot = [w0; 1];
            spi(0x3B, 8, slot.as_mut_ptr() as u32);
        }
        let ebp_v = ctx();
        if flag_on() {
            enter_cs();
        }
        let w2 = rd32(ebp_v.wrapping_add(F_C));
        let w3 = rd32(ebp_v.wrapping_add(F_D));
        if flag_on() {
            leave_cs();
        }
        if (w3 & 1) == 0 {
            let mut slot = [w2; 1];
            spi(0x35, 8, slot.as_mut_ptr() as u32);
        }
        let mut build_slot = [0u32; 2];
        let bctx = ctx();
        let _: u32 =
            lf_checker_rt::callee_thiscall!(CAL_BUILD_A, u32, bctx, build_slot.as_mut_ptr() as u32);
        if (build_slot[1] & 1) != 0 {
            cookie();
            return build_slot[1];
        }
        let mut slot = [build_slot[0]; 1];
        spi(0x33, 0x18, slot.as_mut_ptr() as u32);
        cookie();
        return 1;
    }
});
