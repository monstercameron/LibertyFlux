// original: 0x00CAA6E0 CEventHandler::update

/// Refresh one event handler for its owner's current scope and settle the
/// handler's current slot, running one of several short tails through a
/// shared detach-and-finish exit.
///
/// `this` is the handler: `+0x04` holds the owner, `+0x08`/`+0x0c` two task
/// slots, `+0x1c` a mode byte, `+0x20` the current slot (read once, written
/// on several paths), `+0x34` an auxiliary object passed to two callees.
/// The owner carries the scope at `+0x224` plus a slot/guard pair at
/// `+0x38`/`+0x7b4` used late. The scope carries a five-word candidate
/// window at `+0x44` and a second object at `+0x84` that every tail call
/// takes. No stack arguments; the return value is the finish call's answer.
///
/// Order of business: refresh the current slot against the owner, read the
/// first non-null of the window's first three words plus the probe answer
/// over the window, attach to the `+0x84` object, pick the working item
/// (the answer of the second call; the third call's answer is ignored),
/// and arm it. A null pick keeps or clears the current slot and exits.
/// Otherwise the resolver is asked three times: a null answer, a veto from
/// its `+0x48` slot, or a kind mismatch between its `+0x04` slot and the
/// pick's own `+0x04` slot all drop into the admission path; a match hands
/// the pick off, runs the finish call directly, and exits. Admission may
/// settle and exit at once, else surveys the handler and scans the window
/// for the first non-null candidate. With both task slots empty a gate,
/// a scope lookup and an offer decide two flag bits; otherwise the
/// candidate's flag word (`+0x0c`: bit 0 read, bit 1 set on acceptance)
/// and a chain of checks (slot/guard equality, a readiness slot on the
/// pick, two status slots on a located object around a five-argument
/// report call, a second offer) decide them. The join then dispatches on
/// the first word, the entry current value and the flag bits: a commit
/// block (a three-argument commit when the second task slot is set,
/// otherwise a gated two-argument one, then notify, handoff and settle),
/// a decremented pick reference with a double settle, or a settle with an
/// extra offer. Every exit detaches the `+0x84` object and tail-calls the
/// finish routine with it. All comparisons are equality or exact small
/// bounds; no ordering compare sees a callee answer.
///
/// Original: 0x00CAA6E0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00caa6e0(this: u32) -> u32 {
    unsafe { update_body(this, false) }
});

