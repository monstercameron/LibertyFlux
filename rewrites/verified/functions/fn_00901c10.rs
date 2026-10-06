// original: 0x00901C10 input_ui_build_elements (proposed)

/// Build five UI/overlay element objects, sample two global data sources,
/// then sweep a 1500-entry global pointer table building one element per
/// live row.
///
/// Arguments: none (cdecl/0, plain `ret`). All entry registers are dead.
/// Returns in EAX the last allocator/helper answer on the path taken (the
/// final bit-fold value when the table loop is skipped).
///
/// Layout. Each element object starts with a vtable pointer (+0), an id
/// word (+4, partly generated here), a handler pointer (+8) and a small
/// field (+0xC/0x10). The table at TABLE holds one pointer per row; a live
/// row points at a record with a flag byte at +8, a bitfield word at +0x20
/// (bit 10 gates the row, bit 1 of the low byte feeds the row call) and a
/// parameter dword at +0x54.
///
/// Algorithm. Three blocks allocate a small object (sizes 0xC, 0x14, 0x10),
/// stamp the low 14 bits of its id word with a global counter (masked
/// 0x3FFF, counter incremented), plant a vtable, then run the id-fold pair:
/// two virtual calls through slot +8 whose SIGNED-remainder answers
/// (`% 16`, C semantics) pick a shift of a SIGNED quotient (`/ 16`) that is
/// folded into bits 14..24 of the id word. Two more blocks allocate 0x30
/// bytes and run a six-argument constructor (four float-block pointers, a
/// colour word, a zero) before the same fold pair. The middle float section
/// reads a flag byte from a first-megabyte helper, two floats from heap
/// answers, combines them with five read-only constants and feeds block 5.
/// The gated loop (`[GATE] == 2`) walks rows 0..0x5DC (SIGNED `< 0x5DC`;
/// the row equal to COUNT is skipped, a null row is skipped, a row whose
/// flag byte is clear ENDS the loop): per row it calls a sampler, does the
/// float chain, calls a row helper whose out-word becomes the row colour,
/// constructs an element and hands it to a sink helper. Every other
/// comparison is an equality or nonzero test, except the SIGNED `% 16` and
/// `/ 16` in the fold (see above).
///
/// Edge cases: a null allocator answer faults on the vtable load (same
/// fault both sides); a null row-helper answer reuses the stored float; the
/// flag-byte tests after the loop guards are dead (the guard already broke
/// out) but implemented as written; one store to the out-word slot after it
/// is consumed is dead and omitted (below-ESP scratch, never observed).
///
/// Original: 0x00901C10 (cdecl, no stack arguments).
/// Shared body: `signed_fold = true` is the faithful rewrite (SIGNED % 16
/// and / 16 in the id-fold); `false` is the deliberately wrong version
/// (the same operations UNSIGNED) used only as the checker's mutant.
unsafe fn body(signed_fold: bool) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0118_F6F8;
        const GATE: u32 = 0x011D_6FD4;
        const COUNT: u32 = 0x0103_4494;
        const CTR: u32 = 0x0103_27A0;
        const DATA_F0: u32 = 0x0103_44C8;
        const DATA_F1: u32 = 0x0103_44CC;
        const VT_BASE: u32 = 0x00E7_E048;
        const VT_B1: u32 = 0x00E7_E080;
        const VT_B2: u32 = 0x00E8_4C5C;
        const VT_B3: u32 = 0x00E8_4C78;
        const D_B1: u32 = 0x0043_2BE0;
        const D_B2: u32 = 0x0090_3750;
        const D_B3: u32 = 0x0059_D8B0;
        // Read-only float constants (file values, verified against the image).
        const C_EC: f32 = f32::from_bits(0x3EFE_425B); // 0xE84CEC
        const C_E8: f32 = f32::from_bits(0x3EFC_6A7F); // 0xE84CE8
        const C_C8: f32 = f32::from_bits(0x3AA3_D70A); // 0xE84CC8
        const C_30: f32 = f32::from_bits(0x3F00_0000); // 0xFE8830 (0.5)
        const C_BC: f32 = f32::from_bits(0x3F66_6666); // 0xFE88BC (0.9)
        const C_98: f32 = f32::from_bits(0x3F4C_CCCD); // 0xFE8898 (0.8)
        const C_E8B: f32 = f32::from_bits(0x3F80_0000); // 0xFE88E8 (1.0)
        const C_425B: f32 = f32::from_bits(0x3EFE_425B);
        const C_6666: f32 = f32::from_bits(0x3F66_6666);
        const C_CCCD: f32 = f32::from_bits(0x3DCC_CCCD);
        const C_DED2: f32 = f32::from_bits(0x3F00_DED2);
        const C_999A: f32 = f32::from_bits(0x3E99_999A);
        const C_3333: f32 = f32::from_bits(0x3F33_3333);
        const C_E666: f32 = f32::from_bits(0x3EE6_6666);
        const C_0CCD: f32 = f32::from_bits(0x3F0C_CCCD);
        const C_NEW: u32 = 1;
        const C_CTOR: u32 = 2;
        const C_FMB0: u32 = 3;
        const C_G50: u32 = 4;
        const C_G70: u32 = 5;
        const C_G1C0: u32 = 6;
        const C_M590: u32 = 7;
        const C_PFD0: u32 = 8;
        const C_N9E30: u32 = 9;
        const C_GS: u32 = 10;
        const ROWS: u32 = 0x5DC;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        /// Virtual call through slot +8 (thiscall, no stack args), exactly
        /// like the original: faults identically on a null object.
        #[inline(always)]
        unsafe fn vcall(obj: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(8)) as usize);
                f(obj)
            }
        }
        /// The id-fold pair: two slot-+8 calls, SIGNED % 16 / / 16, fold of
        /// bits 14..24 into [obj+4]. Returns the folded bits (EAX after).
        #[inline(always)]
        unsafe fn fold_pair(obj: u32, signed: bool) -> u32 {
            unsafe {
                let r1 = vcall(obj);
                let a = if signed {
                    (16i32.wrapping_sub((r1 as i32).wrapping_rem(16))).wrapping_rem(16)
                } else {
                    (16u32.wrapping_sub(r1.wrapping_rem(16)).wrapping_rem(16)) as i32
                };
                let r2 = vcall(obj);
                let q = if signed {
                    (r2 as i32).wrapping_add(a).wrapping_div(16)
                } else {
                    r2.wrapping_add(a as u32).wrapping_div(16) as i32
                };
                let m = rd32(obj.wrapping_add(4));
                let bits = ((q as u32) << 14 ^ m) & 0x01FF_C000;
                wr32(obj.wrapping_add(4), m ^ bits);
                bits
            }
        }
        /// Stamp the low 14 id bits from the global counter, then bump it.
        #[inline(always)]
        unsafe fn stamp(obj: u32) {
            unsafe {
                wr32(obj, lf_checker_rt::relocated(VT_BASE));
                let m = rd32(obj.wrapping_add(4));
                let ctr = glob(CTR);
                let c = (m ^ ctr) & 0x3FFF;
                wr32(obj.wrapping_add(4), m ^ c);
                glob_mut(CTR).write_unaligned(ctr.wrapping_add(1));
            }
        }
        #[inline(always)]
        unsafe fn glob(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn glob_mut(va: u32) -> *mut u32 {
            lf_checker_rt::relocated(va) as *mut u32
        }
        #[inline(always)]
        unsafe fn new_obj(size: u32) -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(C_NEW, u32, size, 0) }
        }
        #[inline(always)]
        unsafe fn ctor(obj: u32, s: &[u32; 8], colour: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_thiscall!(C_CTOR, u32, obj,
                    s.as_ptr().add(0) as u32, s.as_ptr().add(2) as u32,
                    s.as_ptr().add(4) as u32, s.as_ptr().add(6) as u32,
                    colour, 0)
            }
        }

        // Block 1: 0xC object, vtable B1.
        let mut ret: u32;
        let o1 = new_obj(0x0C);
        if o1 != 0 {
            stamp(o1);
            wr32(o1, lf_checker_rt::relocated(VT_B1));
            wr32(o1.wrapping_add(8), lf_checker_rt::relocated(D_B1));
        }
        ret = fold_pair(o1, signed_fold);
        // Block 2: 0x14 object, vtable B2, two global floats.
        let o2 = new_obj(0x14);
        if o2 != 0 {
            stamp(o2);
            wr32(o2, lf_checker_rt::relocated(VT_B2));
            wr32(o2.wrapping_add(8), lf_checker_rt::relocated(D_B2));
            wr32(o2.wrapping_add(0x0C), glob(DATA_F0));
            wr32(o2.wrapping_add(0x10), glob(DATA_F1));
        }
        ret = fold_pair(o2, signed_fold);
        // Block 3: 0x10 object, vtable B3, constant 6.
        let o3 = new_obj(0x10);
        if o3 != 0 {
            stamp(o3);
            wr32(o3, lf_checker_rt::relocated(VT_B3));
            wr32(o3.wrapping_add(8), lf_checker_rt::relocated(D_B3));
            wr32(o3.wrapping_add(0x0C), 6);
        }
        ret = fold_pair(o3, signed_fold);
        // Block 4: constant float block + constructor.
        let s4: [u32; 8] = [C_425B.to_bits(), C_6666.to_bits(), C_425B.to_bits(),
            C_CCCD.to_bits(), C_DED2.to_bits(), C_6666.to_bits(),
            C_DED2.to_bits(), C_CCCD.to_bits()];
        let o4 = new_obj(0x30);
        if o4 != 0 {
            let c4 = ctor(o4, &s4, 0xFF00_0000);
            ret = fold_pair(c4, signed_fold);
        } else {
            ret = fold_pair(o4, signed_fold);
        }
        // Middle float section.
        let s14 = if lf_checker_rt::callee_cdecl!(C_FMB0, u32,) as u8 != 0 { C_EC } else { C_E8 };
        let mut zeroed: u32 = 0;
        let g50 = lf_checker_rt::callee_cdecl!(C_G50, u32,);
        let mut x2: f32;
        if g50 != 0 {
            let r70 = lf_checker_rt::callee_cdecl!(C_G70, u32, &mut zeroed as *mut u32 as u32);
            let f070 = rdf(r70.wrapping_add(8));
            let r1c0 = lf_checker_rt::callee_cdecl!(C_G1C0, u32, 0, &mut zeroed as *mut u32 as u32);
            let src = if r1c0 != 0 {
                rdf(rd32(r1c0.wrapping_add(0x20)).wrapping_add(0x38))
            } else {
                f070
            };
            let mut t = mul(src, C_C8);
            t = mul(t, C_98);
            x2 = sub(C_BC, t);
            x2 = sub(x2, C_30);
        } else {
            // Both helper calls are skipped; the slot still holds C_30.
            x2 = C_30;
        }
        let mut x1 = add(x2, C_E8B);
        x2 = add(x2, s14);
        x1 = sub(x1, s14);
        // Block 5: computed float block + constructor.
        let s5: [u32; 8] = [C_999A.to_bits(), x1.to_bits(), C_999A.to_bits(),
            x2.to_bits(), C_3333.to_bits(), x1.to_bits(),
            C_3333.to_bits(), x2.to_bits()];
        let o5 = new_obj(0x30);
        if o5 != 0 {
            let c5 = ctor(o5, &s5, 0xFF00_0000);
            ret = fold_pair(c5, signed_fold);
        } else {
            ret = fold_pair(o5, signed_fold);
        }
        // Table loop.
        if glob(GATE) == 2 {
            let table = lf_checker_rt::relocated(TABLE);
            // Persistent frame slots across iterations: the sampler struct
            // (uninitialized -> fill 0) and the row-helper out-word slot,
            // which still holds block 5's last store (0x3f333333).
            let mut mbuf: [u32; 3] = [0, 0, 0];
            let mut s0c: u32 = 0x3F33_3333;
            let mut count = glob(COUNT);
            let mut idx: u32 = 0;
            while (idx as i32) < (ROWS as i32) {
                if idx != count {
                    let row = rd32(table.wrapping_add(idx.wrapping_mul(4)));
                    if row != 0 {
                        let flag = (row.wrapping_add(8) as *const u8).read();
                        if flag == 0 {
                            break;
                        }
                        let bf = (row.wrapping_add(0x20) as *const u16).read_unaligned() as u32;
                        // The guard loads AX and shifts/masks it, clobbering
                        // EAX's low word even on rows that continue.
                        let ax = (bf & 0xFFFF) >> 10;
                        let new_ax = (ax & 0xFF00) | ((ax & 0xFF) & 1);
                        ret = (ret & 0xFFFF0000) | (new_ax & 0xFFFF);
                        if (bf >> 10) & 1 != 0 {
                            lf_checker_rt::callee_cdecl!(C_M590, u32,
                                mbuf.as_mut_ptr() as u32, idx);
                            let mut f = f32::from_bits(mbuf[2]);
                            f = mul(f, C_C8);
                            f = mul(f, C_98);
                            let mut y1 = sub(C_BC, f);
                            y1 = sub(y1, C_30);
                            let mut y0 = y1;
                            y0 = add(y0, C_E8B);
                            y1 = add(y1, s14);
                            y0 = sub(y0, s14);
                            let s: [u32; 8] = [C_E666.to_bits(), y0.to_bits(),
                                C_E666.to_bits(), y1.to_bits(), C_0CCD.to_bits(),
                                y0.to_bits(), C_0CCD.to_bits(), y1.to_bits()];
                            let dl = (row.wrapping_add(8) as *const u8).read();
                            let bl: u8 = if dl != 0 {
                                (row.wrapping_add(0x20) as *const u8).read()
                            } else {
                                let r2row = rd32(table.wrapping_add(count.wrapping_mul(4)));
                                (r2row.wrapping_add(0x20) as *const u8).read()
                            };
                            let al = (bl >> 1) & 1;
                            let a1: u32 = if dl != 0 {
                                rd32(row.wrapping_add(0x54))
                            } else {
                                let r2row = rd32(table.wrapping_add(count.wrapping_mul(4)));
                                rd32(r2row.wrapping_add(0x54))
                            };
                            let mut outw: u32 = s0c;
                            lf_checker_rt::callee_cdecl!(C_PFD0, u32,
                                &mut outw as *mut u32 as u32, a1, al as u32, 0);
                            let colour = outw | 0xFF00_0000;
                            s0c = colour;
                            let o6 = new_obj(0x30);
                            let co = if o6 != 0 { ctor(o6, &s, colour) } else { 0 };
                            ret = lf_checker_rt::callee_cdecl!(C_N9E30, u32, co);
                            count = glob(COUNT);
                        }
                    }
                }
                idx = idx.wrapping_add(1);
            }
        }
        lf_checker_rt::callee_cdecl!(C_GS, u32,);
        ret
    }
}

lf_checker_rt::export!(cdecl, rw_00901C10() -> u32 {
    unsafe { body(true) }
});
