// original: 0x00CD5480 ped_task_pick_or_scan (proposed)
/// Pick a ready subtask directly, or scan the list for one that fits.
///
/// `this` is the task object (gate word at `+0x14`, parameters at `+0x1c`,
/// byte at `+0x20`, auxiliary object at `+0x24`, mode flags at `+0x5c`) and
/// `a0`/`a1` are two caller objects the answer is stored into. Thiscall
/// with six stack words; returns 1 in `al` with the answer stored, else 0.
/// The low byte of `a2` selects the mode: a zero gate, a nonzero mode byte,
/// or mode bit 1 takes the direct path, otherwise the scan path.
///
/// Direct path: the first probe (callee 0) runs on (`this`, an address or
/// parameter, a patched `a2` word, a patched `a3` word). The `a2` word
/// carries the task byte in its low byte over the input's upper bytes,
/// or is a whole zero word when the mode byte is set and `a3` differs
/// from the parameter; the `a3` word carries a byte read from `a0` at
/// `+0x219` over the input's upper bytes. The second probe (callee 1) runs
/// on the first answer's low byte, then the lookup (callee 2) on the pool
/// object with (1, address of the `a1` slot, 1, 1, `a0`, 0, 0, second
/// answer's low byte). The sixth and seventh words are read from beyond
/// the last argument (caller garbage, pinned to 0 by the contract's stack
/// fill); the `a4`/`a5` inputs are dead (their slots are float scratch).
/// A nonzero lookup answer runs the
/// resolver (callee 3) and stores its answer into `a1`. A zero answer falls
/// into the scan path.
/// Scan path: the gate, the auxiliary object, its kind word (must be 1)
/// and its ready word (must be nonzero) are checked, then the counter
/// callee (callee 4) gives the list length. For each index the membership
/// test (callee 5) runs; on success four float callees (6-9, x87 float
/// answers read from `eax`) supply candidates and `r = f1-(f4+f3)` or
/// `r = f3-f2` is formed in that operand order depending on whether `f1`
/// is above `f3` (unordered takes the second). Only a strictly negative
/// `r` continues: the check callee (callee 11) runs on `a0`, both probes
/// re-run, the accepts test (callee 12) runs on the pool with the object
/// from callee 10 and the candidate bits, the fetch (callee 13) supplies
/// the entry, and the wrap callee (callee 14) links it into scratch whose
/// two words the contract seeds. After the loop the finish callee
/// (callee 15) runs on the pool with the scratch address; a nonzero answer
/// resolves through callee 3 into `a1` and returns 1, otherwise returns 0.
/// Both ends run the cleanup callee (callee 16) on the scratch's first
/// word when its second word's high half is nonzero.
/// The two data globals the direct path reads are dead (written to
/// scratch that is overwritten before any read) and left pristine. Float
/// order is pinned; the two above-tests are false for NaN, matching the
/// branch-not-taken sides exactly.
lf_checker_rt::export!(thiscall, rw_00CD5480(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        let _ = a4;
        let _ = a5;
        const ST_OFF: u32 = 0x14;
        const P0_OFF: u32 = 0x1c;
        const P1_OFF: u32 = 0x20;
        const AUX_OFF: u32 = 0x24;
        const FLAG_OFF: u32 = 0x5c;
        const POOL: u32 = 0x0171C968;
        const PED_BIT_OFF: u32 = 0x219;
        const AUX_OK_OFF: u32 = 0x40;
        const AUX_KIND_OFF: u32 = 0x44;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let pool = lf_checker_rt::relocated(POOL);
        let cl = (a2 & 0xFF) as u8;
        let st = rd32(this + ST_OFF);
        let direct = st == 0 || cl != 0 || rd8(this + FLAG_OFF) & 2 != 0;
        if direct {
            let s1c = rd32(this + P0_OFF);
            let b20 = rd8(this + P1_OFF) as u32;
            let (ecx0, a2w) = if cl != 0 {
                let w = if a3 != s1c { 0 } else { (a2 & 0xFFFFFF00) | b20 };
                (a3, w)
            } else {
                (s1c, (a2 & 0xFFFFFF00) | b20)
            };
            let a3w = (a3 & 0xFFFFFF00) | rd8(a0 + PED_BIT_OFF) as u32;
            let r1: u32 = lf_checker_rt::callee_thiscall!(0, u32, this, ecx0, a2w, a3w);
            let r2: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, r1 & 0xFF);
            let mut a1copy = a1;
            let a1addr = (&mut a1copy as *mut u32) as u32;
            let found: u32 = lf_checker_rt::callee_thiscall!(
                2, u32, pool, 1, a1addr, 1, 1, a0, 0, 0, r2 & 0xFF
            );
            if found != 0 {
                let ans: u32 = lf_checker_rt::callee_thiscall!(3, u32, found);
                (a1 as *mut u32).write_unaligned(ans);
                return 1;
            }
        }
        // Scan path (also the direct path's fallthrough).
        if st == 0 {
            return 0;
        }
        let aux = rd32(this + AUX_OFF);
        if aux == 0 {
            return 0;
        }
        if ((aux + AUX_KIND_OFF) as *const u16).read_unaligned() != 1 {
            return 0;
        }
        if rd32(aux + AUX_OK_OFF) == 0 {
            return 0;
        }
        let mut s = [0u32; 2];
        let saddr = s.as_mut_ptr() as u32;
        let n: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
        if (n as i32) > 0 {
            let mut idx = 0u32;
            while (idx as i32) < (n as i32) {
                let t14 = rd32(this + ST_OFF);
                let g1: u32 = lf_checker_rt::callee_thiscall!(5, u32, t14, idx);
                if g1 & 0xFF != 0 {
                    let f1 = f32::from_bits(lf_checker_rt::callee_thiscall!(6, u32, t14, idx));
                    let f2 = f32::from_bits(lf_checker_rt::callee_thiscall!(7, u32, t14, idx));
                    let ax = rd32(this + AUX_OFF);
                    let f3 = f32::from_bits(lf_checker_rt::callee_thiscall!(8, u32, ax));
                    let f4 = f32::from_bits(lf_checker_rt::callee_thiscall!(9, u32, ax));
                    let r = if f1 > f3 { sub(f1, add(f4, f3)) } else { sub(f3, f2) };
                    if 0.0f32 > r {
                        let bobj: u32 = lf_checker_rt::callee_thiscall!(10, u32, t14, idx);
                        let chk: u32 = lf_checker_rt::callee_thiscall!(11, u32, a0);
                        let s1c = rd32(this + P0_OFF);
                        let c1: u32 = lf_checker_rt::callee_thiscall!(
                            0, u32, this, s1c, rd8(this + P1_OFF) as u32, chk & 0xFF
                        );
                        let c2: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, c1 & 0xFF);
                        let xword = 0u32;
                        let xaddr = (&xword as *const u32) as u32;
                        let ok12: u32 = lf_checker_rt::callee_thiscall!(
                            12, u32, pool, bobj, 0, xaddr, 0, 0, f2.to_bits(), 0, 0, c2 & 0xFF
                        );
                        if ok12 & 0xFF != 0 {
                            let b2: u32 = lf_checker_rt::callee_thiscall!(13, u32, pool, bobj);
                            if b2 != 0 {
                                let w: u32 = lf_checker_rt::callee_thiscall!(14, u32, saddr, 0x10);
                                (w as *mut u32).write_unaligned(b2);
                            }
                        }
                    }
                }
                idx += 1;
            }
        }
        let ok15: u32 = lf_checker_rt::callee_thiscall!(15, u32, pool, saddr);
        if ok15 != 0 {
            let ans: u32 = lf_checker_rt::callee_thiscall!(3, u32, ok15);
            (a1 as *mut u32).write_unaligned(ans);
            if s[1] >> 16 != 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(16, u32, s[0]);
            }
            1
        } else {
            if s[1] >> 16 != 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(16, u32, s[0]);
            }
            0
        }
    }
});
