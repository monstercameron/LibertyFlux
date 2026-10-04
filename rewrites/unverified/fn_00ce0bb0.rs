// original: 0x00ce0bb0 nm_task_params_apply (proposed)

/// Apply a task's NaturalMotion behaviour parameters by name.
///
/// `task` is an opaque handle: it is passed to the iterator and the setters
/// and never dereferenced. Each pass asks the parameter iterator `NEXT(task,
/// 1)` for the next record; a null answer ends the scan with 1. Otherwise a
/// 3-byte key is resolved through `LOOKUP` three times, giving the name
/// string, the value string, and a check string that must read "{". A null
/// or ";"-starting name skips the record; on passes after the first, any
/// other unmatched name is reported through `WARN` and skipped.
///
/// The dispatch chain compares the name against, in order: "GrabHelper",
/// "NMBrace", "NMShot_IsMoving", "NMShot_Standing", "NMHFall", "NMBalance",
/// "NMOnFire", "NMJumpRollFromRoadVehicle", "NMFlinch", "NMExplosion".
/// Single-value behaviours ("GrabHelper", "NMShot_IsMoving",
/// "NMJumpRollFromRoadVehicle", "NMExplosion") fire their setter only when
/// the value starts with '*'. The other six take the default branch when the
/// value starts with '*' or equals their default word ("DEFAULT", "DEFAULT",
/// "STRONG", "DEFAULT", "DEFAULT", "EXPLOSION"); otherwise the value must
/// equal one of the behaviour's value words, each firing the same setter
/// with its own parameter object: brace ("WEAK"); weapon ("WEAK",
/// "POWERFUL_WEAPON", "WEAK_POWERFUL_WEAPON", "SNIPER_RIFLE",
/// "WEAK_SNIPER_RIFLE", "SHOTGUN", "WEAK_SHOTGUN", "MELEE",
/// "MELEE_BASEBALL"); fall ("WEAK"); balance ("WEAK", "MELEE",
/// "AGGRESSIVE"); fire ("WEAK"); react ("MELEE", "REACT",
/// "EXPLOSION_PASIVE", "MELEE_PASIVE", "REACT_PASIVE"). Anything else fails
/// the whole call with 0, as does a setter returning 0.
///
/// On a default branch with a non-zero setter result, the default parameter
/// block is also copied over every value block of that behaviour (21, 9x39,
/// 16, 26 and 5x14 double-words; the balance default instead runs the copy
/// helper over three objects), then the scan continues.
///
/// String comparison is the original's two-bytes-at-a-time loop, reproduced
/// exactly including read order. Original: cdecl, one stack word, result in
/// AL only (upper bytes keep whatever the last call left).