unsafe fn update_body(this: u32, mutate: bool) -> u32 {
    unsafe {
        const OWNER: u32 = 0x04;
        const TASK_A: u32 = 0x08;
        const TASK_B: u32 = 0x0c;
        const MODE: u32 = 0x1c;
        const CURRENT: u32 = 0x20;
        const AUX: u32 = 0x34;
        const OWN_SCOPE: u32 = 0x224;
        const OWN_SLOT: u32 = 0x38;
        const OWN_GUARD: u32 = 0x7b4;
        const SCOPE_WINDOW: u32 = 0x44;
        const SCOPE_AUX: u32 = 0x84;
        const WINDOW_WORDS: u32 = 5;
        const EV_FLAGS: u32 = 0x0c;
        const SEEN_BIT: u32 = 2;
        const MODE_BIT: u32 = 1;
        const REFCOUNT: u32 = 0x04;
        const KIND_840: u32 = 0x840;
        const KIND_DA: u32 = 0xda;
        const VT_VETO: u32 = 0x48;
        const VT_KIND: u32 = 0x04;
        const VT_CONSIDER: u32 = 0x14;
        const VT_READY: u32 = 0x18;
        const VT_STATUS: u32 = 0x0c;
        const EV_TAG_A: u32 = 0x011128a8;
        const EV_TAG_B: u32 = 0x01112778;
        const REFRESH: u32 = 1;
        const PROBE: u32 = 2;
        const ATTACH: u32 = 3;
        const PICK: u32 = 4;
        const ARM: u32 = 5;
        const CLEAR_A: u32 = 6;
        const CLEAR_B: u32 = 7;
        const CLEAR_C: u32 = 8;
        const CLEAR_D: u32 = 9;
        const DETACH: u32 = 10;
        const RESOLVE: u32 = 11;
        const HANDOFF: u32 = 12;
        const FINISH_DIRECT: u32 = 13;
        const ADMIT: u32 = 14;
        const SURVEY: u32 = 15;
        const GATE: u32 = 16;
        const FIND: u32 = 17;
        const OFFER: u32 = 18;
        const DRAIN: u32 = 19;
        const LOCATE: u32 = 20;
        const EMIT: u32 = 21;
        const SETTLE: u32 = 22;
        const COMMIT: u32 = 23;
        const COMMIT_SIMPLE: u32 = 24;
        const NOTIFY: u32 = 25;
        const FINISH: u32 = 26;
        const VETO: u32 = 27;
        const KIND_A: u32 = 28;
        const KIND_B: u32 = 29;
        const CONSIDER: u32 = 30;
        const READY: u32 = 31;
        const STATUS: u32 = 32;

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
        // Indirect (vtable) calls go through the fabricated objects exactly
        // like the original; both sides land on the same planted stubs.
        // The id constants VETO..STATUS name the planted slots for the
        // contract reader; the calls below load them from the objects.
        let _ = (VETO, KIND_A, KIND_B, CONSIDER, READY, STATUS);

        let owner = rd32(this.wrapping_add(OWNER));
        let _: u32 = lf_checker_rt::callee_thiscall!(REFRESH, u32, this.wrapping_add(CURRENT), owner);
        let scope = rd32(owner.wrapping_add(OWN_SCOPE));
        let current = rd32(this.wrapping_add(CURRENT));
        let w0 = rd32(scope.wrapping_add(SCOPE_WINDOW));
        let first = if w0 != 0 {
            w0
        } else {
            let w1 = rd32(scope.wrapping_add(SCOPE_WINDOW + 4));
            if w1 != 0 {
                w1
            } else {
                rd32(scope.wrapping_add(SCOPE_WINDOW + 8))
            }
        };
        let probed: u32 =
            lf_checker_rt::callee_thiscall!(PROBE, u32, scope.wrapping_add(SCOPE_WINDOW), 0);
        let w1 = rd32(scope.wrapping_add(SCOPE_WINDOW + 4));
        let w2 = rd32(scope.wrapping_add(SCOPE_WINDOW + 8));
        let saved = if w2 != 0 { w2 } else { w0 };
        let aux = scope.wrapping_add(SCOPE_AUX);
        let _: u32 = lf_checker_rt::callee_thiscall!(ATTACH, u32, aux, owner);
        let picked: u32 = lf_checker_rt::callee_thiscall!(PICK, u32, aux);
        // The arm call's answer is ignored: ebx is loaded before it runs.
        let _: u32 = lf_checker_rt::callee_thiscall!(ARM, u32, aux);
        if current != 0 {
            if first == 0 {
                wr32(this.wrapping_add(CURRENT), 0);
            } else if current == first {
                wr32(this.wrapping_add(CURRENT), first);
            } else {
                wr32(this.wrapping_add(CURRENT), 0);
            }
        }
        if w1 == 0 {
            if saved == 0 {
                wr32(this.wrapping_add(CURRENT), 0);
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(CLEAR_A, u32, this.wrapping_add(CURRENT));
        }
        if saved == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(CLEAR_B, u32, this.wrapping_add(CURRENT));
        }
        if probed == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(CLEAR_C, u32, this.wrapping_add(CURRENT));
        }
        macro_rules! tail {
            () => {{
                let _: u32 = lf_checker_rt::callee_thiscall!(DETACH, u32, aux, 0);
                return lf_checker_rt::callee_thiscall!(FINISH, u32, aux);
            }};
        }
        macro_rules! commit_block {
            () => {{
                if rd32(this.wrapping_add(TASK_B)) != 0 {
                    let mode = rd8(this.wrapping_add(MODE)) as u32;
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        COMMIT, u32, this.wrapping_add(CURRENT), owner, picked, mode
                    );
                } else {
                    let gate: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, this.wrapping_add(AUX));
                    if gate != 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            COMMIT_SIMPLE, u32, this.wrapping_add(CURRENT), owner, picked
                        );
                    }
                }
                let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, this, picked);
                let _: u32 = lf_checker_rt::callee_thiscall!(HANDOFF, u32, aux, picked);
                let _: u32 = lf_checker_rt::callee_thiscall!(SETTLE, u32, this);
                tail!();
            }};
        }
        if picked == 0 {
            if first == 0 {
                wr32(this.wrapping_add(CURRENT), 0);
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(CLEAR_D, u32, this.wrapping_add(CURRENT));
            }
            tail!();
        }
        let a1: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, this.wrapping_add(CURRENT));
        let mut admit = a1 == 0;
        if !admit {
            let a2: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, this.wrapping_add(CURRENT));
            let veto: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(a2).wrapping_add(VT_VETO)) as usize);
            if veto(a2) as u8 != 0 {
                admit = true;
            } else {
                let a3: u32 =
                    lf_checker_rt::callee_thiscall!(RESOLVE, u32, this.wrapping_add(CURRENT));
                let kind_a: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(a3).wrapping_add(VT_KIND)) as usize);
                let kind_b: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(picked).wrapping_add(VT_KIND)) as usize);
                let va = kind_a(a3);
                let vb = kind_b(picked);
                if (va == vb) != mutate {
                    let _: u32 = lf_checker_rt::callee_thiscall!(HANDOFF, u32, aux, picked);
                    let _: u32 = lf_checker_rt::callee_thiscall!(DETACH, u32, aux, 0);
                    let _: u32 = lf_checker_rt::callee_thiscall!(FINISH_DIRECT, u32, aux);
                    if first == 0 {
                        wr32(this.wrapping_add(CURRENT), 0);
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            CLEAR_D, u32, this.wrapping_add(CURRENT)
                        );
                    }
                    tail!();
                } else {
                    admit = true;
                }
            }
        }
        if admit {
            let mode = rd8(this.wrapping_add(MODE)) as u32;
            let ok: u32 =
                lf_checker_rt::callee_thiscall!(ADMIT, u32, this.wrapping_add(CURRENT), picked, mode);
            if ok as u8 == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(SETTLE, u32, this);
                tail!();
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(SURVEY, u32, this, picked);
            let mut scan = 0u32;
            let mut i = 0u32;
            while i < WINDOW_WORDS {
                let cand = rd32(scope.wrapping_add(SCOPE_WINDOW).wrapping_add(i * 4));
                if cand != 0 {
                    scan = cand;
                    break;
                }
                i += 1;
            }
            let (dl, al): (u32, u32);
            if rd32(this.wrapping_add(TASK_B)) != 0 || rd32(this.wrapping_add(TASK_A)) != 0 {
                dl = 1;
                if scan == 0 {
                    al = 1;
                } else if rd32(scan.wrapping_add(EV_FLAGS)) & MODE_BIT != 0 {
                    al = 1;
                } else {
                    let consider: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(rd32(rd32(scan).wrapping_add(VT_CONSIDER)) as usize);
                    if consider(scan, owner, 1, picked) as u8 != 0 {
                        wr32(
                            scan.wrapping_add(EV_FLAGS),
                            rd32(scan.wrapping_add(EV_FLAGS)) | SEEN_BIT,
                        );
                        al = 1;
                    } else {
                        let slot = rd32(owner.wrapping_add(OWN_SLOT));
                        if slot == 0 || slot != rd32(owner.wrapping_add(OWN_GUARD)) {
                            al = 0;
                        } else {
                            let ready: extern "thiscall" fn(u32) -> u32 =
                                core::mem::transmute(
                                    rd32(rd32(picked).wrapping_add(VT_READY)) as usize
                                );
                            if ready(picked) as u8 == 0 {
                                al = 0;
                            } else {
                                let n1: u32 =
                                    lf_checker_rt::callee_thiscall!(LOCATE, u32, scope);
                                let status_a: extern "thiscall" fn(u32) -> u32 =
                                    core::mem::transmute(
                                        rd32(rd32(n1).wrapping_add(VT_STATUS)) as usize
                                    );
                                if status_a(n1) == KIND_840 {
                                    al = 0;
                                } else {
                                    let n2: u32 =
                                        lf_checker_rt::callee_thiscall!(LOCATE, u32, scope);
                                    let reported: u32 = lf_checker_rt::callee_cdecl!(
                                        EMIT, u32, n2, 0,
                                        lf_checker_rt::relocated(EV_TAG_B),
                                        lf_checker_rt::relocated(EV_TAG_A), 0
                                    );
                                    if reported != 0 {
                                        al = 0;
                                    } else {
                                        let status_b: extern "thiscall" fn(u32) -> u32 =
                                            core::mem::transmute(
                                                rd32(rd32(n2).wrapping_add(VT_STATUS)) as usize
                                            );
                                        if status_b(n2) == KIND_DA {
                                            al = 0;
                                        } else if lf_checker_rt::callee_thiscall!(
                                            OFFER, u32, scan, owner, 2, picked
                                        ) as u8
                                            != 0
                                        {
                                            wr32(
                                                scan.wrapping_add(EV_FLAGS),
                                                rd32(scan.wrapping_add(EV_FLAGS)) | SEEN_BIT,
                                            );
                                            al = 1;
                                        } else {
                                            al = 0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                let gate: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, this.wrapping_add(AUX));
                if gate == 0 {
                    dl = 0;
                    al = 1;
                } else {
                    let found: u32 = lf_checker_rt::callee_thiscall!(FIND, u32, scope);
                    if found == 0 {
                        dl = 0;
                        al = 1;
                    } else if lf_checker_rt::callee_thiscall!(OFFER, u32, found, owner, 1, picked)
                        as u8
                        != 0
                    {
                        dl = 0;
                        al = 1;
                    } else {
                        let _: u32 =
                            lf_checker_rt::callee_thiscall!(DRAIN, u32, this.wrapping_add(AUX));
                        dl = 0;
                        al = 0;
                    }
                }
            }
            if first != 0 {
                if al == 0 {
                    if dl == 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(SETTLE, u32, this);
                        tail!();
                    }
                    let _: u32 = lf_checker_rt::callee_thiscall!(SETTLE, u32, this);
                    wr32(
                        picked.wrapping_add(REFCOUNT),
                        rd32(picked.wrapping_add(REFCOUNT)).wrapping_sub(1),
                    );
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        OFFER, u32, scan, owner, 0, picked
                    );
                    wr32(this.wrapping_add(CURRENT), first);
                    let _: u32 = lf_checker_rt::callee_thiscall!(SETTLE, u32, this);
                    tail!();
                }
                wr32(this.wrapping_add(CURRENT), 0);
                commit_block!();
            } else if current != 0 {
                wr32(this.wrapping_add(CURRENT), 0);
                commit_block!();
            } else if al == 0 {
                wr32(
                    picked.wrapping_add(REFCOUNT),
                    rd32(picked.wrapping_add(REFCOUNT)).wrapping_sub(1),
                );
                let _: u32 = lf_checker_rt::callee_thiscall!(SETTLE, u32, this);
                let _: u32 = lf_checker_rt::callee_thiscall!(SETTLE, u32, this);
                tail!();
            } else {
                commit_block!();
            }
        }
        // Unreachable: every path above exits through the shared tail.
        core::hint::unreachable_unchecked()
    }
}
