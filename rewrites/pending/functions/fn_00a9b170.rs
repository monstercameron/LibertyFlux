// original: 0x00A9B170 files-memory bit-scan dispatcher (unnamed in symbols)
/// Scan one entry's bit range and dispatch each set/clear bit.
///
/// `this` is the owning table, `arg` the entry: after validating the
/// entry's header chain and its threshold float (a helper converts
/// non-default thresholds first, and an over-limit reading aborts the
/// scan), the scan walks bit indices from the entry's start index (or
/// from zero up to a vtable-resolved count when the start is -1).
/// Set bits look up a live node in the table's list and run the
/// sub-operation on a match; clear bits resolve a worker through a
/// lookup helper and either commit it or release it and clear the bit.
/// Returns nothing meaningful (the original leaves call leftovers in EAX).
lf_checker_rt::export!(thiscall, rb110_fn1(this: u32, arg: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_CONVERT: u32 = 1; // threshold helper (cdecl/1, float result)
    // Note: contract id 2 is the planted vtable slot 0xA0, called through
    // the fabricated object (vcall0), not through the stub table.
    const CAL_SUBOP: u32 = 3; // entry sub-operation (thiscall/1)
    const CAL_LOOKUP: u32 = 4; // worker lookup (thiscall/2)
    const CAL_RELEASE: u32 = 5; // worker release (thiscall/0)
    const CAL_DETACH: u32 = 6; // worker detach (thiscall/1)
    const CAL_COMMIT: u32 = 7; // worker commit (thiscall/2)

    // Globals (file VAs; resolved through the worker's image base).
    const K_DEFAULT_VA: u32 = 0x00FE8D18; // default threshold (FLT_MAX)
    const K_LIMIT_VA: u32 = 0x00E9CAE0; // abort limit (900.0)

    const VT_COUNT_SLOT: u32 = 0xA0; // vtable slot answering the count source
    const LIST_HEAD_OFF: u32 = 0x8EC50; // live-node list head on the table

    /// Call a planted vtable slot exactly like the original: load the
    /// table pointer from the object, load the slot, call it. Both sides
    /// land on the same recorder stub.
    #[inline(always)]
    unsafe fn vcall0(object: u32, slot: u32) -> u32 {
        let vtable = *(object as *const u32);
        let target = *((vtable.wrapping_add(slot)) as *const u32);
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        f(object)
    }

    unsafe {
        // Header chain: entry -> body -> inner, inner word must be nonzero.
        let body = *(arg as *const u32);
        if body == 0 {
            return 0;
        }
        let inner = *((body.wrapping_add(0x34)) as *const u32);
        if inner == 0 {
            return 0;
        }
        if *(inner as *const u32) == 0 {
            return 0;
        }
        // Threshold gate: the default skips conversion, anything else is
        // converted, and an over-limit reading aborts the scan. Both
        // comparisons are exact ordered float comparisons (NaN converts
        // and then continues, matching ucomiss/comiss).
        let def = *(lf_checker_rt::global::<f32>(K_DEFAULT_VA));
        let x = *((arg.wrapping_add(0x10)) as *const f32);
        if x != def {
            let y = lf_checker_rt::callee_cdecl!(CAL_CONVERT, f32, arg.wrapping_add(0x10));
            let lim = *(lf_checker_rt::global::<f32>(K_LIMIT_VA));
            if y >= lim {
                return 0;
            }
        }
        // Index range: explicit start runs one step, -1 resolves the
        // count through the vtable (or a fallback object) and runs
        // from zero.
        let start = *((arg.wrapping_add(0x44)) as *const i32);
        let bound: i32;
        let mut idx: i32;
        if start == -1 {
            let r = vcall0(body, VT_COUNT_SLOT);
            let c1 = if r != 0 {
                r
            } else {
                let fb = *((body.wrapping_add(0x38)) as *const u32);
                if fb == 0 {
                    return 0;
                }
                fb
            };
            let c2 = *((c1.wrapping_add(4)) as *const u32);
            let c3 = *((c2.wrapping_add(0x0C)) as *const u32);
            bound = *((c3.wrapping_add(0x92)) as *const u16) as i32;
            idx = 0;
        } else {
            bound = start.wrapping_add(1);
            idx = start;
        }
        // Bit loop with a rotating one-bit mask.
        let mut mask: u32 = 1u32.rotate_left(idx as u32);
        while idx < bound {
            let wordp =
                (body.wrapping_add(((idx >> 5) as u32).wrapping_mul(4)).wrapping_add(0x1AC))
                    as *mut u32;
            if (*wordp & mask) == 0 {
                // Clear bit: resolve a worker and run the sub-operation.
                let w = lf_checker_rt::callee_thiscall!(CAL_LOOKUP, u32, this, body, idx as u32);
                if w != 0 {
                    let g = lf_checker_rt::callee_thiscall!(CAL_SUBOP, u32, w, arg);
                    if g != 0 {
                        lf_checker_rt::callee_thiscall!(CAL_COMMIT, u32, this, body, idx as u32);
                    } else {
                        lf_checker_rt::callee_thiscall!(CAL_RELEASE, u32, w);
                        lf_checker_rt::callee_thiscall!(CAL_DETACH, u32, this, body);
                        *wordp &= !mask;
                    }
                }
            } else {
                // Set bit: find the live node and run the sub-operation.
                let mut node = *((this.wrapping_add(LIST_HEAD_OFF)) as *const u32);
                loop {
                    if node == 0 {
                        break;
                    }
                    let same_body = *((node.wrapping_add(0x68)) as *const u32) == body;
                    let same_idx = *((node.wrapping_add(8)) as *const u32) == idx as u32;
                    if same_body && same_idx {
                        let g = lf_checker_rt::callee_thiscall!(CAL_SUBOP, u32, node, arg);
                        if g != 0 {
                            lf_checker_rt::callee_thiscall!(
                                CAL_COMMIT, u32, this, body, idx as u32
                            );
                        }
                        break;
                    }
                    node = *(node as *const u32);
                }
            }
            mask = mask.rotate_left(1);
            idx = idx.wrapping_add(1);
        }
        0
    }
});