const NEXT: u32 = 0x00000000;
const LOOKUP: u32 = 0x00000001;
const WARN: u32 = 0x00000002;
const H_GRABHELPER: u32 = 0x00000003;
const H_NMBRACE: u32 = 0x00000004;
const H_NMSHOT_MOVING: u32 = 0x00000005;
const H_WEAPON: u32 = 0x00000006;
const H_NMHFALL: u32 = 0x00000007;
const H_NMBALANCE: u32 = 0x00000008;
const COPYHELP: u32 = 0x00000009;
const H_NMONFIRE: u32 = 0x0000000a;
const H_NMJUMP: u32 = 0x0000000b;
const H_REACT: u32 = 0x0000000c;
const H_NMEXPLOSION: u32 = 0x0000000d;
const LIT_KEY: u32 = 0x00edc850;
const LIT_RBRACE: u32 = 0x00edc874;
const LIT_WARN_FMT: u32 = 0x00edc878;
const LIT_LBRACE: u32 = 0x00edc898;
const L_GRABHELPER: u32 = 0x00edc8a0;
const L_NMBRACE: u32 = 0x00edc8ac;
const L_DEFAULT_B2: u32 = 0x00edc8e0;
const L_WEAK_B2: u32 = 0x00edc91c;
const L_NMSHOT_MOVING: u32 = 0x00edc92c;
const L_NMSHOT_STANDING: u32 = 0x00edc950;
const L_DEFAULT_B4: u32 = 0x00edc960;
const L_WEAK_B4: u32 = 0x00edc968;
const L_POWERFUL_WEAPON: u32 = 0x00edc978;
const L_WEAK_POWERFUL_WEAPON: u32 = 0x00edc988;
const L_SNIPER_RIFLE: u32 = 0x00edc9ac;
const L_WEAK_SNIPER_RIFLE: u32 = 0x00edc9c0;
const L_SHOTGUN: u32 = 0x00edc9d8;
const L_WEAK_SHOTGUN: u32 = 0x00edc9e0;
const L_MELEE_B4: u32 = 0x00edc9f0;
const L_MELEE_BASEBALL: u32 = 0x00edc9f8;
const L_NMHFALL: u32 = 0x00edca08;
const L_STRONG: u32 = 0x00edca10;
const L_WEAK_B5: u32 = 0x00edca18;
const L_NMBALANCE: u32 = 0x00edca20;
const L_DEFAULT_B6: u32 = 0x00edca2c;
const L_WEAK_B6: u32 = 0x00edca34;
const L_MELEE_B6: u32 = 0x00edca3c;
const L_AGGRESSIVE: u32 = 0x00edca68;
const L_NMONFIRE: u32 = 0x00edca74;
const L_DEFAULT_B7: u32 = 0x00edca80;
const L_WEAK_B7: u32 = 0x00edca88;
const L_NMJUMP: u32 = 0x00edca98;
const L_NMFLINCH: u32 = 0x00edcad8;
const L_EXPLOSION_B9: u32 = 0x00edcae4;
const L_MELEE_B9: u32 = 0x00edcb18;
const L_REACT: u32 = 0x00edcb3c;
const L_EXPLOSION_PASIVE: u32 = 0x00edcb44;
const L_MELEE_PASIVE: u32 = 0x00edcb78;
const L_REACT_PASIVE: u32 = 0x00edcb88;
const L_NMEXPLOSION: u32 = 0x00edcb98;
const OBJ_GRABHELPER: u32 = 0x0171d004;
const OBJ_NMBRACE: u32 = 0x0171d714;
const OBJ_NMBRACE_ALT: u32 = 0x0171d6c0;
const OBJ_NMSHOT_MOVING: u32 = 0x0171cf68;
const OBJ_WEAPON_ALT: u32 = 0x0171d0a8;
const OBJ_W_WEAK: u32 = 0x0171d144;
const OBJ_W_POWERFUL: u32 = 0x0171d1e0;
const OBJ_W_WEAK_POWERFUL: u32 = 0x0171d27c;
const OBJ_W_SNIPER: u32 = 0x0171d318;
const OBJ_W_WEAK_SNIPER: u32 = 0x0171d3b4;
const OBJ_W_SHOTGUN: u32 = 0x0171d450;
const OBJ_W_WEAK_SHOTGUN: u32 = 0x0171d4ec;
const OBJ_W_MELEE: u32 = 0x0171d588;
const OBJ_W_MELEE_BASEBALL: u32 = 0x0171d624;
const OBJ_NMHFALL: u32 = 0x0171d068;
const OBJ_NMHFALL_ALT: u32 = 0x0171d028;
const OBJ_B_WEAK: u32 = 0x0171cbc0;
const OBJ_B_MELEE: u32 = 0x0171cc70;
const OBJ_B_AGGRESSIVE: u32 = 0x0171cd20;
const OBJ_NMBALANCE_ALT: u32 = 0x0171cb10;
const OBJ_NMONFIRE: u32 = 0x0171caa8;
const OBJ_NMONFIRE_ALT: u32 = 0x0171ca40;
const OBJ_NMJUMP: u32 = 0x0171cdd0;
const OBJ_REACT_ALT: u32 = 0x0171ce18;
const OBJ_R_MELEE: u32 = 0x0171ce50;
const OBJ_R_REACT: u32 = 0x0171ce88;
const OBJ_R_EXPLOSION_PASIVE: u32 = 0x0171cec0;
const OBJ_R_MELEE_PASIVE: u32 = 0x0171cef8;
const OBJ_R_REACT_PASIVE: u32 = 0x0171cf30;
const OBJ_NMEXPLOSION: u32 = 0x0171cf88;
const COPYLOOP_START: u32 = 0x0171cbc0;
const COPYLOOP_ARG: u32 = 0x0171cb10;
const COPYLOOP_STEP: u32 = 0x000000b0;
const COPYLOOP_END: u32 = 0x0171cdd0;
const BRACE_COPIES: [(u32, u32, u32); 1] = [
    (21, 0x0171d6c0, 0x0171d714),
];
const WEAPON_COPIES: [(u32, u32, u32); 9] = [
    (39, 0x0171d0a8, 0x0171d144),
    (39, 0x0171d0a8, 0x0171d1e0),
    (39, 0x0171d0a8, 0x0171d27c),
    (39, 0x0171d0a8, 0x0171d318),
    (39, 0x0171d0a8, 0x0171d3b4),
    (39, 0x0171d0a8, 0x0171d450),
    (39, 0x0171d0a8, 0x0171d4ec),
    (39, 0x0171d0a8, 0x0171d588),
    (39, 0x0171d0a8, 0x0171d624),
];
const FALL_COPIES: [(u32, u32, u32); 1] = [
    (16, 0x0171d028, 0x0171d068),
];
const FIRE_COPIES: [(u32, u32, u32); 1] = [
    (26, 0x0171ca40, 0x0171caa8),
];
const REACT_COPIES: [(u32, u32, u32); 5] = [
    (14, 0x0171ce18, 0x0171ce50),
    (14, 0x0171ce18, 0x0171ce88),
    (14, 0x0171ce18, 0x0171cec0),
    (14, 0x0171ce18, 0x0171cef8),
    (14, 0x0171ce18, 0x0171cf30),
];
const SEMICOLON: u8 = 0x3b;
const STAR: u8 = 0x2a;

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}

