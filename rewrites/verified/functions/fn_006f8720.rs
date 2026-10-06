// original: 0x006f8720 ros_cloud_file_changed (proposed)

/// Dispatch a cloud-file change notification to a list of watchers.
///
/// `arg0` is the request object. Behaviour: a gate call through the
/// request's own table (slot 0, stubbed as callee 1) must answer 14 or the
/// answer itself is returned. A probe call (callee 2, file address 0x6ee110)
/// selects the main path (nonzero answer) or the fallback path (zero).
/// The main path runs a chain of validation and formatting calls (callees
/// 3-8, file addresses 0x6ed420, 0x67aff0, 0x67b1e0, 0x67bd00, 0x6eb8a0,
/// 0xdfd3b7); any zero answer (or an answer other than 1 from the last)
/// returns that answer at once. The fallback path runs two calls (callees
/// 9-10, file addresses 0x6f9690, 0x6f7d90) with the same early-exit rule.
/// Both paths then walk the watcher list whose head is the writable global
/// at file address 0x1a05728: an empty list returns -1. Each node whose
/// word at `+0x08` is zero and whose enable bit (bit 2 of the byte at file
/// address 0x19f32ac) is set has its name at `+0x10` measured and compared
/// against the scratch name through callee 11 (file address 0x671e20); the
/// returned pointer is checked against the scratch address it was derived
/// from. Nodes otherwise advance to the link at `+0x110`. The walk returns
/// the last marker word, bit-test zero, or compare answer seen; only the
/// empty list returns -1.
///
/// The per-node match body past the pointer check is transcribed but never
/// reached in the proof: the check compares a single scripted answer against
/// a stack address, which differs between the two sides' frames, so only the
/// advance branch is satisfiable on both sides at once (the scripted answers
/// are small integers no stack address can equal). Its callee references use
/// undeclared ids, so any surprise reaching it faults loudly instead of
/// passing quietly.
///
/// Original: 0x006f8720 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_006f8720(arg0: u32) -> u32 {
    unsafe {
        const G_HEAD: u32 = 0x01a05728;
        const G_ENBIT: u32 = 0x019f32ac;
        const G_FALLBACK: u32 = 0x011104ac;
        const P_TAG: u32 = 0x00fe50e8;
        const P_FMT: u32 = 0x00fae24c;
        const P_ALT0: u32 = 0x00fae0f8;
        const P_ALT1: u32 = 0x00fae23c;
        const P_ALT2: u32 = 0x00fae228;
        const P_EMPTY: u32 = 0x00f1c38b;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn strlen(p: u32) -> u32 {
            let mut q = p;
            while unsafe { rd8(q) } != 0 {
                q = q.wrapping_add(1);
            }
            q.wrapping_sub(p)
        }

        // Scratch slots standing in for the original's aligned frame.
        let mut q18: [u32; 2] = [0; 2];
        let mut buf60: [u32; 64] = [0; 64];
        let mut buf20: [u32; 16] = [0; 16];
        let mut buf360: [u32; 16] = [0; 16];
        let mut buf760: [u32; 8] = [0; 8];
        let mut s10: u32 = 0;
        let mut s14: u32 = 0;
        let mut s20b: u8 = 0;

        let esi = arg0;
        // Gate call through the request's own table, slot 0.
        let vtab = rd32(esi);
        let gate = rd32(vtab);
        let gf: unsafe extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(gate as usize) };
        let g = unsafe { gf(esi) };
        if g != 14 {
            return g;
        }
        let aux = rd32(esi.wrapping_add(0x80));
        q18[0] = 0;
        q18[1] = 0;
        let probe: u32 = lf_checker_rt::callee_thiscall!(2, u32, aux);
        if (probe as u8) != 0 {
            wr8(buf360.as_mut_ptr() as u32, 0);
            wr8(buf760.as_mut_ptr() as u32, 0);
            let b360 = buf360.as_mut_ptr() as u32;
            let r2: u32 = lf_checker_rt::callee_thiscall!(3, u32, b360, aux);
            if (r2 as u8) == 0 {
                return r2;
            }
            let b20 = buf20.as_mut_ptr() as u32;
            let _r3: u32 =
                lf_checker_rt::callee_thiscall!(4, u32, b20, b360, b360, 0x400u32);
            let r4: u32 = lf_checker_rt::callee_thiscall!(5, u32, b20, b20);
            if (r4 as u8) == 0 {
                return r4;
            }
            let r5: u32 = lf_checker_rt::callee_thiscall!(
                6,
                u32,
                b20,
                lf_checker_rt::relocated(P_TAG),
                b20
            );
            if (r5 as u8) == 0 {
                return r5;
            }
            let b60 = buf60.as_mut_ptr() as u32;
            let r6: u32 = lf_checker_rt::callee_thiscall!(7, u32, b60, b20);
            if (r6 as u8) == 0 {
                return r6;
            }
            let b760 = buf760.as_mut_ptr() as u32;
            let q18p = q18.as_mut_ptr() as u32;
            let r7: u32 = lf_checker_rt::callee_cdecl!(
                8,
                u32,
                b760,
                lf_checker_rt::relocated(P_FMT),
                q18p
            );
            if r7 != 1 {
                return r7;
            }
        } else {
            let r8: u32 = lf_checker_rt::callee_thiscall!(9, u32, aux);
            if (r8 as u8) == 0 {
                return r8;
            }
            let b60 = buf60.as_mut_ptr() as u32;
            let r9: u32 = lf_checker_rt::callee_thiscall!(10, u32, b60, aux);
            if (r9 as u8) == 0 {
                return r9;
            }
        }

        let mut node = rd32(lf_checker_rt::relocated(G_HEAD));
        s10 = 0xffffffff;
        s14 = 0xffffffff;
        s20b = 0;
        // eax threads through the walk: the marker word, the cleared bit
        // test, or the compare call's answer is what a later list end
        // returns; only the empty list returns the initial -1.
        let mut eax: u32 = 0xffffffff;
        if node == 0 {
            return eax;
        }
        loop {
            let w8 = rd32(node.wrapping_add(8));
            if w8 != 0 {
                eax = w8;
            } else {
                let en = rd8(lf_checker_rt::relocated(G_ENBIT));
                if (en >> 2) & 1 == 0 {
                    eax = 0;
                } else {
                    let b60 = buf60.as_mut_ptr() as u32;
                    let name = node.wrapping_add(0x10);
                    let len1 = strlen(b60);
                    let len2 = strlen(name);
                    let got: u32 = lf_checker_rt::callee_cdecl!(11, u32, name);
                    eax = got;
                    let want = b60.wrapping_add(len1.wrapping_sub(len2));
                    if got == want {
                        // Match body (unreached in the proof; see doc comment).
                        let _ = match_body(
                            node, b60, q18.as_mut_ptr() as u32, len1, len2,
                            &mut s10, &mut s14, &mut s20b,
                            G_FALLBACK, P_ALT0, P_ALT1, P_ALT2, P_EMPTY,
                        );
                    }
                }
            }
            node = rd32(node.wrapping_add(0x110));
            if node == 0 {
                break;
            }
        }
        eax
    }
});

