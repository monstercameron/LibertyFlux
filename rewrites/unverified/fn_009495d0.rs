// original: 0x009495d0 NativeImpl_SET_OBJECT_COORDINATES
/// Filter a callee-supplied object array in place, then run a per-object
/// pass over the survivors.
///
/// Takes an object pointer. Three global gates end the call early; then an
/// engine pre-pass (callee 1) runs, a float is read through the object's
/// vtable slot 0x58, and callee 3 fills a count plus an object-pointer array
/// (at most two entries in this contract). A zero count ends the call.
/// Loop 1 drops each entry equal to the input object and dispatches the rest
/// on a type tag: tag 2 takes a flag-gated seven-word call (callee 9), tag 3
/// takes a flag-gated virtual call (slot 0x34), anything else is kept; failed
/// checks zero the slot. Loop 2 skips surviving entries that equal the input,
/// are null, or sit under a cleared gate word, optionally runs a setup pair
/// (callees 5 and 6), probes the pair through callee 7, then dispatches again
/// on the tag: tag 2 clears a pointer slot plus a bounded run of neighbour
/// slots (callees 8 and 10) and notifies a global object through its vtable
/// slot 8, tag 3 issues one call (callee 8), anything else is skipped.
/// Every exit runs the cookie check (callee 12), whose answer is scripted to
/// a fixed word: its entry value depends on the frame on the early-exit path.
///
/// The two loop bounds and the neighbour-run bound are re-read every
/// iteration exactly like the original; counts above two never occur here
/// (the stub writes at most two words), so the 16-bit index truncation in
/// the original is unobservable. Frame-pointer arguments (callees 1 and 3)
/// are skipped in the comparison with call-time snapshots; the count and the
/// array are verified through the loops that consume them.
///
/// Returns the cookie-check answer (0 in this contract).
/// Callee ids: 1 pre-pass, 2 float query (planted slot 0x58), 3 fill-array,
/// 4 filter probe (planted slot 0x34), 5/6 setup pair, 7 pair probe,
/// 8 release, 9 tag-2 filter call, 10 slot release, 11 global notify
/// (planted slot 8), 12 cookie check.
const GATE1: u32 = 0x11f7060;
const GATE2A: u32 = 0x12088b4;
const GATE2B: u32 = 0xf1c040;
const GATE3: u32 = 0x1037720;
const GATE3_EXIT: u32 = 0x12;
const GOBJ: u32 = 0x166d9fc;
const OUTER_ARG_OFF: u32 = 0x20;
const OUTER_GATE_OFF: u32 = 0x38;
const ELEM_TAG_OFF: u32 = 0x28;
const ELEM_GATE_OFF: u32 = 0x38;
const ELEM_SKIP_OFF: u32 = 0x219;
const ELEM_ZAP_OFF: u32 = 0x26c;
const ELEM_F14_OFF: u32 = 0xf14;
const ELEM_PTR_OFF: u32 = 0xf50;
const ELEM_RUN_OFF: u32 = 0xf54;
const ELEM_BOUND_OFF: u32 = 0x1070;
const FLOAT_SLOT: u32 = 0x58;
const PROBE_SLOT: u32 = 0x34;
const NOTIFY_SLOT: u32 = 8;

fn cookie() -> u32 {
    unsafe { callee_cdecl!(12, u32,) }
}

