// original: 0x00ae1a90 slot_range_resolve (proposed)

/// Resolve the active index range of the current slot and store it.
///
/// Takes no arguments (cdecl, nothing read from the incoming stack). The
/// slot is looked up through the registry global: a null registry, a
/// negative selector, or a selector at or above the table's 16-bit count
/// all mean "no slot" and the call ends with no writes. Otherwise the slot
/// is the selector-th slot pointer of the table.
///
/// With a slot, the helper object (thiscall on a fixed object, no
/// arguments) and the live-bits global decide the guard: kind is the slot's
/// first byte minus one. Kind 8 needs a non-negative word at `+0x10` and
/// kind 9 needs nothing further, but either returns early -- copying the
/// base index at `+0x14` to both outputs at `+0x18`/`+0x1c` -- when the
/// live bits or the helper answer is null. Any other kind stores 0 and
/// `0xffffffff` to the outputs and returns.
///
/// Kind 8 scans forward then backward. Each scan starts from the resolver
/// callee (base global added to the slot's `+0x14`) refined through the
/// locator callee, then walks one step at a time while the probe callee
/// (slot index, two scratch buffers, live word) and the test callee
/// (thiscall on the helper answer, same two buffers) both answer true;
/// the forward walk stops at the limit global, the backward walk at zero,
/// each with a one-step overshoot correction. The locator's `+0xc` word
/// minus the base global lands in `+0x1c` (forward) and `+0x18`
/// (backward). A final measure call (whose third argument reuses the
/// locator's still-pushed argument) widens `+0x18` up and the limiter
/// call narrows `+0x1c` down, both comparisons unsigned.
///
/// Kind 9 has the same skeleton with the probe called as
/// (index, live word, buffer, -1, 0) and the second test callee taking
/// (slot, buffer): the forward walk passes the helper answer, while the
/// backward walk reloads the helper answer from its spill slot (the load
/// reads the same saved word the forward walk's setup uses) and passes it
/// along; it reloads its candidate from the current index each round and
/// stops at zero.
///
/// Original: 0x00ae1a90 (cdecl, no stack arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00ae1a90() -> u32 {
    unsafe {
        const SLOT_KIND: u32 = 0x00;
        const SLOT_INDEX: u32 = 0x10;
        const SLOT_BASE: u32 = 0x14;
        const SLOT_LO: u32 = 0x18;
        const SLOT_HI: u32 = 0x1c;
        const REG_SELECT: u32 = 0xf8;
        const REG_TABLE: u32 = 0x9c;
        const TAB_COUNT: u32 = 0x04;
        const LIVE_WORD: u32 = 0x64;
        const G_REG: u32 = 0x01593b6c;
        const G_LIVE: u32 = 0x011f70fc;
        const G_BASE: u32 = 0x011f7028;
        const G_LIMIT: u32 = 0x011f707c;
        const HELPER_OBJ: u32 = 0x0103e498;
        const CALLEE_HELPER: u32 = 1;
        const CALLEE_RESOLVE: u32 = 2;
        const CALLEE_LOCATE: u32 = 3;
        const CALLEE_PROBE: u32 = 4;
        const CALLEE_PROBE9: u32 = 9;
        const CALLEE_TEST: u32 = 5;
        const CALLEE_MEASURE: u32 = 6;
        const CALLEE_LIMIT: u32 = 7;
        const CALLEE_TEST2: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let reg = rd32(lf_checker_rt::relocated(G_REG));
        let mut slot: u32 = 0;
        if reg != 0 {
            let sel = rd32(reg.wrapping_add(REG_SELECT));
            if (sel as i32) >= 0 {
                let tab = rd32(reg.wrapping_add(REG_TABLE));
                if sel < rd16(tab.wrapping_add(TAB_COUNT)) as u32 {
                    slot = rd32(rd32(tab).wrapping_add(sel.wrapping_mul(4)));
                }
            }
        }
        if slot == 0 {
            return 0;
        }
        let live0 = rd32(lf_checker_rt::relocated(G_LIVE));
        let helper: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_HELPER,
            u32,
            lf_checker_rt::relocated(HELPER_OBJ),
        );
        let kind = rd8(slot.wrapping_add(SLOT_KIND)).wrapping_sub(1);
        let base = rd32(lf_checker_rt::relocated(G_BASE));
        let limit = rd32(lf_checker_rt::relocated(G_LIMIT));
        if kind == 8 {
            if rd32(slot.wrapping_add(SLOT_INDEX)) == 0xffffffff {
                wr32(slot.wrapping_add(SLOT_LO), 0);
                wr32(slot.wrapping_add(SLOT_HI), 0xffffffff);
                return 0;
            }
            if live0 == 0 || helper == 0 {
                let b = rd32(slot.wrapping_add(SLOT_BASE));
                wr32(slot.wrapping_add(SLOT_LO), b);
                wr32(slot.wrapping_add(SLOT_HI), b);
                return 0;
            }
            let live_word = rd32(live0.wrapping_add(LIVE_WORD));
            let mut buf_a: u32 = 0;
            let mut buf_b: u32 = 0;
            let pa = core::ptr::addr_of_mut!(buf_a) as u32;
            let pb = core::ptr::addr_of_mut!(buf_b) as u32;
            // Forward scan.
            let r = lf_checker_rt::callee_cdecl!(
                CALLEE_RESOLVE,
                u32,
                rd32(slot.wrapping_add(SLOT_BASE)).wrapping_add(base)
            );
            let l = lf_checker_rt::callee_cdecl!(
                CALLEE_LOCATE,
                u32,
                rd32(r.wrapping_add(0x1c)).wrapping_add(1)
            );
            let mut idx = rd32(l.wrapping_add(0x1c));
            if idx < limit {
                loop {
                    let ok: u32 = lf_checker_rt::callee_cdecl!(
                        CALLEE_PROBE,
                        u32,
                        idx,
                        live_word,
                        pb,
                        rd32(slot.wrapping_add(SLOT_INDEX)),
                        pa
                    );
                    if (ok & 0xff) == 0 {
                        break;
                    }
                    let ok2: u32 =
                        lf_checker_rt::callee_thiscall!(CALLEE_TEST, u32, helper, pb, pa);
                    if (ok2 & 0xff) == 0 {
                        idx = idx.wrapping_sub(1);
                        break;
                    }
                    idx = idx.wrapping_add(1);
                    if idx >= limit {
                        break;
                    }
                }
            }
            if idx == limit {
                idx = idx.wrapping_sub(1);
            }
            let l2 = lf_checker_rt::callee_cdecl!(CALLEE_LOCATE, u32, idx);
            wr32(
                slot.wrapping_add(SLOT_HI),
                rd32(l2.wrapping_add(0x0c)).wrapping_sub(base),
            );
            // Backward scan.
            let r = lf_checker_rt::callee_cdecl!(
                CALLEE_RESOLVE,
                u32,
                rd32(slot.wrapping_add(SLOT_BASE)).wrapping_add(base)
            );
            let mut idx = rd32(r.wrapping_add(0x1c));
            if (idx as i32) >= 0 {
                loop {
                    let ok: u32 = lf_checker_rt::callee_cdecl!(
                        CALLEE_PROBE,
                        u32,
                        idx,
                        live_word,
                        pb,
                        rd32(slot.wrapping_add(SLOT_INDEX)),
                        pa
                    );
                    if (ok & 0xff) == 0 {
                        break;
                    }
                    let ok2: u32 =
                        lf_checker_rt::callee_thiscall!(CALLEE_TEST, u32, helper, pb, pa);
                    if (ok2 & 0xff) == 0 {
                        idx = idx.wrapping_add(1);
                        break;
                    }
                    idx = idx.wrapping_sub(1);
                    if (idx as i32) < 0 {
                        break;
                    }
                }
                if (idx as i32) < 0 {
                    idx = 0;
                }
            } else {
                idx = 0;
            }
            let l3 = lf_checker_rt::callee_cdecl!(CALLEE_LOCATE, u32, idx);
            wr32(
                slot.wrapping_add(SLOT_LO),
                rd32(l3.wrapping_add(0x0c)).wrapping_sub(base),
            );
            let m = lf_checker_rt::callee_cdecl!(
                CALLEE_MEASURE,
                u32,
                1u32,
                rd32(slot.wrapping_add(SLOT_INDEX)),
                idx
            );
            let mut floor: u32 = 0;
            if m != 0 {
                floor = rd32(m.wrapping_add(0x68));
            }
            let cap: u32 = lf_checker_rt::callee_cdecl!(
                CALLEE_LIMIT,
                u32,
                rd32(slot.wrapping_add(SLOT_INDEX))
            );
            if rd32(slot.wrapping_add(SLOT_LO)) < floor {
                wr32(slot.wrapping_add(SLOT_LO), floor);
            }
            if rd32(slot.wrapping_add(SLOT_HI)) > cap {
                wr32(slot.wrapping_add(SLOT_HI), cap);
            }
            return 0;
        }
        if kind == 9 {
            if live0 == 0 || helper == 0 {
                let b = rd32(slot.wrapping_add(SLOT_BASE));
                wr32(slot.wrapping_add(SLOT_LO), b);
                wr32(slot.wrapping_add(SLOT_HI), b);
                return 0;
            }
            let live_word = rd32(live0.wrapping_add(LIVE_WORD));
            let mut buf_b: u32 = 0;
            let pb = core::ptr::addr_of_mut!(buf_b) as u32;
            // Forward scan.
            let r = lf_checker_rt::callee_cdecl!(
                CALLEE_RESOLVE,
                u32,
                rd32(slot.wrapping_add(SLOT_BASE)).wrapping_add(base)
            );
            let mut idx = rd32(r.wrapping_add(0x1c));
            if idx < limit {
                loop {
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        CALLEE_PROBE9,
                        u32,
                        idx,
                        live_word,
                        pb,
                        0xffffffffu32,
                        0u32
                    );
                    let ok: u32 =
                        lf_checker_rt::callee_thiscall!(CALLEE_TEST2, u32, helper, slot, pb);
                    if (ok & 0xff) == 0 {
                        if idx != 0 {
                            idx = idx.wrapping_sub(1);
                        }
                        break;
                    }
                    idx = idx.wrapping_add(1);
                    if idx >= limit {
                        break;
                    }
                }
            }
            if idx == limit {
                idx = idx.wrapping_sub(1);
            }
            let l2 = lf_checker_rt::callee_cdecl!(CALLEE_LOCATE, u32, idx);
            wr32(
                slot.wrapping_add(SLOT_HI),
                rd32(l2.wrapping_add(0x0c)).wrapping_sub(base),
            );
            // Backward scan.
            let r = lf_checker_rt::callee_cdecl!(
                CALLEE_RESOLVE,
                u32,
                rd32(slot.wrapping_add(SLOT_BASE)).wrapping_add(base)
            );
            let mut idx = rd32(r.wrapping_add(0x1c));
            if idx != 0 {
                loop {
                    let cand = idx.wrapping_sub(1);
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        CALLEE_PROBE9,
                        u32,
                        cand,
                        live_word,
                        pb,
                        0xffffffffu32,
                        0u32
                    );
                    // The original reloads the spilled helper answer here.
                    let ok: u32 =
                        lf_checker_rt::callee_thiscall!(CALLEE_TEST2, u32, helper, slot, pb);
                    if (ok & 0xff) == 0 {
                        idx = idx.wrapping_add(1);
                        break;
                    }
                    idx = cand;
                    if idx == 0 {
                        break;
                    }
                }
            }
            let l3 = lf_checker_rt::callee_cdecl!(CALLEE_LOCATE, u32, idx);
            wr32(
                slot.wrapping_add(SLOT_LO),
                rd32(l3.wrapping_add(0x0c)).wrapping_sub(base),
            );
            return 0;
        }
        wr32(slot.wrapping_add(SLOT_LO), 0);
        wr32(slot.wrapping_add(SLOT_HI), 0xffffffff);
        0
    }
});
