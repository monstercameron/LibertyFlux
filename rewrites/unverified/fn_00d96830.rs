// original: 0x00d96830 timer_stamp_guarded_slots (proposed)

/// Stamp the current timer tick into every armed timer slot.
///
/// Reads one 32-bit tick from the timer global and copies it into each
/// destination word whose guard word is non-zero: three strided table loops
/// (32, 16 and 16 slots) and two groups of eight individually addressed
/// guard/value pairs. A destination is written only when its guard is
/// non-zero; the value words themselves are never read. The whole update is
/// bracketed by three intercepted lock calls (thiscall): an acquire with a
/// constant lock id that pops its word, then two releases, all addressed at
/// the same eight-byte local.
///
/// Arguments: none. Incoming registers are ignored except ESI, whose saved
/// copy is untouched by the stubs and restored on exit. All three calls take
/// the same local buffer. Returns the third lock call's answer. Convention:
/// cdecl, no stack words.
lf_checker_rt::export!(cdecl, rw_00d96830() -> u32 {
    unsafe {
        const TICK: u32 = 0x0117_35b4;
        const LOCK_ID: u32 = 0x016c_5360;
        // Table loops as (first slot, end (exclusive), stride, guard back-offset).
        const LOOP_A_FIRST: u32 = 0x016b_9fd8;
        const LOOP_A_END: u32 = 0x016c_19d8;
        const LOOP_A_STRIDE: u32 = 0x3d0;
        const LOOP_A_BACK: u32 = 0x18;
        const LOOP_B_FIRST: u32 = 0x016c_76dc;
        const LOOP_B_END: u32 = 0x016c_83dc;
        const LOOP_B_STRIDE: u32 = 0xd0;
        const LOOP_B_BACK: u32 = 0x0c;
        const LOOP_C_FIRST: u32 = 0x016c_1fdc;
        const LOOP_C_END: u32 = 0x016c_3cdc;
        const LOOP_C_STRIDE: u32 = 0x1d0;
        const LOOP_C_BACK: u32 = 0x0c;
        const PAIR_A_GUARDS: [u32; 8] = [
            0x016c_40e0, 0x016c_4330, 0x016c_4580, 0x016c_47d0, 0x016c_4a20, 0x016c_4c70,
            0x016c_4ec0, 0x016c_5110,
        ];
        const PAIR_B_GUARDS: [u32; 8] = [
            0x016c_5390, 0x016c_5600, 0x016c_5870, 0x016c_5ae0, 0x016c_5d50, 0x016c_5fc0,
            0x016c_6230, 0x016c_64a0,
        ];
        const PAIR_VALUE_OFF: u32 = 0x0c;
        const ACQUIRE: u32 = 1;
        const RELEASE_A: u32 = 2;
        const RELEASE_B: u32 = 3;

        #[inline(always)]
        unsafe fn guarded(guard: u32, slot: u32, stamp: u32) {
            unsafe {
                if lf_checker_rt::global::<u32>(guard).read() != 0 {
                    lf_checker_rt::global::<u32>(slot).write(stamp);
                }
            }
        }

        let mut guard = [0u32; 2];
        let stamp = lf_checker_rt::global::<u32>(TICK).read();
        lf_checker_rt::callee_thiscall!(
            ACQUIRE,
            u32,
            guard.as_mut_ptr() as u32,
            lf_checker_rt::relocated(LOCK_ID)
        );
        let mut slot = LOOP_A_FIRST;
        while slot < LOOP_A_END {
            guarded(slot - LOOP_A_BACK, slot, stamp);
            slot += LOOP_A_STRIDE;
        }
        for g in PAIR_A_GUARDS {
            guarded(g, g + PAIR_VALUE_OFF, stamp);
        }
        slot = LOOP_B_FIRST;
        while slot < LOOP_B_END {
            guarded(slot - LOOP_B_BACK, slot, stamp);
            slot += LOOP_B_STRIDE;
        }
        slot = LOOP_C_FIRST;
        while slot < LOOP_C_END {
            guarded(slot - LOOP_C_BACK, slot, stamp);
            slot += LOOP_C_STRIDE;
        }
        for g in PAIR_B_GUARDS {
            guarded(g, g + PAIR_VALUE_OFF, stamp);
        }
        lf_checker_rt::callee_thiscall!(RELEASE_A, u32, guard.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(RELEASE_B, u32, guard.as_mut_ptr() as u32)
    }
});