fn body<const MUT: bool>(arg0: u32) -> u32 {
    unsafe {
        if *global::<u32>(GATE1) == 1 {
            return cookie();
        }
        if *global::<u32>(GATE2A) != *global::<u32>(GATE2B) {
            return cookie();
        }
        if *global::<u32>(GATE3) == GATE3_EXIT {
            return cookie();
        }
        let esi = arg0;
        let mut slot60: [u32; 1] = [0];
        callee_thiscall!(1, u32, slot60.as_mut_ptr() as u32);
        // Float query through the object's vtable, answered in ST0; the
        // f32 return type reads it without assembly.
        let vt0 = *(esi as *const u32);
        let query: extern "thiscall" fn(u32) -> f32 = core::mem::transmute(
            *((vt0.wrapping_add(FLOAT_SLOT)) as *const u32),
        );
        let f = query(esi);
        let base20 = *(esi.wrapping_add(OUTER_ARG_OFF) as *const u32);
        let a0 = if base20 == 0 {
            esi.wrapping_add(0x10)
        } else {
            base20.wrapping_add(0x30)
        };
        let mut countw: [u32; 1] = [0];
        let mut arr: [u32; 2] = [0, 0];
        callee_cdecl!(
            3, u32, a0, f.to_bits(), 0, 0, countw.as_mut_ptr() as u32, 0x10,
            arr.as_mut_ptr() as u32, 0, 1, 1, 0, 0
        );
        let count = countw[0] as i32;
        let done = if MUT { count > 0 } else { count <= 0 };
        if done {
            return cookie();
        }
        let saved = esi;
        let n = count as usize;
        // Loop 1: filter the array in place.
        let mut i = 0usize;
        while i < n {
            let ecx = arr[i];
            if ecx != saved {
                let tag =
                    (*(ecx.wrapping_add(ELEM_TAG_OFF) as *const u32) >> 6) & 0xF;
                if tag == 2 {
                    if (*(ecx.wrapping_add(ELEM_F14_OFF) as *const u8) & 4) != 0
                    {
                        arr[i] = 0;
                    } else {
                        let al: u32 =
                            callee_stdcall!(9, u32, 1, 1, 1, 1, 0, 0, 0);
                        if (al & 0xFF) == 0 {
                            arr[i] = 0;
                        }
                    }
                } else if tag == 3 {
                    if (*(ecx.wrapping_add(ELEM_ZAP_OFF) as *const u8) & 4) != 0
                    {
                        arr[i] = 0;
                    }
                    if *(ecx.wrapping_add(ELEM_SKIP_OFF) as *const u8) != 0 {
                        arr[i] = 0;
                    } else {
                        let vt = *(ecx as *const u32);
                        let probe: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(
                                *((vt.wrapping_add(PROBE_SLOT)) as *const u32),
                            );
                        if (probe(ecx) & 0xFF) == 0 {
                            arr[i] = 0;
                        }
                    }
                }
            }
            i += 1;
        }
        // Loop 2: per-object pass over the survivors.
        let mut j = 0usize;
        while j < n {
            let edi = arr[j];
            if edi != saved
                && edi != 0
                && *(saved.wrapping_add(OUTER_GATE_OFF) as *const u32) != 0
                && *(edi.wrapping_add(ELEM_GATE_OFF) as *const u32) != 0
            {
                let b20 = *(saved.wrapping_add(OUTER_ARG_OFF) as *const u32);
                if b20 == 0 {
                    callee_thiscall!(5, u32, saved);
                    callee_thiscall!(6, u32, saved.wrapping_add(0x10), b20);
                }
                let al: u32 = callee_cdecl!(7, u32, saved, b20, edi);
                if (al & 0xFF) != 0 {
                    let tag =
                        (*(edi.wrapping_add(ELEM_TAG_OFF) as *const u32) >> 6)
                            & 0xF;
                    if tag == 2 {
                        let f50 =
                            *(edi.wrapping_add(ELEM_PTR_OFF) as *const u32);
                        if f50 != 0 {
                            callee_cdecl!(8, u32, f50, 1);
                            if *(edi.wrapping_add(ELEM_PTR_OFF) as *const u32)
                                != 0
                            {
                                callee_stdcall!(
                                    10, u32, edi.wrapping_add(ELEM_PTR_OFF)
                                );
                            }
                            *(edi.wrapping_add(ELEM_PTR_OFF) as *mut u32) = 0;
                        }
                        let mut k = 0u32;
                        let mut slotp = edi.wrapping_add(ELEM_RUN_OFF);
                        loop {
                            let bound = *(edi.wrapping_add(ELEM_BOUND_OFF)
                                as *const u8) as u32;
                            if k >= bound {
                                break;
                            }
                            let w = *(slotp as *const u32);
                            if w != 0 {
                                callee_cdecl!(8, u32, w, 1);
                                if *(slotp as *const u32) != 0 {
                                    callee_stdcall!(10, u32, slotp);
                                }
                                *(slotp as *mut u32) = 0;
                            }
                            k += 1;
                            slotp = slotp.wrapping_add(4);
                        }
                        let g = *global::<u32>(GOBJ);
                        let gvt = *(g as *const u32);
                        let notify: extern "thiscall" fn(u32, u32) -> u32 =
                            core::mem::transmute(
                                *((gvt.wrapping_add(NOTIFY_SLOT))
                                    as *const u32),
                            );
                        notify(g, edi);
                    } else if tag == 3 {
                        callee_cdecl!(8, u32, edi, 1);
                    }
                }
            }
            j += 1;
        }
        cookie()
    }
}

export!(cdecl, rw_009495D0(arg0: u32) -> u32 {
    body::<false>(arg0)
});

export!(cdecl, mut_009495D0(arg0: u32) -> u32 {
    body::<true>(arg0)
});
