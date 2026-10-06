// original: 0x005D7CC0 join_paths_and_rescale (proposed)

/// Joins two path parts, resolves the result, and rescales two stored
/// floats.
///
/// `this` (ECX) is ignored; the single stack argument `obj` is an object
/// (callee pops 4, stdcall). Two lookups on the descriptor at
/// `[obj+0xDC]` (callees 1 and 2, thiscall: descriptor, table constant)
/// answer a source record and a path record.
///
/// When the path record is non-null: its string at `+4` is copied to a
/// frame buffer, a two-byte separator from file VA `SEP_W` is appended,
/// and the source string at `+4` (a null source faults on both sides
/// here) is appended with its terminator; the joined length goes with
/// the buffer to callee 5 (thiscall: `[obj+0xE0]`, buffer, length), the
/// record string feeds callees 8/9/10 (cdecl/1 chain) and callee 7
/// (cdecl/0), and the word at `[obj+0xE4]` (zeroed first, then set by
/// callee 5's scripted outputs) selects either the default table
/// address (file VA `NO_ROW`, relocated) or the dword at `[obj+0xE0]`.
/// Callee 11 (cdecl: that word, `0x3A`) answers 0 (re-read the selector)
/// or is incremented. When the path record is null instead: the global
/// slot at file VA `OCNT_VA` is taken and cleared, a non-null old
/// object has its count at `+0xC` decremented and, on reaching zero, is
/// released through `[obj] -> [vtable+0]` (thiscall: object, 1); a
/// non-null source string goes with its length to callee 6 (thiscall:
/// `[obj+0xE0]`, string, length), and the same selector read picks the
/// word.
///
/// Both paths call the resolver in the planted global slot (cdecl:
/// table address `TABLE_ADDR`, selector word). The resolver
/// answer, when non-null, is queried twice
/// (`[obj] -> vtable[+0x20]` and `[+0x24]`, thiscall/0) and both integer
/// answers are converted exactly to float; otherwise two frame slots
/// keep a float constant from file VA `FCONST_VA`. Two more lookups
/// (callees 3 and 4, thiscall: descriptor, table constant) then drive
/// four float updates with pinned operand order: both non-null stores
/// both converted integers; exactly one null combines the other
/// conversion with `[obj+0x20] * slot14 / slot10` (first null) or
/// `[obj+0x24] * slot10 / slot14` (second null); both null copies the
/// slots. The search for the chained record in the global array (length
/// a zero-extended word, so the signed `jle`/`jl` only ever see
/// non-negative values) appends it via callee 15 (thiscall: array
/// address, `0x10`) when absent. Finally `[obj+0x18]` shifts to
/// `[obj+0x20]` and `[obj+0x1C]` to `[obj+0x24]`; returns `[obj+0x1C]`.
///
/// The trailing security-cookie check runs natively in the original and
/// is not intercepted: it always passes and writes nothing.
///
/// Original: 0x005D7CC0 (stdcall, one stack word, callee pops 4).
lf_checker_rt::export!(stdcall, rw_005D7CC0(obj: u32) -> u32 {
    unsafe {
        const DESC_OFF: u32 = 0xDC;
        const ROW_OFF: u32 = 0xE0;
        const SEP_W: u32 = 0x00F90770;
        const FCONST_VA: u32 = 0x00FE8D94;
        const TABLE_ADDR: u32 = 0x019E8658;
        const ARR_VA: u32 = 0x019E8650;
        const CNT_VA: u32 = 0x019E8654;
        const OCNT_VA: u32 = 0x01BB5554;
        const A1T: u32 = 0x00F9079C;
        const A2T: u32 = 0x00F90798;
        const A3T: u32 = 0x00F90768;
        const A4T: u32 = 0x00F9077C;
        const NO_ROW: u32 = 0x00FC9C85;
        const A1: u32 = 1;
        const A2: u32 = 2;
        const A3: u32 = 3;
        const A4: u32 = 4;
        const L: u32 = 5;
        const S: u32 = 6;
        const P0: u32 = 7;
        const P1: u32 = 8;
        const P2: u32 = 9;
        const P3: u32 = 10;
        const DFC: u32 = 11;
        const IDI: u32 = 12;
        const V20: u32 = 13;
        const V24: u32 = 14;
        const GROW: u32 = 15;
        const REL: u32 = 16;
        const Q: u32 = 17;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        let ebp0 = rd32(obj.wrapping_add(DESC_OFF));
        let esi0 = lf_checker_rt::callee_thiscall!(A1, u32, ebp0, lf_checker_rt::relocated(A1T));
        let ebp = lf_checker_rt::callee_thiscall!(A2, u32, ebp0, lf_checker_rt::relocated(A2T));
        let mut edi: u32;
        if ebp != 0 {
            let s1 = rd32(ebp.wrapping_add(4));
            let s2 = rd32(esi0.wrapping_add(4));
            let mut buf = [0u8; 64];
            let mut i = 0usize;
            loop {
                let b = rd8(s1.wrapping_add(i as u32));
                buf[i] = b;
                if b == 0 {
                    break;
                }
                i = i.wrapping_add(1);
            }
            let sep = rd16(lf_checker_rt::relocated(SEP_W));
            buf[i] = (sep & 0xFF) as u8;
            buf[i.wrapping_add(1)] = (sep >> 8) as u8;
            let mut j = 0usize;
            loop {
                let b = rd8(s2.wrapping_add(j as u32));
                if b == 0 {
                    break;
                }
                j = j.wrapping_add(1);
            }
            let mut k = 0usize;
            while buf[k] != 0 {
                k = k.wrapping_add(1);
            }
            let mut m = 0usize;
            while m <= j {
                buf[k.wrapping_add(m)] = rd8(s2.wrapping_add(m as u32));
                m = m.wrapping_add(1);
            }
            let mut len = 0u32;
            while buf[len as usize] != 0 {
                len = len.wrapping_add(1);
            }
            let dest = obj.wrapping_add(ROW_OFF);
            wr16(dest.wrapping_add(4), 0);
            let _ = lf_checker_rt::callee_thiscall!(L, u32, dest, buf.as_mut_ptr() as u32, len);
            let p1 = lf_checker_rt::callee_cdecl!(P1, u32, s1);
            let esi = lf_checker_rt::callee_cdecl!(P2, u32, p1);
            let _ = lf_checker_rt::callee_cdecl!(P0, u32,);
            let _ = lf_checker_rt::callee_cdecl!(P3, u32, esi);
            let e = if rd16(dest.wrapping_add(4)) == 0 {
                lf_checker_rt::relocated(NO_ROW)
            } else {
                rd32(dest)
            };
            let r = lf_checker_rt::callee_cdecl!(DFC, u32, e, 0x3A);
            let e2 = if r != 0 {
                r.wrapping_add(1)
            } else if rd16(dest.wrapping_add(4)) == 0 {
                lf_checker_rt::relocated(NO_ROW)
            } else {
                rd32(dest)
            };
            edi = lf_checker_rt::callee_cdecl!(IDI, u32, lf_checker_rt::relocated(TABLE_ADDR), e2);
            let _ = lf_checker_rt::callee_cdecl!(Q, u32,);
            let arr = rd32(lf_checker_rt::relocated(ARR_VA));
            let cnt = rd16(lf_checker_rt::relocated(CNT_VA)) as u32;
            let mut cx = 0u32;
            if (cnt as i32) > 0 {
                loop {
                    if rd32(arr.wrapping_add(cx.wrapping_mul(4))) == esi {
                        break;
                    }
                    cx = cx.wrapping_add(1);
                    if !((cx as i32) < (cnt as i32)) {
                        break;
                    }
                }
            }
            if cx == cnt {
                let g = lf_checker_rt::callee_thiscall!(
                    GROW,
                    u32,
                    lf_checker_rt::relocated(ARR_VA),
                    0x10
                );
                wr32(g, esi);
            }
        } else {
            let ocx = rd32(lf_checker_rt::relocated(OCNT_VA));
            wr32(lf_checker_rt::relocated(OCNT_VA), 0);
            if ocx != 0 {
                let rc = rd32(ocx.wrapping_add(0xC));
                wr32(ocx.wrapping_add(0xC), rc.wrapping_sub(1));
                if rc.wrapping_sub(1) == 0 {
                    let vt = rd32(ocx);
                    let rel: extern "thiscall" fn(u32, u32) -> u32 =
                        unsafe { core::mem::transmute(rd32(vt) as usize) };
                    let _ = rel(ocx, 1);
                }
            }
            let edx = rd32(esi0.wrapping_add(4));
            wr16(obj.wrapping_add(0xE4), 0);
            if edx != 0 {
                let mut len = 0u32;
                while rd8(edx.wrapping_add(len)) != 0 {
                    len = len.wrapping_add(1);
                }
                let _ = lf_checker_rt::callee_thiscall!(S, u32, obj.wrapping_add(ROW_OFF), edx, len);
            }
            let e = if rd16(obj.wrapping_add(0xE4)) == 0 {
                lf_checker_rt::relocated(NO_ROW)
            } else {
                rd32(obj.wrapping_add(ROW_OFF))
            };
            edi = lf_checker_rt::callee_cdecl!(IDI, u32, lf_checker_rt::relocated(TABLE_ADDR), e);
        }
        let c0 = f32::from_bits(rd32(lf_checker_rt::relocated(FCONST_VA)));
        let mut f10 = c0;
        let mut f14 = c0;
        if edi != 0 {
            let v20 = lf_checker_rt::callee_thiscall!(V20, u32, edi);
            f10 = (v20 as i32) as f32;
            let v24 = lf_checker_rt::callee_thiscall!(V24, u32, edi);
            f14 = (v24 as i32) as f32;
        }
        let e3 = lf_checker_rt::callee_thiscall!(A3, u32, ebp0, lf_checker_rt::relocated(A3T));
        let e4 = lf_checker_rt::callee_thiscall!(A4, u32, ebp0, lf_checker_rt::relocated(A4T));
        if e3 != 0 {
            wr32(
                obj.wrapping_add(0x18),
                ((rd32(e3.wrapping_add(4)) as i32) as f32).to_bits(),
            );
            if e4 != 0 {
                wr32(
                    obj.wrapping_add(0x1C),
                    ((rd32(e4.wrapping_add(4)) as i32) as f32).to_bits(),
                );
            } else {
                let a = f32::from_bits(rd32(obj.wrapping_add(0x20)));
                wr32(obj.wrapping_add(0x1C), fdiv(fmul(a, f14), f10).to_bits());
            }
        } else if e4 != 0 {
            wr32(
                obj.wrapping_add(0x1C),
                ((rd32(e4.wrapping_add(4)) as i32) as f32).to_bits(),
            );
            let b = f32::from_bits(rd32(obj.wrapping_add(0x24)));
            wr32(obj.wrapping_add(0x18), fdiv(fmul(b, f10), f14).to_bits());
        } else {
            wr32(obj.wrapping_add(0x1C), f14.to_bits());
            wr32(obj.wrapping_add(0x18), f10.to_bits());
        }
        let r18 = rd32(obj.wrapping_add(0x18));
        wr32(obj.wrapping_add(0x20), r18);
        let r1c = rd32(obj.wrapping_add(0x1C));
        wr32(obj.wrapping_add(0x24), r1c);
        r1c
    }
});