/// Fire a thiscall setter: object pointer in ECX, task handle on the stack.
#[inline(always)]
unsafe fn fire(id: u32, obj_va: u32, task: u32) -> u32 {
    unsafe {
        lf_checker_rt::callee_thiscall!(id, u32, lf_checker_rt::relocated(obj_va), task)
    }
}

/// Copy `count` double-words from one parameter block to another, ascending
/// (`rep movsd` with a clear direction flag).
#[inline(always)]
unsafe fn copy_dwords(src_va: u32, dst_va: u32, count: u32) {
    unsafe {
        let mut s = lf_checker_rt::relocated(src_va) as *const u32;
        let mut d = lf_checker_rt::relocated(dst_va) as *mut u32;
        let mut n = count;
        while n != 0 {
            d.write(s.read());
            s = s.wrapping_add(1);
            d = d.wrapping_add(1);
            n -= 1;
        }
    }
}

/// The original's inline string compare, two bytes at a time: compare the
/// first pair, equal-and-NUL ends the match, else compare the second pair.
/// Reads happen in the original's order so fault behaviour matches too.
#[inline(always)]
unsafe fn nm_streq(var: u32, lit: u32) -> bool {
    unsafe {
        let mut i = 0u32;
        loop {
            let a0 = rd8(var.wrapping_add(i));
            let b0 = rd8(lit.wrapping_add(i));
            if a0 != b0 {
                return false;
            }
            if a0 == 0 {
                return true;
            }
            let a1 = rd8(var.wrapping_add(i).wrapping_add(1));
            let b1 = rd8(lit.wrapping_add(i).wrapping_add(1));
            if a1 != b1 {
                return false;
            }
            i = i.wrapping_add(2);
            if a1 == 0 {
                return true;
            }
        }
    }
}

/// Fetch the next task-parameter record. `None` ends the scan (return 1).
#[inline(always)]
unsafe fn tailcall(task: u32) -> Option<u32> {
    unsafe {
        let h = lf_checker_rt::callee_cdecl!(NEXT, u32, task, 1);
        if h == 0 {
            None
        } else {
            Some(h)
        }
    }
}

enum Work<'a> {
    Direct,
    Copies(&'a [(u32, u32, u32)]),
    CopyLoop,
}

