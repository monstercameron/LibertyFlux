// original: 0x00ad3560 audio_channel_lookup
/// Select an audio channel object by index and run the two-stage apply.
///
/// The selector (`sel`) indexes a 24-entry jump table. Indices 0 and 1 load
/// their channel pointer from its global slot and set the control code from
/// the probe callee's answer (non-zero becomes 1). Indices 5, 6, 7 share one
/// pointer slot with codes 1, 2, 3; indices 11, 12 share one slot with codes
/// 1, 2. Index 2, index 4, index 10 and every other listed index load their
/// slot's pointer (index 2 loads none and passes null) and reuse the incoming
/// scratch word as the code. Any selector above 23 passes null with the
/// scratch word. The tail then calls the apply callee with (0, 1, pointer)
/// and the commit callee with (code), both with the shared object from its
/// global slot, and returns the commit callee's answer.
///
/// The scratch word is the caller's stack slot above the argument, which the
/// original uses as outgoing temporary storage; cases that set a constant
/// code overwrite it, the rest read it through. The contract declares it as
/// a second stack word so both sides read the same value, and the stack
/// check is off because the original writes that slot while the rewrite
/// keeps the code in a local (the value itself is verified through the
/// commit call's argument).
///
/// Original: 0x00ad3560 (cdecl, one stack word plus the scratch slot).
lf_checker_rt::export!(cdecl, rw_00ad3560(sel: u32, scratch: u32) -> u32 {
    unsafe {
        const SHARED_OBJ: u32 = 0x0154e194;
        const SLOT_0: u32 = 0x0154e198;
        const SLOT_1: u32 = 0x0154e19c;
        const SLOT_SHARED_A: u32 = 0x0154e1a0;
        const SLOT_SHARED_B: u32 = 0x0154e1a4;
        const SLOT_3: u32 = 0x0154e1a8;
        const SLOT_8: u32 = 0x0154e1ac;
        const SLOT_9: u32 = 0x0154e1b0;
        const SLOT_19: u32 = 0x0154e1b4;
        const SLOT_20: u32 = 0x0154e1b8;
        const SLOT_13: u32 = 0x0154e1bc;
        const SLOT_14: u32 = 0x0154e1c0;
        const SLOT_15: u32 = 0x0154e1c4;
        const SLOT_16: u32 = 0x0154e1c8;
        const SLOT_17: u32 = 0x0154e1cc;
        const SLOT_23: u32 = 0x0154e1d0;
        const SLOT_18: u32 = 0x0154e1d4;
        const SLOT_21: u32 = 0x0154e1d8;
        const SLOT_22: u32 = 0x0154e1dc;
        const PROBE_CALLEE: u32 = 1;
        const APPLY_CALLEE: u32 = 2;
        const COMMIT_CALLEE: u32 = 3;
        #[inline(always)]
        unsafe fn slot(addr: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(addr).read() }
        }
        let (ptr, code): (u32, u32) = match sel {
            0 => {
                let p = slot(SLOT_0);
                let ans: u32 =
                    lf_checker_rt::callee_cdecl!(PROBE_CALLEE, u32,);
                (p, u32::from((ans as u8) != 0))
            }
            1 => {
                let p = slot(SLOT_1);
                let ans: u32 =
                    lf_checker_rt::callee_cdecl!(PROBE_CALLEE, u32,);
                (p, u32::from((ans as u8) != 0))
            }
            2 => (0, scratch),
            3 => (slot(SLOT_3), scratch),
            4 => (slot(SLOT_SHARED_A), scratch),
            5 => (slot(SLOT_SHARED_A), 1),
            6 => (slot(SLOT_SHARED_A), 2),
            7 => (slot(SLOT_SHARED_A), 3),
            8 => (slot(SLOT_8), scratch),
            9 => (slot(SLOT_9), scratch),
            10 => (slot(SLOT_SHARED_B), scratch),
            11 => (slot(SLOT_SHARED_B), 1),
            12 => (slot(SLOT_SHARED_B), 2),
            13 => (slot(SLOT_13), scratch),
            14 => (slot(SLOT_14), scratch),
            15 => (slot(SLOT_15), scratch),
            16 => (slot(SLOT_16), scratch),
            17 => (slot(SLOT_17), scratch),
            18 => (slot(SLOT_18), scratch),
            19 => (slot(SLOT_19), scratch),
            20 => (slot(SLOT_20), scratch),
            21 => (slot(SLOT_21), scratch),
            22 => (slot(SLOT_22), scratch),
            23 => (slot(SLOT_23), scratch),
            _ => (0, scratch),
        };
        let shared = slot(SHARED_OBJ);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            APPLY_CALLEE, u32, shared, 0u32, 1u32, ptr);
        lf_checker_rt::callee_thiscall!(COMMIT_CALLEE, u32, shared, code)
    }
});
