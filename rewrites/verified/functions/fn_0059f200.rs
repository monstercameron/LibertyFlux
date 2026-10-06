// original: 0x0059F200 memsize_apply (proposed)
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global};

/// Apply the memory-class limits: reconcile the two limit tables, publish
/// derived floats, and return the applied float.
///
/// `arg` (ECX) contributes only its LOW BYTE (`bl`): zero skips the final
/// memory-sizing block. Two 14-word tables are reconciled: `B` (live limits
/// at `BTAB`, shared with the sibling setters) against `A` (saved limits at
/// `ATAB`).
///
/// The estimator callee first fills a float `F` through a scratch slot (the
/// filler callee's four outputs are never read back). When the mode global
/// `G0` is nonzero the tables are copied back directly. Otherwise `F` is
/// compared against 1.25 with `comiss`+`jb`: an unordered (NaN) `F` takes
/// the below-branch too, so the test is `!(F >= 1.25)`.
///
/// - At-or-above branch: the first scan looks for the first index with
///   `(B[i] as i32) > (A[i] as i32)` (SIGNED); found goes to the apply path,
///   completed falls through to the second scan.
/// - Below branch: only the second scan runs; found calls the checker
///   callee (u64 answer): a nonzero high word, or a low word at or above
///   `MEM_BOUND` (UNSIGNED; the `jb` after the high-word test is dead --
///   `test` clears CF), goes to copy-back, else the sibling outputs decide
///   (SIGNED): `B[3] > 0` applies, else `B[5] > 0` applies, else copy-back.
/// - The apply path copies `A` over `B`, calls the notify callee with 0,
///   and continues with the new `B` values. The copy-back path copies `B`
///   over `A` and stores `F` to the applied-float slot `AFLT`.
///
/// Then derived values publish (SIGNED int-to-float conversions, pinned
/// float order): `1 << (B[4] & 31)`; `((B[5])*C1)*C2+C3`; the notify-float
/// `((B[6])*C1)*100+10` passed to the second notify callee (the pushed slot
/// first holds `B[4]` and is overwritten with the float -- unobservable
/// scratch); `((B[7])*C1)*3+0.5` plus the flag byte `(B[8] != 0)`;
/// `((B[6])*C1)*3.5+0.5`.
///
/// With nonzero `bl`, RAM sizing like the siblings runs (cached pair,
/// GlobalMemoryStatusEx refresh, UNSIGNED bound): small memory clears the
/// memory output, otherwise (and on query failure) it takes `B[3]`.
/// The return, on every path, is the applied-float slot in XMM0 -- on the
/// apply path that slot still holds its previous contents (the copy-back
/// store is skipped). EAX on return (a bound OR or the query answer) is not
/// a compared channel.
///
/// Original: 0x0059F200 (thiscall; only the low byte of ECX is read).
export!(thiscall, rw_0059f200(arg: u32) -> f32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    #[inline(always)]
    fn fmul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    #[inline(always)]
    fn fadd(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    /// First index with `(B[i] as i32) > (A[i] as i32)` (the original's
    /// SIGNED `jg` scan over 14 words).
    #[inline(always)]
    unsafe fn scan_gt(b: u32, a: u32) -> bool {
        unsafe {
            let mut i = 0u32;
            while i < 14 {
                if (rd32(b.wrapping_add(i * 4)) as i32) > (rd32(a.wrapping_add(i * 4)) as i32) {
                    return true;
                }
                i += 1;
            }
            false
        }
    }
    /// Return `bits` as f32 through XMM0 (upper lanes zeroed), matching the
    /// original's `(an instruction of the original)`; the built export was disassembled to
    /// confirm (`movss` then `fld`).
    #[inline(always)]
    fn return_f32_bits(bits: u32) -> f32 {
        unsafe {
            let v = core::arch::x86::_mm_load_ss(&bits as *const u32 as *const f32);
            let f = core::arch::x86::_mm_cvtss_f32(v);
            core::hint::black_box(v);
            f
        }
    }
    unsafe {
        const MEM_BOUND: u32 = 0x5999_9980;
        const CACHED_LO: u32 = 0x01BB_5500;
        const CACHED_HI: u32 = 0x01BB_5504;
        const STATUS_LEN: u32 = 0x40;
        const BTAB: u32 = 0x0116_0E80;
        const ATAB: u32 = 0x018B_6DBC;
        const AFLT: u32 = 0x018B_6E90;
        const G0: u32 = 0x0110_E6BC;
        const SHLOUT: u32 = 0x0106_B3DC;
        const FOUT_A: u32 = 0x0118_D838;
        const FOUT_C: u32 = 0x0103_FFA8;
        const FOUT_D: u32 = 0x0104_8230;
        const FLAGOUT: u32 = 0x011A_28F0;
        const MEMOUT: u32 = 0x0106_B56C;
        const C_CMP: u32 = 0x00FE_8920;
        const C_A: u32 = 0x00FE_8714;
        const C_B: u32 = 0x00FE_8924;
        const C_C: u32 = 0x00FE_8888;
        const C_D: u32 = 0x00FE_8BB0;
        const C_E: u32 = 0x00FE_8B08;
        const C_F: u32 = 0x00FE_8A94;
        const C_G: u32 = 0x00FE_8830;
        const C_H: u32 = 0x00FE_8AB0;
        const FILL_CALLEE: u32 = 1;
        const MEM_QUERY_CALLEE: u32 = 2;
        const ESTIMATE_CALLEE: u32 = 3;
        const NOTIFY_CALLEE: u32 = 4;
        const CHECK_CALLEE: u32 = 5;
        const NOTIFY2_CALLEE: u32 = 6;

        let bb = global::<u32>(BTAB) as u32;
        let aa = global::<u32>(ATAB) as u32;
        let mut w = [0u32; 4];
        let wptr = w.as_mut_ptr() as u32;
        let _ = callee_cdecl!(
            FILL_CALLEE, u32, wptr, wptr.wrapping_add(4), wptr.wrapping_add(8), wptr.wrapping_add(12));
        let mut fslot = 0u32;
        let _ = callee_thiscall!(ESTIMATE_CALLEE, u32, &mut fslot as *mut u32 as u32);
        let f = f32::from_bits(fslot);
        let cthr = f32::from_bits(global::<u32>(C_CMP).read());
        let g0 = global::<u32>(G0).read();
        // Reconcile the tables.
        if g0 != 0 {
            let mut i = 0u32;
            while i < 14 {
                wr32(aa.wrapping_add(i * 4), rd32(bb.wrapping_add(i * 4)));
                i += 1;
            }
            wr32(global::<u32>(AFLT) as u32, fslot);
        } else if !(f >= cthr) {
            if scan_gt(bb, aa) {
                let ans = callee_cdecl!(CHECK_CALLEE, u64,);
                let alo = ans as u32;
                let ahi = (ans >> 32) as u32;
                if ahi != 0 || alo >= MEM_BOUND {
                    let mut i = 0u32;
                    while i < 14 {
                        wr32(aa.wrapping_add(i * 4), rd32(bb.wrapping_add(i * 4)));
                        i += 1;
                    }
                    wr32(global::<u32>(AFLT) as u32, fslot);
                        } else if (rd32(bb.wrapping_add(12)) as i32) > 0 {
                    let mut i = 0u32;
                    while i < 14 {
                        wr32(bb.wrapping_add(i * 4), rd32(aa.wrapping_add(i * 4)));
                        i += 1;
                    }
                    let _ = callee_cdecl!(NOTIFY_CALLEE, u32, 0);
                        } else if (rd32(bb.wrapping_add(20)) as i32) > 0 {
                    let mut i = 0u32;
                    while i < 14 {
                        wr32(bb.wrapping_add(i * 4), rd32(aa.wrapping_add(i * 4)));
                        i += 1;
                    }
                    let _ = callee_cdecl!(NOTIFY_CALLEE, u32, 0);
                        } else {
                    let mut i = 0u32;
                    while i < 14 {
                        wr32(aa.wrapping_add(i * 4), rd32(bb.wrapping_add(i * 4)));
                        i += 1;
                    }
                    wr32(global::<u32>(AFLT) as u32, fslot);
                        }
            } else {
                let mut i = 0u32;
                while i < 14 {
                    wr32(aa.wrapping_add(i * 4), rd32(bb.wrapping_add(i * 4)));
                    i += 1;
                }
                wr32(global::<u32>(AFLT) as u32, fslot);
                }
        } else if scan_gt(bb, aa) {
            let mut i = 0u32;
            while i < 14 {
                wr32(bb.wrapping_add(i * 4), rd32(aa.wrapping_add(i * 4)));
                i += 1;
            }
            let _ = callee_cdecl!(NOTIFY_CALLEE, u32, 0);
        } else if scan_gt(bb, aa) {
            let ans = callee_cdecl!(CHECK_CALLEE, u64,);
            let alo = ans as u32;
            let ahi = (ans >> 32) as u32;
            if ahi != 0 || alo >= MEM_BOUND {
                let mut i = 0u32;
                while i < 14 {
                    wr32(aa.wrapping_add(i * 4), rd32(bb.wrapping_add(i * 4)));
                    i += 1;
                }
                wr32(global::<u32>(AFLT) as u32, fslot);
                } else if (rd32(bb.wrapping_add(12)) as i32) > 0 {
                let mut i = 0u32;
                while i < 14 {
                    wr32(bb.wrapping_add(i * 4), rd32(aa.wrapping_add(i * 4)));
                    i += 1;
                }
                let _ = callee_cdecl!(NOTIFY_CALLEE, u32, 0);
                } else if (rd32(bb.wrapping_add(20)) as i32) > 0 {
                let mut i = 0u32;
                while i < 14 {
                    wr32(bb.wrapping_add(i * 4), rd32(aa.wrapping_add(i * 4)));
                    i += 1;
                }
                let _ = callee_cdecl!(NOTIFY_CALLEE, u32, 0);
                } else {
                let mut i = 0u32;
                while i < 14 {
                    wr32(aa.wrapping_add(i * 4), rd32(bb.wrapping_add(i * 4)));
                    i += 1;
                }
                wr32(global::<u32>(AFLT) as u32, fslot);
                }
        } else {
            let mut i = 0u32;
            while i < 14 {
                wr32(aa.wrapping_add(i * 4), rd32(bb.wrapping_add(i * 4)));
                i += 1;
            }
            wr32(global::<u32>(AFLT) as u32, fslot);
        }
        let b4 = rd32(bb.wrapping_add(16));
        let b5 = rd32(bb.wrapping_add(20));
        let b6 = rd32(bb.wrapping_add(24));
        let b7 = rd32(bb.wrapping_add(28));
        let b8 = rd32(bb.wrapping_add(32));
        global::<u32>(SHLOUT).write(1u32.wrapping_shl(b4));
        let ca = f32::from_bits(global::<u32>(C_A).read());
        let fa = fadd(fmul(fmul((b5 as i32) as f32, ca), f32::from_bits(global::<u32>(C_B).read())), f32::from_bits(global::<u32>(C_C).read()));
        global::<u32>(FOUT_A).write(fa.to_bits());
        let fb = fadd(fmul(fmul((b6 as i32) as f32, ca), f32::from_bits(global::<u32>(C_D).read())), f32::from_bits(global::<u32>(C_E).read()));
        let _ = callee_cdecl!(NOTIFY2_CALLEE, u32, fb.to_bits());
        let fc = fadd(fmul(fmul((b7 as i32) as f32, ca), f32::from_bits(global::<u32>(C_F).read())), f32::from_bits(global::<u32>(C_G).read()));
        global::<u32>(FOUT_C).write(fc.to_bits());
        global::<u8>(FLAGOUT).write(if b8 != 0 { 1 } else { 0 });
        let fd = fadd(fmul(fmul((b6 as i32) as f32, ca), f32::from_bits(global::<u32>(C_H).read())), f32::from_bits(global::<u32>(C_G).read()));
        global::<u32>(FOUT_D).write(fd.to_bits());
        if (arg as u8) != 0 {
            let mut lo = global::<u32>(CACHED_LO).read();
            let mut hi = global::<u32>(CACHED_HI).read();
            if (lo | hi) == 0 {
                let mut status = [0u32; 16];
                status[0] = STATUS_LEN;
                let ok = callee_stdcall!(MEM_QUERY_CALLEE, u32, status.as_mut_ptr() as u32);
                if ok != 0 {
                    lo = status[2];
                    hi = status[3];
                    global::<u32>(CACHED_LO).write(lo);
                    global::<u32>(CACHED_HI).write(hi);
                    if hi != 0 || lo > MEM_BOUND {
                        global::<u32>(MEMOUT).write(rd32(bb.wrapping_add(12)));
                    } else {
                        global::<u32>(MEMOUT).write(0);
                    }
                } else {
                    global::<u32>(MEMOUT).write(rd32(bb.wrapping_add(12)));
                }
            } else if hi != 0 || lo > MEM_BOUND {
                global::<u32>(MEMOUT).write(rd32(bb.wrapping_add(12)));
            } else {
                global::<u32>(MEMOUT).write(0);
            }
        }
        return_f32_bits(global::<u32>(AFLT).read())
    }
});
