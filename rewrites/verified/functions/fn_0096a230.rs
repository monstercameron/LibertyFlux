// original: 0x0096A230 DEFAULT_OFFICE

/// Resolve an object to a result handle and classify it (stdcall, two stack
/// words, callee pops 8).
///
/// `obj` is an object pointer (may be null) with a flag word at `+0x24` (bit
/// 0x8000000 means "has a pool handle"), a class byte at `+0x40` and a handle
/// at `+0x48`. `out` receives one status byte and is written before anything
/// else is read, so a null `out` faults before any other behaviour.
///
/// Behaviour: clear `*out`; when the mode byte (global) is set, return the
/// early constant address. Otherwise, when `obj` is non-null and the flag bit
/// is set, look the handle up in the pool (callee 1, `this` from the pool
/// global). Read the class byte and sign-extend it through 16 bits (`movsx
/// ax, byte; (an instruction of the original)`, so bytes 0x80..0xFF are negative); a null pool
/// entry, a negative class or class 0x3F ends the call returning 0. Else use
/// the pool entry's signed word at `+0x2E` as an index into the dword table to
/// fetch a row, pass the row's slot at `+0x70` with the sign-extended class to
/// callee 2, copy the returned handle's byte at `+0x58` to `*out`, and return
/// its dword at `+0x54` when that is non-zero. Otherwise ask the two flag
/// callees (4 and 5, low byte of the answer only) and return whatever the
/// classify callee (6, fixed `this`, one of three constant arguments) answers.
///
/// The original's `(an instruction of the original)` after the second call writes a dead
/// value to below-ESP scratch; it is unobserved and omitted here.
///
/// NOT covered by this proof: the rebuild path gated by the second mode byte
/// (global+1). It passes a pointer into the return-address slot to callee 3
/// and reads the slot back afterwards, so its behaviour depends on the live
/// return address; the contract pins that byte to 0 and callee 3 is never
/// called. See the `narrowed` record.
lf_checker_rt::export!(stdcall, rw_0096A230(obj: u32, out: u32) -> u32 {
    unsafe {
        const MODE_FLAG: u32 = 0x12184a0;
        const EARLY_RESULT: u32 = 0x1037a60;
        const POOL_GLOBAL: u32 = 0x12fb214;
        const HANDLE_TABLE: u32 = 0x1295cd8;
        const CLASS_TABLE_THIS: u32 = 0x115d9a0;
        const HANDLE_FLAG: u32 = 0x0800_0000;
        const EXCLUDED_CLASS: u16 = 0x003f;
        const OFF_FLAGS: u32 = 0x24;
        const OFF_HANDLE: u32 = 0x48;
        const OFF_CLASS: u32 = 0x40;
        const OFF_POOL_CLASS: u32 = 0x2e;
        const OFF_ROW_SLOT: u32 = 0x70;
        const OFF_RES_VALUE: u32 = 0x54;
        const OFF_RES_BYTE: u32 = 0x58;
        const C_POOL_LOOKUP: u32 = 1;
        const C_ROW_HANDLE: u32 = 2;
        const C_FLAG_A: u32 = 4;
        const C_FLAG_B: u32 = 5;
        const C_CLASSIFY: u32 = 6;
        const ARG_A: u32 = 0xe8b55c;
        const ARG_B: u32 = 0xe8b56c;
        const ARG_C: u32 = 0xe8b57c;

        (out as *mut u8).write(0);
        if lf_checker_rt::global::<u8>(MODE_FLAG).read() != 0 {
            return lf_checker_rt::relocated(EARLY_RESULT);
        }
        let mut pool_obj = 0u32;
        if obj != 0 {
            // Barrier: keep these loads inside the null guard and in order.
            let base = core::hint::black_box(obj);
            if (base.wrapping_add(OFF_FLAGS) as *const u32).read_unaligned() & HANDLE_FLAG != 0 {
                let handle =
                    (base.wrapping_add(OFF_HANDLE) as *const u32).read_unaligned();
                let pool = lf_checker_rt::global::<u32>(POOL_GLOBAL).read_unaligned();
                pool_obj = lf_checker_rt::callee_thiscall!(C_POOL_LOOKUP, u32, pool, handle);
            }
        }
        // Sign-extended through 16 bits: bytes 0x80..0xFF read back negative.
        let cls =
            (core::hint::black_box(obj.wrapping_add(OFF_CLASS)) as *const i8).read() as i16 as u16;
        // Barrier: the original faults on this load (null obj) before testing
        // pool_obj; without materialising cls here the load sinks below the
        // branch and the fault is lost (probe2: 25 fault/ok).
        core::hint::black_box(cls);
        if pool_obj == 0 {
            return 0;
        }
        if (cls as i16) < 0 {
            return 0;
        }
        if cls == EXCLUDED_CLASS {
            return 0;
        }
        let idx = (pool_obj.wrapping_add(OFF_POOL_CLASS) as *const i16).read_unaligned() as i32;
        let row = (lf_checker_rt::relocated(HANDLE_TABLE).wrapping_add((idx as u32).wrapping_mul(4))
            as *const u32)
            .read_unaligned();
        let slot = (row.wrapping_add(OFF_ROW_SLOT) as *const u32).read_unaligned();
        let h = lf_checker_rt::callee_thiscall!(C_ROW_HANDLE, u32, slot, cls as i16 as i32 as u32);
        let hb = core::hint::black_box(h);
        // Order matters for faults (null h faults at +0x58 first): pin it.
        let hb_byte = (hb.wrapping_add(OFF_RES_BYTE) as *const u8).read();
        core::hint::black_box(hb_byte);
        (out as *mut u8).write(hb_byte);
        let res = (hb.wrapping_add(OFF_RES_VALUE) as *const u32).read_unaligned();
        if res != 0 {
            return res;
        }
        let fa = lf_checker_rt::callee_thiscall!(C_FLAG_A, u32, pool_obj);
        if (fa & 0xFF) != 0 {
            return lf_checker_rt::callee_thiscall!(
                C_CLASSIFY,
                u32,
                lf_checker_rt::relocated(CLASS_TABLE_THIS),
                lf_checker_rt::relocated(ARG_A)
            );
        }
        let fb = lf_checker_rt::callee_thiscall!(C_FLAG_B, u32, pool_obj);
        if (fb & 0xFF) != 0 {
            return lf_checker_rt::callee_thiscall!(
                C_CLASSIFY,
                u32,
                lf_checker_rt::relocated(CLASS_TABLE_THIS),
                lf_checker_rt::relocated(ARG_B)
            );
        }
        lf_checker_rt::callee_thiscall!(C_CLASSIFY, u32, lf_checker_rt::relocated(CLASS_TABLE_THIS), lf_checker_rt::relocated(ARG_C))
    }
});
