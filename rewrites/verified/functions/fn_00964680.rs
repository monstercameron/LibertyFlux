// original: 0x00964680 replay_thumbnail_publish (proposed)
//
// Checked-in text of this file is in out/rewrites/fn_00964680.rs; the mutant
// below it is in-crate only, never shipped.

/// Publish a replay thumbnail: gate on the slot triple, size a share from
/// two virtual queries, and either build a filename and submit it or take a
/// fallback submit path.
///
/// Gates: byte `GATE_A` must be 0, `SLOT`/`SLOT_VAL` must be non-null, and
/// `SLOT_ID` must read 1 (it is decremented to 0). Anything else exits
/// quietly; the original returns the security-cookie (an instruction of the original)
/// those paths, which no rewrite can reproduce, so the contract pins the
/// gates open (see narrowed).
///
/// The main sequence notifies a singleton, then asks the `SLOT_VAL` object
/// two virtual questions (slots +0x24 and +0x20): with the first answer `a`
/// and the word at `SLOT+2`, the share is `((a - w) - sign(a - w)) >> 1`
/// (arithmetic), and the second answer times the share is handed to a
/// three-argument registrar as (slot, 0, product).
///
/// A 24-byte descriptor is then built from fixed table bytes. When byte
/// `GATE_B` is set, or the global probe declines, the fallback path runs a
/// pair helper and a keyed writer on the descriptor head. Otherwise the full
/// path runs them on the descriptor tail, looks a name up (a null name ends
/// the work), binds it, runs a bounded formatter, assembles a filename in a
/// local buffer (a global string, a second global string copied over its
/// NUL, then a fixed dword and byte past the new end) and submits it through
/// a data-table
/// slot. The full path and the gate-driven fallback converge on a
/// 14-constant submit call and a release call (the probe-driven fallback and
/// the null-name path skip them), then the slot notifier; the return value is
/// the notifier's answer. Every exit runs the security-cookie check.
///
/// Original: 0x00964680 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00964680() -> u32 {
    unsafe {
        const GATE_A: u32 = 0x01037759;
        const GATE_B: u32 = 0x0103775A;
        const SLOT: u32 = 0x012088B8;
        const SLOT_VAL: u32 = 0x012088BC;
        const SLOT_ID: u32 = 0x012088D4;
        const SLOT_KEY: u32 = 0x012088D8;
        const NAME_SRC: u32 = 0x011F6954;
        const NAME_ARG: u32 = 0x011F6F34;
        const OBJ_NOTIFY: u32 = 0x011737D0;
        const OBJ_SUBMIT: u32 = 0x0116BFF0;
        const OBJ_RELEASE: u32 = 0x01033130;
        const STR1: u32 = 0x01168DD8;
        const TBL_QW: u32 = 0x00E8A8CC;
        const TBL_DW: u32 = 0x00E8A8D4;
        const TBL_B0: u32 = 0x00E8A8D8;
        const TBL_DW2: u32 = 0x00E8A8DC;
        const TBL_W: u32 = 0x00E8A8E0;
        const TBL_B1: u32 = 0x00E8A8E2;
        const SUFFIX_DW: u32 = 0x00E8A8F4;
        const SUFFIX_B: u32 = 0x00E8A8F8;
        const SUBMIT_MAIN: u32 = 0x00E8A8FC;
        const SUBMIT_ALT: u32 = 0x00E8A8E4;
        const DTABLE_SLOT: u32 = 0x00E73268;
        const FMT_BOUND: u32 = 0x1FF;
        const VSLOT_A: u32 = 0x24;
        const VSLOT_B: u32 = 0x20;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
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
        unsafe fn cookie() {
            unsafe { lf_checker_rt::callee_cdecl!(15, u32,) };
        }
        /// NUL-terminated byte length of the string at `s`.
        #[inline(always)]
        unsafe fn strlen(s: u32) -> usize {
            unsafe {
                let mut n = 0usize;
                while rd8(s.wrapping_add(n as u32)) != 0 {
                    n += 1;
                }
                n
            }
        }
        /// The 14-constant submit call plus the release call.
        #[inline(always)]
        unsafe fn submit_block(submit_const: u32) {
            unsafe {
                let r = lf_checker_rt::relocated;
                let ans: u32 = lf_checker_rt::callee_thiscall!(
                    12, u32, r(OBJ_SUBMIT), submit_const, 1, 0, 0, 0, 0, 1, 1, 0, 1,
                    1, 1, 1, 0xFFFFFFFF
                );
                let _: u32 = lf_checker_rt::callee_thiscall!(13, u32, r(OBJ_RELEASE), ans);
            }
        }

        let r = lf_checker_rt::relocated;
        // All four early exits return the cookie (an instruction of the original)
        // original; the contract pins every gate open (see narrowed).
        if rd8(r(GATE_A)) != 0 {
            cookie();
            return 0;
        }
        let slot = rd32(r(SLOT));
        if slot == 0 {
            cookie();
            return 0;
        }
        let slot_val = rd32(r(SLOT_VAL));
        if slot_val == 0 {
            cookie();
            return 0;
        }
        let left = rd32(r(SLOT_ID)).wrapping_sub(1);
        wr32(r(SLOT_ID), left);
        if left != 0 {
            cookie();
            return 0;
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, r(OBJ_NOTIFY), 0);
        let vt = rd32(slot_val);
        let qa: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vt.wrapping_add(VSLOT_A)) as usize) };
        let qb: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vt.wrapping_add(VSLOT_B)) as usize) };
        let a = qa(slot_val);
        let w = rd16(slot.wrapping_add(2));
        let d = a.wrapping_sub(w);
        // cdq then subtract: d - sign(d).
        let e = d.wrapping_sub((d as i32 >> 31) as u32);
        // Arithmetic halving of the adjusted difference.
        let share = ((e as i32) >> 1) as u32;
        let b = qb(slot_val);
        let product = b.wrapping_mul(share);
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, slot_val, slot, 0, product);

        // 24-byte descriptor from the fixed table (last byte stays zero).
        let mut full = [0u8; 24];
        full[0..4].copy_from_slice(&rd32(r(TBL_QW)).to_le_bytes());
        full[4..8].copy_from_slice(&rd32(r(TBL_QW).wrapping_add(4)).to_le_bytes());
        full[8..12].copy_from_slice(&rd32(r(TBL_DW)).to_le_bytes());
        full[12] = rd8(r(TBL_B0));
        full[16..20].copy_from_slice(&rd32(r(TBL_DW2)).to_le_bytes());
        full[20..22].copy_from_slice(&(rd16(r(TBL_W)) as u16).to_le_bytes());
        full[22] = rd8(r(TBL_B1));

        if rd8(r(GATE_B)) != 0 {
            // Fallback path, gate-driven: runs the submit block.
            let _: u32 =
                lf_checker_rt::callee_cdecl!(6, u32, full.as_ptr() as u32, 0);
            let _: u32 = lf_checker_rt::callee_cdecl!(
                8,
                u32,
                r(SLOT_KEY),
                slot,
                full.as_ptr() as u32
            );
            submit_block(r(SUBMIT_ALT));
        } else {
            let probe: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
            if probe == 0 {
                // Fallback path, probe-driven: skips the submit block.
                let _: u32 =
                    lf_checker_rt::callee_cdecl!(6, u32, full.as_ptr() as u32, 0);
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    8,
                    u32,
                    r(SLOT_KEY),
                    slot,
                    full.as_ptr() as u32
                );
            } else {
                // Full path.
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    6,
                    u32,
                    full.as_ptr().wrapping_add(16) as u32,
                    0
                );
                let named: u32 = lf_checker_rt::callee_thiscall!(
                    7,
                    u32,
                    rd32(r(NAME_SRC)),
                    rd32(r(NAME_ARG))
                );
                if named != 0 {
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        8,
                        u32,
                        r(SLOT_KEY),
                        slot,
                        full.as_ptr().wrapping_add(16) as u32
                    );
                    let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, named);
                    let mut name = [0u8; 64];
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        10,
                        u32,
                        name.as_mut_ptr().wrapping_add(5) as u32,
                        0,
                        FMT_BOUND
                    );
                    // Loop 1: copy the first global string.
                    let mut i = 0usize;
                    loop {
                        let ch = rd8(r(STR1).wrapping_add(i as u32));
                        name[i] = ch;
                        i += 1;
                        if ch == 0 {
                            break;
                        }
                    }
                    // Append the second global string past the NUL.
                    let l2 = strlen(r(SLOT_KEY));
                    let mut j = 0usize;
                    while name[j] != 0 {
                        j += 1;
                    }
                    let mut k = 0usize;
                    while k <= l2 {
                        name[j + k] = rd8(r(SLOT_KEY).wrapping_add(k as u32));
                        k += 1;
                    }
                    // Append the fixed suffix past the new end.
                    let mut m = 0usize;
                    while name[m] != 0 {
                        m += 1;
                    }
                    let suf = rd32(r(SUFFIX_DW));
                    name[m..m + 4].copy_from_slice(&suf.to_le_bytes());
                    name[m + 4] = rd8(r(SUFFIX_B));
                    let tgt = rd32(r(DTABLE_SLOT));
                    let submit: extern "stdcall" fn(u32) -> u32 =
                        unsafe { core::mem::transmute(tgt as usize) };
                    let _: u32 = submit(name.as_mut_ptr() as u32);
                    submit_block(r(SUBMIT_MAIN));
                }
            }
        }

        let out: u32 = lf_checker_rt::callee_thiscall!(14, u32, r(SLOT));
        cookie();
        out
    }
});