/// Transcription of the per-node match body. Never reached in the proof
/// (see above); its callee ids are deliberately undeclared so any surprise
/// reaching it faults loudly.
#[allow(clippy::too_many_arguments)]
unsafe fn match_body(
    node: u32, b60: u32, q18p: u32, len1: u32, len2: u32,
    s10: &mut u32, s14: &mut u32, s20b: &mut u8,
    g_fb: u32, p0: u32, p1: u32, p2: u32, p_empty: u32,
) -> u32 {
    unsafe {
        let mut buf40: [u32; 8] = [0; 8];
        let mut buf1e0: [u32; 8] = [0; 8];
        let mut s0c: u32 = 0;
        let _ = (len1, len2);
        let edx = rd32(node.wrapping_add(0x120));
        let orq = edx | rd32(node.wrapping_add(0x124));
        let cl: u8 = if orq != 0 { 1 } else { 0 };
        let b0 = rd32(q18p);
        let b1 = rd32(q18p.wrapping_add(4));
        let al: u8 = if (b0 | b1) != 0 { 1 } else { 0 };
        if cl != al {
            return 1;
        }
        if cl != 0 {
            if edx != b0 {
                return 1;
            }
            if rd32(node.wrapping_add(0x124)) != b1 {
                return 1;
            }
        }
        let kind = rd32(node.wrapping_add(0x0c));
        if kind != 1 && kind != 0 {
            s0c = rd32(lf_checker_rt::relocated(g_fb));
        } else {
            s0c = lf_checker_rt::relocated(p0);
        }
        if (b0 | b1) != 0 {
            let b40 = buf40.as_mut_ptr() as u32;
            let _r: u32 =
                lf_checker_rt::callee_thiscall!(12, u32, b40, b1, b0);
            let s40b = ((b40.wrapping_add(0)) as *const u8).read_unaligned();
            let s40p = b40;
            let strp: u32 = if s40b == 0 {
                lf_checker_rt::relocated(p_empty)
            } else {
                s40p
            };
            let edi = node.wrapping_add(0x10);
            let b1e0 = buf1e0.as_mut_ptr() as u32;
            let _c: u32 = lf_checker_rt::callee_cdecl!(
                13,
                u32,
                b1e0,
                lf_checker_rt::relocated(p1),
                s0c,
                strp,
                edi
            );
        } else {
            let ecx = rd32(node.wrapping_add(8));
            if ecx != *s10 || kind != *s14 {
                let s20p = (s20b as *mut u8) as u32;
                let ok: u32 = lf_checker_rt::callee_cdecl!(14, u32, s20p);
                if (ok as u8) == 0 {
                    return 1;
                }
                *s10 = rd32(node.wrapping_add(8));
                *s14 = rd32(node.wrapping_add(0x0c));
            }
            let strp: u32 = if *s20b == 0 {
                lf_checker_rt::relocated(p_empty)
            } else {
                (s20b as *mut u8) as u32
            };
            let edi = node.wrapping_add(0x10);
            let b1e0 = buf1e0.as_mut_ptr() as u32;
            let _c: u32 = lf_checker_rt::callee_cdecl!(
                13,
                u32,
                b1e0,
                lf_checker_rt::relocated(p2),
                s0c,
                strp,
                edi
            );
        }
        let b1e0 = buf1e0.as_mut_ptr() as u32;
        let ok2: u32 = lf_checker_rt::callee_thiscall!(15, u32, b60);
        let _ = (b1e0, ok2);
        if (ok2 as u8) == 0 {
            return 1;
        }
        let tgt = rd32(node.wrapping_add(4));
        let f: unsafe extern "cdecl" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(tgt as usize) };
        let _ = unsafe { f(node.wrapping_add(0x10), b1e0) };
        0
    }
}

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
