// original: 0x00cb0060 CTaskComplexGetOutOfWater::vf20

/// Decide the next step of the get-out-of-water task for one pedestrian.
///
/// `this` is the task object, `ped` an opaque handle passed on to callees
/// (never dereferenced here). Returns a task pointer: usually the current
/// sub-task at `+SUBTASK`, zero after a fresh dispatch, or whatever the
/// scheduler-swap callee answers.
///
/// Behaviour. Let `kind(x)` be the virtual type query at vtable slot `+0xc`:
/// - If `kind(subtask) == KIND_WATER` and the auxiliary slot `+AUX` is set:
///   gate on the predicate callee (id 1); then call the fill callee (id 2)
///   with `(aux, this+LOCAL_POS, frame_slot, 5)` where `frame_slot` points at
///   a word holding `this` (the original passes the address of its own saved
///   register slot; only the pointed-to word is compared), clear `+AUX`, and
///   dispatch through the scheduler callee (id 3):
///   - fill answered nonzero: dispatch and return 0;
///   - else set flag `+FLAG`, dispatch again, and if that answers true return
///     the swap callee's answer (id 4, code `KIND_SWAP_A`).
/// - Then, when `+STATE == ARMED` and `kind(subtask) == KIND_ROUTE`,
///   the sub-sub-task at `subtask+INNER` exists with kind `KIND_SWAP_A`,
///   the pick callee (id 5) returns a live object of kind `KIND_SWAP_A`
///   whose probe callee (id 6) answers 2 and whose signed level byte
///   `+LEVEL` is at least 2, and the scheduler answers true: set
///   `+STATE = DONE` and return the swap callee's answer (code `KIND_WATER`).
/// - Every other path returns the current sub-task unchanged.
///
/// Original: 0x00cb0060 (thiscall, one stack word). No floating point, no
/// globals. The level-byte test is signed (`jl`), so values above 0x7f fail.
fn kind_of(obj: u32) -> u32 {
    unsafe {
        let vtable = (obj as *const u32).read_unaligned();
        let target = ((vtable as *const u8).add(0x0c) as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        query(obj)
    }
}

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}

fn decide<const CLEAR_AUX: bool>(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 0x08;
        const LOCAL_POS: u32 = 0x20;
        const AUX: u32 = 0x30;
        const FLAG: u32 = 0x40;
        const STATE: u32 = 0x44;
        const INNER: u32 = 0x14;
        const LEVEL: u32 = 0x9a;
        const KIND_WATER: u32 = 0xcb;
        const KIND_ROUTE: u32 = 0x11d;
        const KIND_SWAP_A: u32 = 0x3ae;
        const ARMED: u32 = 1;
        const DONE: u32 = 2;
        const FILL_TAG: u32 = 5;
        const DISPATCH_MODE: u32 = 2;
        let task = rd32(this + SUBTASK);
        if kind_of(task) == KIND_WATER {
            let aux = rd32(this + AUX);
            if aux != 0 && lf_checker_rt::callee_cdecl!(1, u32, aux) as u8 != 0 {
                let mut slot = this;
                let slot_ptr = (&mut slot as *mut u32) as u32;
                let filled: u32 =
                    lf_checker_rt::callee_cdecl!(2, u32, aux, this + LOCAL_POS, slot_ptr, FILL_TAG);
                if CLEAR_AUX {
                    wr32(this + AUX, 0);
                }
                if filled != 0 {
                    lf_checker_rt::callee_stdcall!(3, u32, ped, DISPATCH_MODE, 0);
                    return 0;
                }
                ((this + FLAG) as *mut u8).write_unaligned(1);
                if lf_checker_rt::callee_stdcall!(3, u32, ped, DISPATCH_MODE, 0) as u8 != 0 {
                    return lf_checker_rt::callee_thiscall!(4, u32, this, ped, KIND_SWAP_A);
                }
            }
        }
        if rd32(this + STATE) == ARMED {
            if kind_of(task) == KIND_ROUTE {
                let inner = rd32(task + INNER);
                if inner != 0 && kind_of(inner) == KIND_SWAP_A {
                    let picked: u32 = lf_checker_rt::callee_thiscall!(5, u32, task, ped);
                    if picked != 0 && kind_of(picked) == KIND_SWAP_A {
                        if lf_checker_rt::callee_thiscall!(6, u32, picked) == 2 {
                            let level = ((picked + LEVEL) as *const u8).read_unaligned();
                            if (level as i8) >= 2 {
                                if lf_checker_rt::callee_stdcall!(3, u32, ped, DISPATCH_MODE, 0) as u8
                                    != 0
                                {
                                    wr32(this + STATE, DONE);
                                    return lf_checker_rt::callee_thiscall!(
                                        4, u32, this, ped, KIND_WATER
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        task
    }
}

lf_checker_rt::export!(thiscall, rw_00cb0060(this: u32, ped: u32) -> u32 {
    decide::<true>(this, ped)
});
