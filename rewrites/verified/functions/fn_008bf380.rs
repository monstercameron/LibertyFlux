// original: 0x008bf380 ui_state_reset (proposed)

/// Reset the UI/input subsystem's cached state and re-poll its dependencies.
///
/// Calls the slot-reset helper (a thiscall taking no stack arguments) once
/// for each of four fixed slots and once for each of four looped slots, then
/// asks the gate helper whether a full re-acquire is needed (its low byte
/// decides; zero means re-acquire). The re-acquire path clears the flag
/// dword, and when the flag byte is set runs one extra refresh helper. When
/// the mode word equals `MODE_STREAMING` the texture-dictionary slot for a
/// fixed name is looked up and, unless the lookup reports missing (-1, an
/// exact equality test), handed to the bind helper. Two further refresh
/// helpers run unconditionally, a busy byte is set, a state object is poked
/// through a thiscall, and a pending helper runs when the pending byte is
/// set (the pending byte and two flag bytes are then cleared). Returns the
/// answer of the finalizer helper, or of the deferred helper when the
/// deferred byte is set (which also clears that byte).
///
/// Original: 0x008bf380 (cdecl, no arguments; callee-saved esi preserved).
lf_checker_rt::export!(cdecl, rw_008bf380() -> u32 {
    unsafe {
        const SLOT_FIXED: [u32; 4] = [0x1161510, 0x1161818, 0x116181c, 0x1161820];
        const SLOT_LOOP_FIRST: u32 = 0x1161800;
        const SLOT_LOOP_END: u32 = 0x1161810;
        const FLAG_BYTE: u32 = 0x1160c28;
        const FLAG_DWORD: u32 = 0x1160c2c;
        const MODE_WORD: u32 = 0x1030b7c;
        const MODE_STREAMING: u32 = 0x34;
        const TXD_NAME: u32 = 0xe7df10;
        const MISSING: u32 = 0xFFFFFFFF;
        const STATE_OBJ: u32 = 0x11737d0;
        const BUSY_BYTE: u32 = 0x118f4bc;
        const PENDING_BYTE: u32 = 0x1160b84;
        const FLAG2: u32 = 0x11609f6;
        const FLAG3: u32 = 0x1160c32;
        const DEFERRED_BYTE: u32 = 0x1160b87;

        for s in SLOT_FIXED {
            lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(s));
        }
        let mut slot = SLOT_LOOP_FIRST;
        // Signed bound comparison in the original; both sides are small
        // positive constants, so the four iterations are exact.
        while (slot as i32) < SLOT_LOOP_END as i32 {
            lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(slot));
            slot += 4;
        }
        let gate: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        if (gate as u8) == 0 {
            lf_checker_rt::callee_cdecl!(3, u32,);
            lf_checker_rt::callee_cdecl!(4, u32,);
            *lf_checker_rt::global::<u32>(FLAG_DWORD) = 0;
            if *lf_checker_rt::global::<u8>(FLAG_BYTE) != 0 {
                lf_checker_rt::callee_cdecl!(5, u32,);
            }
        }
        if *lf_checker_rt::global::<u32>(MODE_WORD) == MODE_STREAMING {
            let found: u32 =
                lf_checker_rt::callee_cdecl!(6, u32, lf_checker_rt::relocated(TXD_NAME));
            if found != MISSING {
                lf_checker_rt::callee_cdecl!(7, u32, found);
            }
        }
        lf_checker_rt::callee_cdecl!(8, u32, 0u32);
        lf_checker_rt::callee_cdecl!(9, u32, 0u32);
        *lf_checker_rt::global::<u8>(BUSY_BYTE) = 1;
        lf_checker_rt::callee_thiscall!(10, u32, lf_checker_rt::relocated(STATE_OBJ), 0u32);
        lf_checker_rt::callee_cdecl!(11, u32,);
        if *lf_checker_rt::global::<u8>(PENDING_BYTE) != 0 {
            lf_checker_rt::callee_cdecl!(12, u32,);
        }
        *lf_checker_rt::global::<u8>(PENDING_BYTE) = 0;
        *lf_checker_rt::global::<u8>(FLAG2) = 0;
        *lf_checker_rt::global::<u8>(FLAG3) = 0;
        lf_checker_rt::callee_cdecl!(13, u32, 0u32);
        // Stack order is (0, -1), so the first argument is -1.
        let r: u32 = lf_checker_rt::callee_cdecl!(14, u32, 0xFFFFFFFFu32, 0u32);
        if *lf_checker_rt::global::<u8>(DEFERRED_BYTE) != 0 {
            let r2: u32 = lf_checker_rt::callee_cdecl!(15, u32,);
            *lf_checker_rt::global::<u8>(DEFERRED_BYTE) = 0;
            r2
        } else {
            r
        }
    }
});
