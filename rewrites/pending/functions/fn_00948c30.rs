// original: 0x00948c30 NativeImpl_REMOVE_CHAR_ELEGANTLY
/// Remove a character elegantly (full rewrite).
///
/// Takes an object pointer and returns nothing. Exits quietly unless the
/// object is non-null with tag byte 2 at +0xA60. Then: a query helper is
/// called with obj+0x80, a start helper with (obj, 1), and when
/// [obj+0x22C] is non-null a +0x14 virtual call on it (with obj as the
/// argument) feeds a follow-up helper called with ([obj+0x224]+0x44 and
/// the virtual answer, 4, 0). Four flag words are updated (set bit 20 at
/// +0x118; clear bit 24 at +0x270, bit 5 at +0x268, bit 0 at +0x274).
/// When bit 15 at +0x260 is set the function returns here; otherwise, if
/// bit 2 at +0x26C is set with a non-null link at +0xB30 owned by obj
/// ([link+0xF50] == obj), a keep-flag is cleared and one allocator lane
/// runs by [link+0x1304]: 4 writes a (4, +10000, -10000, 1000, 1000,
/// 1000) record, 5 writes (4, +10000, +10000, 1000) plus two 1000 words
/// on the link, and any other value with [link+0x1300] in {0, 1} writes
/// level 1 with the old level clamped to at least 8 plus a notify call;
/// a null allocator answer skips the lane. A probe helper is called with
/// obj; a non-null answer runs up to three gated teardown calls and then
/// the function returns unless the keep-flag is still set with a zero
/// byte at +0x211. Two removal branches share one shape: a 0x20-probe on
/// [obj+0x224]+0x84 whose +0x10 chain is resolved through a +0xC virtual
/// call (0xF4 returns in branch 1, 0x390 in branch 2), a second +0xC
/// virtual through [[obj+0x224]+0x50] with the same gate, and a closing
/// triplet that builds a six-word frame block, passes it to two helpers
/// and releases it. Branch 1 (link bit set with a non-null link) first
/// resolves two service handles from a global slot and two per-object
/// values through them.
export!(cdecl, rw_00948c30(obj: u32) -> u32 {
    unsafe {
        const QUERY_ID: u32 = 1;
        const START_ID: u32 = 2;
        const DISPATCH_ID: u32 = 3;
        const FOLLOW_ID: u32 = 4;
        const ALLOC_ID: u32 = 5;
        const NOTIFY_ID: u32 = 6;
        const PROBE_ID: u32 = 7;
        const TEAR1_ID: u32 = 8;
        const TEAR2_ID: u32 = 9;
        const TEAR3_ID: u32 = 10;
        const LOOKUP_ID: u32 = 11;
        const KIND_ID: u32 = 12;
        const SVC_ID: u32 = 13;
        const HANDLE_ID: u32 = 14;
        const PAIR_ID: u32 = 15;
        const VALUE_ID: u32 = 16;
        const EMIT_ID: u32 = 17;
        const BUILD_ID: u32 = 18;
        const USE_ID: u32 = 19;
        const RELEASE_ID: u32 = 20;
        const LINK_OFF: u32 = 0xB30;
        const OWNER_OFF: u32 = 0xF50;
        const MODE_OFF: u32 = 0x1304;
        const LEVEL_GATE_OFF: u32 = 0x1300;
        const SVC_GLOB: u32 = 0x167E2A0;
        const POS_BITS: u32 = 0x461C4000;
        const NEG_BITS: u32 = 0xC61C4000;
        const KILO_BITS: u32 = 0x447A0000;

        if obj == 0 {
            return 0;
        }
        if *((obj.wrapping_add(0xA60)) as *const u8) != 2 {
            return 0;
        }
        let _: u32 = callee_cdecl!(QUERY_ID, u32, obj.wrapping_add(0x80),);
        let _: u32 = callee_thiscall!(START_ID, u32, obj, 1);
        let target = *((obj.wrapping_add(0x22C)) as *const u32);
        if target != 0 {
            let vt = *(target as *const u32);
            let slot: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                *((vt.wrapping_add(0x14)) as *const u32),
            );
            let answer = slot(target, obj);
            let helper = *((obj.wrapping_add(0x224)) as *const u32);
            let _: u32 = callee_thiscall!(
                FOLLOW_ID,
                u32,
                helper.wrapping_add(0x44),
                answer,
                4,
                0
            );
        }
        *((obj.wrapping_add(0x118)) as *mut u32) |= 0x100000;
        *((obj.wrapping_add(0x270)) as *mut u32) &= 0xFEFFFFFF;
        *((obj.wrapping_add(0x268)) as *mut u32) &= 0xFFFFFFDF;
        *((obj.wrapping_add(0x274)) as *mut u32) &= 0xFFFFFFFE;
        if *((obj.wrapping_add(0x260)) as *const u32) & 0x8000 != 0 {
            return 0;
        }
        let mut keep = true;
        if *((obj.wrapping_add(0x26C)) as *const u8) & 4 != 0 {
            let link = *((obj.wrapping_add(LINK_OFF)) as *const u32);
            if link != 0 && *((link.wrapping_add(OWNER_OFF)) as *const u32) == obj {
                keep = false;
                let mode = *((link.wrapping_add(MODE_OFF)) as *const u32);
                if mode == 4 {
                    let rec: u32 = callee_cdecl!(ALLOC_ID, u32, link,);
                    if rec != 0 {
                        *((rec.wrapping_add(0x26)) as *mut u8) = 4;
                        *((rec.wrapping_add(4)) as *mut u32) = POS_BITS;
                        *((rec.wrapping_add(8)) as *mut u32) = NEG_BITS;
                        *((rec.wrapping_add(0xC)) as *mut u32) = KILO_BITS;
                        *((rec.wrapping_add(0x18)) as *mut u32) = KILO_BITS;
                        *((rec.wrapping_add(0x22)) as *mut u16) = 1000;
                    }
                } else if mode == 5 {
                    let rec: u32 = callee_cdecl!(ALLOC_ID, u32, link,);
                    if rec != 0 {
                        *((rec.wrapping_add(0x26)) as *mut u8) = 4;
                        *((rec.wrapping_add(4)) as *mut u32) = POS_BITS;
                        *((rec.wrapping_add(8)) as *mut u32) = POS_BITS;
                        *((rec.wrapping_add(0xC)) as *mut u32) = KILO_BITS;
                        *((link.wrapping_add(0x1ED0)) as *mut u32) = KILO_BITS;
                        *((link.wrapping_add(0x1ED8)) as *mut u32) = KILO_BITS;
                    }
                } else {
                    let gate = *((link.wrapping_add(LEVEL_GATE_OFF)) as *const u32);
                    if gate == 0 || gate == 1 {
                        let rec: u32 = callee_cdecl!(ALLOC_ID, u32, link,);
                        if rec != 0 {
                            let level = *((rec.wrapping_add(0x27)) as *const u8);
                            *((rec.wrapping_add(0x26)) as *mut u8) = 1;
                            let clamped = if level > 8 { level } else { 8 };
                            *((rec.wrapping_add(0x27)) as *mut u8) = clamped;
                            let arg = *((obj.wrapping_add(LINK_OFF)) as *const u32);
                            let _: u32 = callee_cdecl!(NOTIFY_ID, u32, arg,);
                        }
                    }
                }
            }
        }
        let probe: u32 = callee_thiscall!(PROBE_ID, u32, obj,);
        if probe != 0 {
            let t1: u32 = callee_thiscall!(TEAR1_ID, u32, probe.wrapping_add(8), obj);
            if (t1 as u8) != 0 {
                let t2: u32 = callee_thiscall!(TEAR2_ID, u32, probe,);
                if (t2 as u8) != 0 {
                    // Argument order is (obj, 1): the 1 is pushed first.
                    let _: u32 =
                        callee_thiscall!(TEAR3_ID, u32, probe.wrapping_add(8), obj, 1);
                }
            }
        }
        if !keep {
            return 0;
        }
        if *((obj.wrapping_add(0x211)) as *const u8) != 0 {
            return 0;
        }
        let branch1 = *((obj.wrapping_add(0x26C)) as *const u8) & 4 != 0
            && *((obj.wrapping_add(LINK_OFF)) as *const u32) != 0;
        // Both branches resolve two kind codes through +0xC virtual calls;
        // the gate value differs (0xF4 vs 0x390).
        let gate = if branch1 { 0xF4u32 } else { 0x390u32 };
        let worker = *((obj.wrapping_add(0x224)) as *const u32);
        let found: u32 = callee_thiscall!(LOOKUP_ID, u32, worker.wrapping_add(0x84), 0x20);
        if found != 0 {
            let inner = *((found.wrapping_add(0x10)) as *const u32);
            if inner != 0 {
                let vt = *(inner as *const u32);
                let slot: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(*((vt.wrapping_add(0xC)) as *const u32));
                if slot(inner) == gate {
                    return 0;
                }
            }
        }
        let outer = *((worker.wrapping_add(0x50)) as *const u32);
        if outer != 0 {
            let vt = *(outer as *const u32);
            let slot: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(*((vt.wrapping_add(0xC)) as *const u32));
            if slot(outer) == gate {
                return 0;
            }
        }
        let mut extra = 0u32;
        let mut extra2 = 0u32;
        if branch1 {
            let svc = *(global::<u32>(SVC_GLOB));
            let h1: u32 = callee_thiscall!(SVC_ID, u32, svc,);
            let handle = if h1 != 0 {
                callee_thiscall!(HANDLE_ID, u32, h1,)
            } else {
                0
            };
            let svc2 = *(global::<u32>(SVC_GLOB));
            let h2: u32 = callee_thiscall!(SVC_ID, u32, svc2,);
            let pair = if h2 != 0 {
                callee_thiscall!(PAIR_ID, u32, h2, 0, 1, 0, 1)
            } else {
                0
            };
            let target2 = *((obj.wrapping_add(0x22C)) as *const u32);
            let vt = *(target2 as *const u32);
            let slot: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(*((vt.wrapping_add(0xC)) as *const u32));
            let value = slot(target2, obj);
            let _: u32 = callee_thiscall!(EMIT_ID, u32, handle, pair);
            let _: u32 = callee_thiscall!(EMIT_ID, u32, handle, value);
            extra = handle;
            extra2 = pair;
            let _ = (extra, extra2);
            let mut block = [0u32; 6];
            let _: u32 = callee_thiscall!(
                BUILD_ID,
                u32,
                block.as_mut_ptr() as u32,
                3,
                handle,
                0
            );
            let _: u32 = callee_thiscall!(
                USE_ID,
                u32,
                worker.wrapping_add(0x84),
                block.as_ptr() as u32,
                0,
                1
            );
            let _: u32 = callee_thiscall!(RELEASE_ID, u32, block.as_mut_ptr() as u32,);
        } else {
            let target2 = *((obj.wrapping_add(0x22C)) as *const u32);
            let vt = *(target2 as *const u32);
            let slot: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(*((vt.wrapping_add(0xC)) as *const u32));
            let value = slot(target2, obj);
            let mut block = [0u32; 6];
            let _: u32 = callee_thiscall!(
                BUILD_ID,
                u32,
                block.as_mut_ptr() as u32,
                3,
                value,
                0
            );
            let _: u32 = callee_thiscall!(
                USE_ID,
                u32,
                worker.wrapping_add(0x84),
                block.as_ptr() as u32,
                0,
                1
            );
            let _: u32 = callee_thiscall!(RELEASE_ID, u32, block.as_mut_ptr() as u32,);
        }
        0
    }
});
