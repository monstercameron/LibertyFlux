// original: 0x00ccbda0 CTaskComplexRevive::vf19
/// Start the revive task for a ped, returning the starter's answer or 0.
///
/// Wakes the ped through its virtual slot at `+0xf4` (thiscall, 110.0 and
/// 0), clears bits of the state words at `+0x26c` and `+0x118`, and runs
/// three ped helpers. Unless both words at `[ped+0x224]+0x50`/`+0x54` are
/// zero, the revive check (thiscall on `[ped+0x22c]`, the ped) and its
/// sink (thiscall, the answer, 4, 0) are skipped. A random draw reseeds
/// `ped+0xb94` to 0x40 or 0x3f around the signed 0x3fff cut. When
/// `ped+0x7b8` is 6 the manager handle (thiscall on the global pointer)
/// starts kind (2, 0x67); otherwise two more virtual slots (`+0xb0`,
/// `+0xac`) run before the handle starts the default. A null handle
/// returns 0. ECX is unused; the ped is the stack argument. Thiscall.
export!(thiscall, rw_00ccbda0(_this: u32, ped: u32) -> u32 {
    unsafe {
        const MGR_G: u32 = 0x0167e2a0;
        const RATE110: u32 = 0x42dc0000;
        const DRAW_CUT: i32 = 0x3fff;
        const SEED_LO: u32 = 0x3f;
        let vt = (ped as *const u32).read_unaligned();
        let s1 = (vt.wrapping_add(0xf4) as *const u32).read_unaligned();
        let f1: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(s1 as usize);
        let _: u32 = f1(ped, RATE110, 0u32);
        let w26c = (ped.wrapping_add(0x26c) as *const u32).read_unaligned();
        (ped.wrapping_add(0x26c) as *mut u32).write_unaligned(w26c & 0xffff7ff7);
        let _: u32 = callee_thiscall!(2, u32, ped, 0u32);
        let w118 = (ped.wrapping_add(0x118) as *const u32).read_unaligned();
        (ped.wrapping_add(0x118) as *mut u32).write_unaligned(w118 & 0xfffeffff);
        let _: u32 = callee_thiscall!(3, u32, ped);
        let _: u32 = callee_thiscall!(4, u32, ped, 0u32);
        let w = (ped.wrapping_add(0x224) as *const u32).read_unaligned();
        if (w.wrapping_add(0x50) as *const u32).read_unaligned() == 0
            && (w.wrapping_add(0x54) as *const u32).read_unaligned() == 0
        {
            let q = (ped.wrapping_add(0x22c) as *const u32).read_unaligned();
            let vt5 = (q as *const u32).read_unaligned();
            let s5 = (vt5.wrapping_add(0x0c) as *const u32).read_unaligned();
            let f5: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(s5 as usize);
            let chk = f5(q, ped);
            let _: u32 = callee_thiscall!(6, u32, w.wrapping_add(0x44), chk, 4u32, 0u32);
        }
        let draw: u32 = callee_cdecl!(7, u32,);
        let seed = if (draw as i32) >= DRAW_CUT { SEED_LO + 1 } else { SEED_LO };
        (ped.wrapping_add(0xb94) as *mut u32).write_unaligned(seed);
        let mgr = *global::<u32>(MGR_G);
        if (ped.wrapping_add(0x7b8) as *const u32).read_unaligned() == 6 {
            let h: u32 = callee_thiscall!(8, u32, mgr);
            if h == 0 {
                return 0;
            }
            callee_thiscall!(9, u32, h, 2u32, 0x67u32)
        } else {
            let vt = (ped as *const u32).read_unaligned();
            let sa = (vt.wrapping_add(0xb0) as *const u32).read_unaligned();
            let fa: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(sa as usize);
            let _: u32 = fa(ped);
            let vt = (ped as *const u32).read_unaligned();
            let sb = (vt.wrapping_add(0xac) as *const u32).read_unaligned();
            let fb: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(sb as usize);
            let _: u32 = fb(ped);
            let h: u32 = callee_thiscall!(8, u32, mgr);
            if h == 0 {
                return 0;
            }
            callee_thiscall!(13, u32, h)
        }
    }
});