unsafe fn run_00ce0bb0(task: u32, weapon_copies: &[(u32, u32, u32)]) -> u32 {
    unsafe {
        let mut ebx: u32 = 1;
        // [esp+0x13]: set to 1 once up front (via the pushed-word slot).
        let mut flag13: u8 = 1;
        let first = lf_checker_rt::callee_cdecl!(NEXT, u32, task, 1);
        if first == 0 {
            return 1;
        }
        let mut handle = first;
        loop {
            let mut buf = [0u8; 4];
            let key = lf_checker_rt::relocated(LIT_KEY);
            buf[0] = rd8(key);
            buf[1] = rd8(key.wrapping_add(1));
            buf[2] = rd8(key.wrapping_add(2));
            let key_ptr = buf.as_ptr() as u32;
            let esi = lf_checker_rt::callee_cdecl!(LOOKUP, u32, handle, key_ptr);
            let run_chain: bool;
            if esi == 0 || rd8(esi) == SEMICOLON {
                run_chain = false;
            } else if ebx == 2 {
                flag13 = 0;
                run_chain = true;
            } else {
                let al = u8::from(flag13 == 0);
                flag13 = al;
                if al == 0 {
                    run_chain = true;
                } else if nm_streq(esi, lf_checker_rt::relocated(LIT_RBRACE)) {
                    run_chain = false;
                } else {
                    lf_checker_rt::callee_cdecl!(
                        WARN,
                        u32,
                        lf_checker_rt::relocated(LIT_WARN_FMT),
                        esi
                    );
                    flag13 = 0;
                    run_chain = false;
                }
            }
            if run_chain {
                let edi = lf_checker_rt::callee_cdecl!(LOOKUP, u32, 0, key_ptr);
                if edi == 0 {
                    return 0;
                }
                let e3 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, 0, key_ptr);
                if e3 == 0 {
                    return 0;
                }
                if !nm_streq(e3, lf_checker_rt::relocated(LIT_LBRACE)) {
                    return 0;
                }
                let bl = u8::from(rd8(edi) == STAR);
                // `sete bl`: only the low byte changes.
                ebx = (ebx & 0xffff_ff00) | bl as u32;
                // [esp+0x12], rewritten every pass before any ALT tail reads it.
                let flag12 = bl;
                // Dispatch chain. Runs once: every setter call breaks out to
                // the shared ALT/tail sequence below; mismatches return 0.
                let mut work = Work::Direct;
                loop {
                    if nm_streq(esi, lf_checker_rt::relocated(L_GRABHELPER)) {
                        if bl == 0 {
                            return 0;
                        }
                        ebx = fire(H_GRABHELPER, OBJ_GRABHELPER, task);
                        break;
                    }
                    if nm_streq(esi, lf_checker_rt::relocated(L_NMBRACE)) {
                        if bl != 0 {
                            ebx = fire(H_NMBRACE, OBJ_NMBRACE_ALT, task);
                            work = Work::Copies(&BRACE_COPIES);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_DEFAULT_B2)) {
                            ebx = fire(H_NMBRACE, OBJ_NMBRACE_ALT, task);
                            work = Work::Copies(&BRACE_COPIES);
                            break;
                        }
                        if !nm_streq(edi, lf_checker_rt::relocated(L_WEAK_B2)) {
                            return 0;
                        }
                        ebx = fire(H_NMBRACE, OBJ_NMBRACE, task);
                        break;
                    }
                    if nm_streq(esi, lf_checker_rt::relocated(L_NMSHOT_MOVING)) {
                        if bl == 0 {
                            return 0;
                        }
                        ebx = fire(H_NMSHOT_MOVING, OBJ_NMSHOT_MOVING, task);
                        break;
                    }
                    if nm_streq(esi, lf_checker_rt::relocated(L_NMSHOT_STANDING)) {
                        if bl != 0 {
                            ebx = fire(H_WEAPON, OBJ_WEAPON_ALT, task);
                            work = Work::Copies(weapon_copies);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_DEFAULT_B4)) {
                            ebx = fire(H_WEAPON, OBJ_WEAPON_ALT, task);
                            work = Work::Copies(weapon_copies);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_WEAK_B4)) {
                            ebx = fire(H_WEAPON, OBJ_W_WEAK, task);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_POWERFUL_WEAPON)) {
                            ebx = fire(H_WEAPON, OBJ_W_POWERFUL, task);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_WEAK_POWERFUL_WEAPON)) {
                            ebx = fire(H_WEAPON, OBJ_W_WEAK_POWERFUL, task);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_SNIPER_RIFLE)) {
                            ebx = fire(H_WEAPON, OBJ_W_SNIPER, task);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_WEAK_SNIPER_RIFLE)) {
                            ebx = fire(H_WEAPON, OBJ_W_WEAK_SNIPER, task);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_SHOTGUN)) {
                            ebx = fire(H_WEAPON, OBJ_W_SHOTGUN, task);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_WEAK_SHOTGUN)) {
                            ebx = fire(H_WEAPON, OBJ_W_WEAK_SHOTGUN, task);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_MELEE_B4)) {
                            ebx = fire(H_WEAPON, OBJ_W_MELEE, task);
                            break;
                        }
                        if !nm_streq(edi, lf_checker_rt::relocated(L_MELEE_BASEBALL)) {
                            return 0;
                        }
                        ebx = fire(H_WEAPON, OBJ_W_MELEE_BASEBALL, task);
                        break;
                    }
                    if nm_streq(esi, lf_checker_rt::relocated(L_NMHFALL)) {
                        if bl != 0 {
                            ebx = fire(H_NMHFALL, OBJ_NMHFALL_ALT, task);
                            work = Work::Copies(&FALL_COPIES);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_STRONG)) {
                            ebx = fire(H_NMHFALL, OBJ_NMHFALL_ALT, task);
                            work = Work::Copies(&FALL_COPIES);
                            break;
                        }
                        if !nm_streq(edi, lf_checker_rt::relocated(L_WEAK_B5)) {
                            return 0;
                        }
                        ebx = fire(H_NMHFALL, OBJ_NMHFALL, task);
                        break;
                    }
                    if nm_streq(esi, lf_checker_rt::relocated(L_NMBALANCE)) {
                        if bl != 0 {
                            ebx = fire(H_NMBALANCE, OBJ_NMBALANCE_ALT, task);
                            work = Work::CopyLoop;
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_DEFAULT_B6)) {
                            ebx = fire(H_NMBALANCE, OBJ_NMBALANCE_ALT, task);
                            work = Work::CopyLoop;
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_WEAK_B6)) {
                            ebx = fire(H_NMBALANCE, OBJ_B_WEAK, task);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_MELEE_B6)) {
                            ebx = fire(H_NMBALANCE, OBJ_B_MELEE, task);
                            break;
                        }
                        if !nm_streq(edi, lf_checker_rt::relocated(L_AGGRESSIVE)) {
                            return 0;
                        }
                        ebx = fire(H_NMBALANCE, OBJ_B_AGGRESSIVE, task);
                        break;
                    }
                    if nm_streq(esi, lf_checker_rt::relocated(L_NMONFIRE)) {
                        if bl != 0 {
                            ebx = fire(H_NMONFIRE, OBJ_NMONFIRE_ALT, task);
                            work = Work::Copies(&FIRE_COPIES);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_DEFAULT_B7)) {
                            ebx = fire(H_NMONFIRE, OBJ_NMONFIRE_ALT, task);
                            work = Work::Copies(&FIRE_COPIES);
                            break;
                        }
                        if !nm_streq(edi, lf_checker_rt::relocated(L_WEAK_B7)) {
                            return 0;
                        }
                        ebx = fire(H_NMONFIRE, OBJ_NMONFIRE, task);
                        break;
                    }
                    if nm_streq(esi, lf_checker_rt::relocated(L_NMJUMP)) {
                        if bl == 0 {
                            return 0;
                        }
                        ebx = fire(H_NMJUMP, OBJ_NMJUMP, task);
                        break;
                    }
                    if nm_streq(esi, lf_checker_rt::relocated(L_NMFLINCH)) {
                        if bl != 0 {
                            ebx = fire(H_REACT, OBJ_REACT_ALT, task);
                            work = Work::Copies(&REACT_COPIES);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_EXPLOSION_B9)) {
                            ebx = fire(H_REACT, OBJ_REACT_ALT, task);
                            work = Work::Copies(&REACT_COPIES);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_MELEE_B9)) {
                            ebx = fire(H_REACT, OBJ_R_MELEE, task);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_REACT)) {
                            ebx = fire(H_REACT, OBJ_R_REACT, task);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_EXPLOSION_PASIVE)) {
                            ebx = fire(H_REACT, OBJ_R_EXPLOSION_PASIVE, task);
                            break;
                        }
                        if nm_streq(edi, lf_checker_rt::relocated(L_MELEE_PASIVE)) {
                            ebx = fire(H_REACT, OBJ_R_MELEE_PASIVE, task);
                            break;
                        }
                        if !nm_streq(edi, lf_checker_rt::relocated(L_REACT_PASIVE)) {
                            return 0;
                        }
                        ebx = fire(H_REACT, OBJ_R_REACT_PASIVE, task);
                        break;
                    }
                    if !nm_streq(esi, lf_checker_rt::relocated(L_NMEXPLOSION)) {
                        return 0;
                    }
                    if bl == 0 {
                        return 0;
                    }
                    ebx = fire(H_NMEXPLOSION, OBJ_NMEXPLOSION, task);
                    break;
                }
                match work {
                    Work::Direct => {}
                    Work::Copies(list) => {
                        if flag12 != 0 {
                            if ebx == 0 {
                                return 0;
                            }
                            for &(count, src, dst) in list {
                                copy_dwords(src, dst, count);
                            }
                        }
                    }
                    Work::CopyLoop => {
                        if flag12 != 0 {
                            if ebx == 0 {
                                return 0;
                            }
                            let mut p = COPYLOOP_START;
                            loop {
                                lf_checker_rt::callee_thiscall!(
                                    COPYHELP,
                                    u32,
                                    lf_checker_rt::relocated(p),
                                    lf_checker_rt::relocated(COPYLOOP_ARG)
                                );
                                p = p.wrapping_add(COPYLOOP_STEP);
                                if p >= COPYLOOP_END {
                                    break;
                                }
                            }
                        }
                    }
                }
                if ebx == 0 {
                    return 0;
                }
            }
            handle = match tailcall(task) {
                None => return 1,
                Some(h) => h,
            };
        }
    }
}

lf_checker_rt::export!(cdecl, rw_00ce0bb0(task: u32) -> u32 {
    unsafe { run_00ce0bb0(task, &WEAPON_COPIES) }
});
