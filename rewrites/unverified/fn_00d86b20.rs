// original: 0x00D86B20 input_mode_dispatch (proposed)

/// Dispatch an input-mode request on an object to one of nineteen handlers.
///
/// `a0` is the object (status word at `+0x1304`, mode byte at `+0xE6E`,
/// magnitude byte at `+0xE6F`, count byte at `+0xEFA`, inner state at
/// `+0x20`); `a1`..`a5` are five words passed through to the handlers, some
/// receiving results. On entry a status of 4 runs a flag/float gate (a set
/// flag byte at `+0x11A` or either of two floats at `+0x1EF8`/`+0x1EFC` not
/// above zero ordered sends the object to a reset helper, callee 21, and
/// returns; NaN floats fall through to the dispatch), a status of 5 with the
/// flag byte set runs a notifier (callee 19) and returns, and every other
/// combination reaches the dispatch.
///
/// Dispatch is two-level on the mode byte. Modes 2, 6, 7, 9 through 13 and
/// 28 first call a lookup helper (callee 1); a zero answer rewrites the mode
/// to 5. The main switch then selects the handler: 0 stores defaults and,
/// for status 4, a signed vehicle check (callee 7, `<= 0` means set); 1
/// forwards to callee 8 unless the magnitude byte is zero; 2 marshals two
/// floats from callee 3 through callees 4 and 5; 3 chains the lookup helper
/// (callee 2) and two virtual-slot (`+0xEC`) calls into an eleven-argument
/// scorer (callee 22); 4 picks callee 6, 9 or 8 by status; 5 and 6 marshal
/// floats into callee 5, and 6 continues into a length-and-blend tail that
/// writes two outputs; 7 pairs the lookup helper with callee 13; 8 pairs
/// callee 3 with the neighbouring steer routine (callee 15); 9, 10, 11, 12,
/// 15 and 17 are plain forwarders (callees 10, 11, 16, 17, 12, 14); 13 and 14
/// pick a reset helper by status; 16 mirrors case 2 with a zero handle or
/// picks callee 6 or 9 by status; modes 16, 22, 24 and anything above 32
/// (including every negative byte, compared unsigned) take the thread-local
/// path, which resolves slot `+0x4C8` of the thread block selected by the
/// global slot index and calls callee 20.
///
/// All float comparisons are the original's ordered single-precision
/// compares (NaN takes the same side in every branch), every multi-operand
/// float expression keeps the original's operand order, and one blend adds
/// the bits of a heap address as a float, which is deterministic because the
/// heap base is fixed. The dispatch tables live in the executable's code, so
/// they are matched on the mode value directly rather than read.
///
/// Original: 0x00D86B20 (cdecl, six stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00D86B20(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const INNER: u32 = 0x20;
        const STATUS: u32 = 0x1304;
        const FLAG11A: u32 = 0x11A;
        const F1EF8: u32 = 0x1EF8;
        const F1EFC: u32 = 0x1EFC;
        const MODE: u32 = 0xE6E;
        const MAG: u32 = 0xE6F;
        const EFA: u32 = 0xEFA;
        const E48: u32 = 0xE48;
        const E4C: u32 = 0xE4C;
        const E50: u32 = 0xE50;
        const E54: u32 = 0xE54;
        const EE0: u32 = 0xEE0;
        const B0: u32 = 0x1EB0;
        const B4: u32 = 0x1EB4;
        const B8: u32 = 0x1EB8;
        const BC: u32 = 0x1EBC;
        const C0: u32 = 0x1EC0;
        const VT_SLOT: u32 = 0xEC;
        const TLS_INDEX: u32 = 0x17ABA14;
        const TLS_MAGIC: u32 = 0xEEC754;
        const TLS_PLUS: u32 = 0x4C8;
        const HALF_BITS: u32 = 0x3F000000;
        const K95_BITS: u32 = 0x3F733333;
        const ZERO: f32 = f32::from_bits(0x00000000);
        const ONE: f32 = f32::from_bits(0x3F800000);
        const TWO: f32 = f32::from_bits(0x40000000);
        const FIVE: f32 = f32::from_bits(0x40A00000);
        const TEN: f32 = f32::from_bits(0x41200000);
        const NEG_TENTH: f32 = f32::from_bits(0xBDCCCCCD);
        const TWENTIETH: f32 = f32::from_bits(0x3D4CCCCD);

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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn b1(edi: u32) -> u32 {
            unsafe { lf_checker_rt::callee_thiscall!(1, u32, edi.wrapping_add(E48)) }
        }
        #[inline(always)]
        unsafe fn b2(edi: u32) -> u32 {
            unsafe { lf_checker_rt::callee_thiscall!(2, u32, edi.wrapping_add(E48)) }
        }
        #[inline(always)]
        unsafe fn af(edi: u32, slot: *mut u32, extra: u32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(3, u32, edi.wrapping_add(E48), slot as u32, extra);
            }
        }
        #[inline(always)]
        unsafe fn vcall(obj: u32, slot: *mut u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(VT_SLOT)) as usize);
                f(obj, slot as u32)
            }
        }
        /// Follow `+0x20`, or fall back to `+0x10` when it is null.
        #[inline(always)]
        unsafe fn nulladj(p: u32) -> u32 {
            unsafe {
                let q = rd32(p.wrapping_add(INNER));
                if q != 0 {
                    q.wrapping_add(0x30)
                } else {
                    p.wrapping_add(0x10)
                }
            }
        }

        let edi = a0;
        let st = rd32(edi.wrapping_add(STATUS));
        if st == 4 {
            if (rd8(edi.wrapping_add(FLAG11A)) & 1) != 0 {
                lf_checker_rt::callee_cdecl!(21, u32, edi);
                return 0;
            }
            // jae falls through for NaN too: reset unless both are ordered > 0.
            if ZERO >= rdf(edi.wrapping_add(F1EF8)) || ZERO >= rdf(edi.wrapping_add(F1EFC)) {
                lf_checker_rt::callee_cdecl!(21, u32, edi);
                return 0;
            }
        } else if st == 5 {
            if (rd8(edi.wrapping_add(FLAG11A)) & 1) != 0 {
                lf_checker_rt::callee_cdecl!(19, u32, edi);
                return 0;
            }
        }

        // Pre-switch: some modes run the lookup gate first.
        let mut mode = rd8(edi.wrapping_add(MODE));
        let d = (mode as i8 as i32).wrapping_sub(2);
        if (d as u32) <= 0x1A {
            match d {
                0 | 4 | 5 | 7 | 8 | 9 | 10 | 11 | 26 => {
                    if b1(edi) == 0 {
                        wr8(edi.wrapping_add(MODE), 5);
                        mode = 5;
                    }
                }
                _ => {}
            }
        }

        // Main switch on the (possibly rewritten) mode, unsigned.
        let ms = mode as i8 as i32;
        if (ms as u32) > 0x20 {
            // Thread-local path, shared with table entries 16, 22 and 24.
            let idx = rd32(lf_checker_rt::relocated(TLS_INDEX));
            let tp = lf_checker_rt::tls_slot(idx as usize).wrapping_add(TLS_PLUS);
            lf_checker_rt::callee_cdecl!(
                20,
                u32,
                tp,
                lf_checker_rt::relocated(TLS_MAGIC),
                ms as u32
            );
            return 0;
        }
        match ms {
            0x00 | 0x05 | 0x08 | 0x1B | 0x1D | 0x1E | 0x1F | 0x20 => {
                wr32(a1, 0);
                wr32(a2, 0);
                wr8(a4, 1);
                wr32(a3, HALF_BITS);
                let st2 = rd32(edi.wrapping_add(STATUS));
                if st2 == 4 {
                    wr32(edi.wrapping_add(B4), 0);
                    wr32(edi.wrapping_add(B8), 0);
                    wr32(edi.wrapping_add(BC), 0);
                    let r: u32 = lf_checker_rt::callee_thiscall!(7, u32, edi);
                    let h_le = (r as i32) <= 0; // MUT2-SIGNED
                    if h_le {
                        wr32(edi.wrapping_add(C0), K95_BITS);
                    } else {
                        wr32(edi.wrapping_add(C0), 0);
                    }
                } else if st2 == 5 {
                    wr32(edi.wrapping_add(B0), 0);
                    wr32(edi.wrapping_add(B4), 0);
                    wr32(edi.wrapping_add(B8), 0);
                    wr32(edi.wrapping_add(BC), 0);
                }
            }
            0x01 => {
                if rd8(edi.wrapping_add(MAG)) != 0 {
                    lf_checker_rt::callee_cdecl!(8, u32, edi, a1, a2, a3, a4, a5);
                } else {
                    wr32(a1, 0);
                    wr32(a2, 0);
                    wr8(a4, 1);
                    wr32(a3, HALF_BITS);
                }
            }
            0x02 => {
                let mut s = [0u32; 2];
                af(edi, s.as_mut_ptr(), edi);
                let r = b2(edi);
                let f = rd8(edi.wrapping_add(MAG)) as f32;
                let res: u32 = lf_checker_rt::callee_cdecl!(
                    4, u32, r, s[0], s[1], a1, a2, a3, a4, a5, f.to_bits()
                );
                lf_checker_rt::callee_cdecl!(
                    5, u32, edi, res, s[0], s[1], a1, a2, a3, a4, a5, f.to_bits()
                );
            }
            0x03 => {
                let r1 = b2(edi);
                let r2 = b2(edi);
                let r3 = b2(edi);
                let p3 = nulladj(r3);
                let r4 = b2(edi);
                let p4 = nulladj(r4);
                let mut slot1: u32 = 0;
                let v1 = vcall(r2, core::ptr::addr_of_mut!(slot1));
                let mut slot2: u32 = 0;
                let v2 = vcall(r1, core::ptr::addr_of_mut!(slot2));
                let r5 = b2(edi);
                lf_checker_rt::callee_cdecl!(
                    22, u32, edi, r5, rd32(p4), rd32(p3.wrapping_add(4)), rd32(v1),
                    rd32(v2.wrapping_add(4)), a1, a2, a3, a4, a5
                );
            }
            0x04 => {
                let st2 = rd32(edi.wrapping_add(STATUS));
                if st2 == 4 {
                    lf_checker_rt::callee_cdecl!(6, u32, edi, a5, 0);
                } else if st2 == 5 {
                    lf_checker_rt::callee_cdecl!(9, u32, edi);
                } else {
                    lf_checker_rt::callee_cdecl!(8, u32, edi, a1, a2, a3, a4, a5);
                }
            }
            0x06 | 0x07 => {
                let st2 = rd32(edi.wrapping_add(STATUS));
                if st2 == 4 {
                    fn_00d86b20_cfb(edi, a5);
                } else if st2 != 5 {
                    if ms == 0x06 {
                        if b1(edi) == 0 {
                            return 0;
                        }
                    }
                    let r1 = b2(edi);
                    let p1 = nulladj(r1);
                    let r2 = b2(edi);
                    let p2 = nulladj(r2);
                    let f = rd8(edi.wrapping_add(MAG)) as f32;
                    let r3 = b2(edi);
                    lf_checker_rt::callee_cdecl!(
                        5, u32, edi, r3, rd32(p2), rd32(p1.wrapping_add(4)), a1, a2, a3, a4,
                        a5, f.to_bits()
                    );
                    if ms == 0x07 {
                        fn_00d86b20_c6tail(edi, a1, a2, a3, a4, a5, p1);
                    }
                }
            }
            0x09 => {
                let r = b2(edi);
                lf_checker_rt::callee_cdecl!(13, u32, edi, r, a1, a2, a3, a4);
            }
            0x0A | 0x0B | 0x0C | 0x0D => {
                let mut s = [0u32; 2];
                af(edi, s.as_mut_ptr(), edi);
                let r = b2(edi);
                lf_checker_rt::callee_cdecl!(
                    15, u32, edi, r, s.as_mut_ptr() as u32, a1, a2, a3, a4, a5
                );
            }
            0x0E => {
                lf_checker_rt::callee_cdecl!(10, u32, edi, a1, a2, a3, a4, a5);
            }
            0x0F => {
                lf_checker_rt::callee_cdecl!(11, u32, edi, a1, a2, a3, a4, a5);
            }
            0x10 | 0x16 | 0x18 => {
                let idx = rd32(lf_checker_rt::relocated(TLS_INDEX));
                let tp = lf_checker_rt::tls_slot(idx as usize).wrapping_add(TLS_PLUS);
                lf_checker_rt::callee_cdecl!(
                    20,
                    u32,
                    tp,
                    lf_checker_rt::relocated(TLS_MAGIC),
                    ms as u32
                );
            }
            0x11 | 0x17 => {
                lf_checker_rt::callee_cdecl!(16, u32, edi, a1, a2, a3, a4, a5);
            }
            0x12 | 0x19 => {
                lf_checker_rt::callee_cdecl!(17, u32, edi, a1, a2, a3, a4, a5);
            }
            0x13 => {
                if rd32(edi.wrapping_add(STATUS)) == 4 {
                    lf_checker_rt::callee_cdecl!(18, u32, edi, a5);
                }
            }
            0x14 => {
                let st2 = rd32(edi.wrapping_add(STATUS));
                if st2 == 4 {
                    lf_checker_rt::callee_cdecl!(21, u32, edi);
                } else if st2 == 5 {
                    lf_checker_rt::callee_cdecl!(19, u32, edi);
                }
            }
            0x15 => {
                lf_checker_rt::callee_cdecl!(12, u32, edi, a1, a2, a3, a4, a5);
            }
            0x1A => {
                let st2 = rd32(edi.wrapping_add(STATUS));
                if st2 == 4 {
                    let mut s = [0u32; 2];
                    af(edi, s.as_mut_ptr(), edi);
                    lf_checker_rt::callee_cdecl!(6, u32, edi, a5, 0);
                } else if st2 == 5 {
                    lf_checker_rt::callee_cdecl!(9, u32, edi);
                } else {
                    let mut s = [0u32; 2];
                    af(edi, s.as_mut_ptr(), edi);
                    let f = rd8(edi.wrapping_add(MAG)) as f32;
                    lf_checker_rt::callee_cdecl!(
                        5, u32, edi, 0, s[0], s[1], a1, a2, a3, a4, a5, f.to_bits()
                    );
                }
            }
            0x1C => {
                lf_checker_rt::callee_cdecl!(14, u32, edi.wrapping_add(E48), edi, a1, a2, a3, a4);
            }
            _ => {}
        }
        0
    }
});

