// original: 0x0059E1C0 timing_tick_update (proposed)

/// Run one mainloop timing tick: poll the timing helpers, publish the
/// tick results to the timing globals, and run the scheduled-tick callback
/// registration.
///
/// No arguments and no object: everything flows through globals. The call
/// order is fixed except for three scripted gates: the helper at gate `G1`
/// runs only when the gate word is zero, its follow-up only when the
/// helper's low byte (`AL`) is non-zero, and the exchange-and-register tail
/// only when the tail flag byte is non-zero and the interlocked exchange
/// reports zero. The four out-pointers handed to the sample helper address
/// the function's own frame (compared by snapshot, not by address); the two
/// uninitialised words copied into the registration call's stack arguments
/// read the defined stack fill (`0`); the registration buffer handed to the
/// final call holds `{0x59E400, 0, 0, 0x430260}`. The indirect call goes
/// through TLS slot 0 (`[slot+8]`, then the table at
/// `[ECX]` slot `+8`, thiscall with `(0x40, 0x10, 0)`); its answer and the
/// float (`XMM0`) answer of the tune helper are published to globals. The
/// clear loop zeroes 57 words (`0x4F..0x88`, signed bound, with two dead
/// skip conditions for `0x89` and `0x7FFFFFFF` that can never hit).
/// Returns the last callee answer on the taken path.
///
/// Original: 0x0059E1C0 (cdecl, no arguments, `EAX` return).
/// Encrypted callees are direct `E8` sites and are patched like the rest.
lf_checker_rt::export!(cdecl, rw_0059E1C0() -> u32 {
    unsafe {
        const G_NEG1: u32 = 0x18B6C84;
        const G_GATE: u32 = 0x110E6EC;
        const G_A: u32 = 0x1160C4C;
        const G_B0: u32 = 0x1BB37B0;
        const G_BC: u32 = 0x1BB37BC;
        const G_C: u32 = 0x18B6E84;
        const G_D90: u32 = 0x18B6C90;
        const G_D93: u32 = 0x18B6C93;
        const G_DA5: u32 = 0x18B6CA5;
        const G_E: u32 = 0x105C6B6;
        const G_F: u32 = 0x18B6E80;
        const G_FLTCONST: u32 = 0x106C290;
        const G_G: u32 = 0x18B6E9C;
        const G_ANS9: u32 = 0x18B6E98;
        const G_ANS12: u32 = 0x18B6E90;
        const G_31: u32 = 0x106C2C8;
        const G_H: u32 = 0x18E51E0;
        const G_I0: u32 = 0x1160CF4;
        const LOOP_BASE: u32 = 0x1160C48;
        const G_CB: u32 = 0x18B82CB;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn wg32(va: u32, v: u32) {
            unsafe { wr32(lf_checker_rt::relocated(va), v) }
        }
        #[inline(always)]
        unsafe fn wg8(va: u32, v: u8) {
            unsafe { wr8(lf_checker_rt::relocated(va), v) }
        }

        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        wg32(G_NEG1, 0xFFFFFFFF);
        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
        if g32(G_GATE) == 0 {
            let al: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
            if (al & 0xFF) != 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(6, u32,);
            }
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(7, u32,);
        let b0: u32 = 0;
        let b1: u32 = 0;
        let b2: u32 = 0;
        let b3: u32 = 0;
        let _: u32 = lf_checker_rt::callee_cdecl!(8, u32, &b0 as *const u32 as u32,
            &b1 as *const u32 as u32, &b2 as *const u32 as u32,
            &b3 as *const u32 as u32);
        let p1: u32 = lf_checker_rt::tls_slot(0);
        let thisv: u32 = rd32(p1.wrapping_add(8));
        wg32(G_A, 0);
        wg32(G_B0, 0);
        wg32(G_B0.wrapping_add(4), 0);
        wg32(G_B0.wrapping_add(8), 0);
        wg32(G_BC, 0);
        wg32(G_C, 0);
        wg8(G_D90, 0);
        wg8(G_D93, 0);
        wg8(G_DA5, 0);
        wg8(G_E, 0);
        wg8(G_F, 0);
        wg32(G_FLTCONST, 0x433E0000);
        wg32(G_G, 0);
        let vt: u32 = rd32(thisv);
        let slot: u32 = rd32(vt.wrapping_add(8));
        let ind: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        let ans9: u32 = ind(thisv, 0x40, 0x10, 0);
        wg32(G_ANS9, ans9);
        let _: u32 = lf_checker_rt::callee_cdecl!(10, u32, lf_checker_rt::relocated(0x5ADE70));
        let _: u32 = lf_checker_rt::callee_cdecl!(11, u32,);
        let ecx12: u32 = (thisv & 0xFFFFFF00) | 1;
        let ans12: u32 = lf_checker_rt::callee_thiscall!(12, u32, ecx12);
        wg32(G_ANS12, ans12);
        wg32(G_31, 0x31);
        let _: u32 = lf_checker_rt::callee_cdecl!(13, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(14, u32,);
        wg8(G_H, 0);
        wg32(G_I0, 0);
        wg32(G_I0.wrapping_add(4), 0);
        wg32(G_I0.wrapping_add(8), 0);
        wg32(G_I0.wrapping_add(12), 0);
        wg32(G_I0.wrapping_add(16), 0);
        wg32(G_I0.wrapping_add(20), 0);
        let mut eax: u32 = 0x4F;
        while (eax as i32) < 0x88 {
            if eax == 0x89 || eax == 0x7FFFFFFF {
            } else {
                wg32(LOOP_BASE.wrapping_add(eax.wrapping_mul(4)), 0);
            }
            eax = eax.wrapping_add(1);
        }
        let ans15: u32 = lf_checker_rt::callee_cdecl!(15, u32, lf_checker_rt::relocated(0x4016A0), 0, 0,
            lf_checker_rt::relocated(0x430260), lf_checker_rt::relocated(0x59E400), 0, 0, lf_checker_rt::relocated(0x430260));
        if rd8(lf_checker_rt::relocated(G_CB)) == 0 {
            return ans15;
        }
        let ans16: u32 =
            lf_checker_rt::callee_stdcall!(16, u32, lf_checker_rt::relocated(0x110E6DC), lf_checker_rt::relocated(0x19F31D4), 0);
        if ans16 != 0 {
            return ans16;
        }
        let buf: [u32; 4] = [lf_checker_rt::relocated(0x59E400), 0, 0,
            lf_checker_rt::relocated(0x430260)];
        let ans17: u32 = lf_checker_rt::callee_thiscall!(17, u32,
            lf_checker_rt::relocated(0x19F31D4), buf.as_ptr() as u32, ans16,
            lf_checker_rt::relocated(0x110E6CC));
        ans17
    }
});
