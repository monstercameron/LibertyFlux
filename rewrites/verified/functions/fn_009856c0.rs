// original: 0x009856C0 audEmitter_update_slots (proposed)

/// Emitter slot update sweep: runs the per-slot solver on every ready slot
/// and clears the pending flag.
///
/// Does nothing (leaving everything untouched) when the global flag byte at
/// `PENDING` is clear. Otherwise walks the slot count (UNSIGNED) at
/// `+COUNT_OFF` of `this`, slots starting at `+SLOT0_OFF` and spaced
/// `SLOT_STRIDE` bytes apart. A slot is skipped unless its byte at `+HAS_A_OFF`
/// and its byte at `+HAS_B_OFF` are both set while its head dword is zero. The
/// target handle comes from the slot's helper object at `-HELPER_OFF` through
/// virtual slot 3 (indirect callee id 3) when the byte at `+MODE_OFF` is set,
/// else from the slot's word at `+HANDLE_OFF`; a null handle skips the slot.
/// Three floats from the slot (`+FX_OFF`/`+FY_OFF`/`+FZ_OFF`) are passed to
/// the solver (id 1, cdecl) together with an out-struct and the constant
/// `SOLVER_K`: the out-struct's first word starts at 0, and the solver's
/// scripted answer and written-back word go to the commit callee (id 2,
/// thiscall on the handle). The slot's byte at `+HAS_B_OFF` is then cleared,
/// and the loop bound is re-read through the written-back word, exactly as the
/// original re-reads its saved slot. Finally the global flag is cleared.
/// No meaningful return value.
/// Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_009856C0(this: u32) -> u32 {
    const PENDING: u32 = 0x1238954;
    const COUNT_OFF: u32 = 0x8230;
    const SLOT0_OFF: u32 = 0xdc;
    const SLOT_STRIDE: u32 = 0xd0;
    const HAS_A_OFF: u32 = 0x19;
    const HAS_B_OFF: u32 = 0x14;
    const MODE_OFF: u32 = 0x15;
    const HANDLE_OFF: u32 = 4;
    const HELPER_OFF: u32 = 0x9c;
    const VT_SLOT3: u32 = 0x0c;
    const FX_OFF: u32 = 0xa2;
    const FY_OFF: u32 = 0xa6;
    const FZ_OFF: u32 = 0xaa;
    const SOLVER_K: u32 = 0x41a00000;
    const SOLVER: u32 = 1;
    const COMMIT: u32 = 2;
    const HELPER_VF3: u32 = 3;
    unsafe {
        let pending = lf_checker_rt::global::<u8>(PENDING);
        if pending.read() == 0 {
            return 0;
        }
        let count0 = ((this + COUNT_OFF) as *const u32).read_unaligned();
        if count0 > 0 {
            let mut i = 0u32;
            let mut cur_this = this;
            while i < ((cur_this + COUNT_OFF) as *const u32).read_unaligned() {
                let slot = this
                    .wrapping_add(SLOT0_OFF)
                    .wrapping_add(i.wrapping_mul(SLOT_STRIDE));
                let ready = ((slot + HAS_A_OFF) as *const u8).read() != 0
                    && ((slot + HAS_B_OFF) as *const u8).read() != 0
                    && (slot as *const u32).read_unaligned() == 0;
                if ready {
                    let handle = if ((slot + MODE_OFF) as *const u8).read() != 0
                    {
                        let obj = slot.wrapping_sub(HELPER_OFF);
                        let vt = (obj as *const u32).read_unaligned();
                        let vf3: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(
                                ((vt + VT_SLOT3) as *const u32).read_unaligned()
                                    as usize,
                            );
                        vf3(obj)
                    } else {
                        ((slot + HANDLE_OFF) as *const u32).read_unaligned()
                    };
                    if handle != 0 {
                        let f0 = ((slot + FX_OFF) as *const u32).read_unaligned();
                        let f1 = ((slot + FY_OFF) as *const u32).read_unaligned();
                        let f2 = ((slot + FZ_OFF) as *const u32).read_unaligned();
                        // Same words the original's frame holds at the two
                        // struct pointers: (f0, f1, f2) and (0, this, f0).
                        let floats = [f0, f1, f2];
                        let mut out = [0u32, this, f0];
                        let ans: u32 = lf_checker_rt::callee_cdecl!(
                            SOLVER,
                            u32,
                            floats.as_ptr() as u32,
                            out.as_mut_ptr() as u32,
                            SOLVER_K
                        );
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            COMMIT, u32, handle, ans, out[0]
                        );
                        cur_this = out[0];
                        ((slot + HAS_B_OFF) as *mut u8).write(0);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        pending.write(0);
    }
    0
});