/// Shared status-4 handler of cases 5 and 6: copy three floats, then callee 6.
unsafe fn fn_00d86b20_cfb(edi: u32, a5: u32) {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(2, u32, edi.wrapping_add(0xE48));
        let q = rd32(r.wrapping_add(0x20));
        let p = if q != 0 { q.wrapping_add(0x30) } else { r.wrapping_add(0x10) };
        unsafe {
            ((edi.wrapping_add(0xE4C)) as *mut u32)
                .write_unaligned(rd32(p));
            ((edi.wrapping_add(0xE50)) as *mut u32)
                .write_unaligned(rd32(p.wrapping_add(4)));
            ((edi.wrapping_add(0xE54)) as *mut u32)
                .write_unaligned(rd32(p.wrapping_add(8)));
        }
        lf_checker_rt::callee_cdecl!(6, u32, edi, a5, 0);
    }
}

/// Length-and-blend tail of case 6. `p1` is the first adjusted pointer whose
/// bits join the blend as a float.
#[allow(clippy::too_many_arguments)]
unsafe fn fn_00d86b20_c6tail(
    edi: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    p1: u32,
) -> u32 {
    unsafe {
        const TEN: f32 = f32::from_bits(0x41200000);
        const ONE: f32 = f32::from_bits(0x3F800000);
        const TWO: f32 = f32::from_bits(0x40000000);
        const FIVE: f32 = f32::from_bits(0x40A00000);
        const NEG_TENTH: f32 = f32::from_bits(0xBDCCCCCD);
        const TWENTIETH: f32 = f32::from_bits(0x3D4CCCCD);
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn vcall(obj: u32, slot: *mut u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(0xEC)) as usize);
                f(obj, slot as u32)
            }
        }
        let _ = (a1, a4, a5);
        let r: u32 = lf_checker_rt::callee_thiscall!(2, u32, edi.wrapping_add(0xE48));
        let ecx = rd32(r.wrapping_add(0x20));
        let eax = rd32(edi.wrapping_add(0x20));
        let dx = sub(rdf(eax.wrapping_add(0x30)), rdf(ecx.wrapping_add(0x30)));
        let dy = sub(rdf(eax.wrapping_add(0x34)), rdf(ecx.wrapping_add(0x34)));
        let dz = sub(rdf(eax.wrapping_add(0x38)), rdf(ecx.wrapping_add(0x38)));
        let f = rd8(edi.wrapping_add(0xEFA)) as f32;
        let len = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz)).sqrt();
        let f10 = add(f, TEN);
        if !(f10 > len) {
            return 0;
        }
        let mut slot1: u32 = 0;
        let v1 = vcall(r, core::ptr::addr_of_mut!(slot1));
        let l1 = add(mul(rdf(v1), rdf(v1)), mul(rdf(v1.wrapping_add(4)), rdf(v1.wrapping_add(4))))
            .sqrt();
        let mut slot2: u32 = 0;
        let v2 = vcall(edi, core::ptr::addr_of_mut!(slot2));
        let l2 = add(mul(rdf(v2), rdf(v2)), mul(rdf(v2.wrapping_add(4)), rdf(v2.wrapping_add(4))))
            .sqrt();
        let mut t1 = sub(l1, f);
        t1 = if t1 < 0.0 { mul(t1, FIVE) } else { mul(t1, TWO) };
        t1 = add(t1, f32::from_bits(p1));
        t1 = sub(t1, l2);
        if t1 < 0.0 {
            t1 = mul(t1, NEG_TENTH);
            wr32(a2, 0);
            let o = if t1 > ONE { ONE } else { t1 };
            wrf(a3, o);
        } else {
            t1 = mul(t1, TWENTIETH);
            let o = if t1 > ONE { ONE } else { t1 };
            wrf(a2, o);
            wr32(a3, 0);
        }
        wr32(edi.wrapping_add(0xEE0), 0);
        0
    }
}
