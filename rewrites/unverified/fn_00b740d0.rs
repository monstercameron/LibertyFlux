// original: 0x00b740d0 HIGH_FALL (symbols)

/// Build the high-fall task for a falling ped.
///
/// `this` is the task object (state number at `+STATE`, host object at
/// `+HOST`, created follow-up stored at `+TASK`); `arg` is the host-side
/// record (callback object at `+CALLBACK`, effect anchor at `+ANCHOR`,
/// probe object at `+PROBE`).
///
/// Behaviour: two float slots start at `K_A` and `K_B` and the request
/// code at `0x156`. The state selects a sub-case through a byte map plus
/// jump table: states 5-6 keep `0x156`, 7-8 set `0x157`, 28 runs the
/// effect path, and every other state keeps `0x156`. The effect path
/// reloads the slots from `K_A2` (game data) and `K_B2`, resolves an
/// effect id through callee 0, triggers it through callee 1 (ten stack
/// words, `1.0` in the eighth), then probes through callee 2: a non-zero
/// answer sets the code to `0x15A`, zero sets `0x15B`. The code is passed
/// by reference to callee 3 with the kind word from the host's table
/// entry (`TABLE[idx]` at `+KIND`, `idx` the signed word at `+MODEL`);
/// callee 3 answers in `ebx` and overwrites the slot. Callee 4 creates
/// the follow-up task from (`ebx`, overwritten slot, slot B, -1) and the
/// slot-A value is stored at `+DURATION`; callee 5 attaches the task.
/// Returns callee 5's answer.
///
/// Original: 0x00b740d0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b740d0(this: u32, arg: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x24;
        const HOST: u32 = 0x1c;
        const TASK: u32 = 0x18;
        const MODEL: u32 = 0x2e;
        const KIND: u32 = 0xc4;
        const TABLE: u32 = 0x01295cd8;
        const CALLBACK: u32 = 0x78;
        const ANCHOR: u32 = 0x570;
        const PROBE: u32 = 0xb30;
        const DURATION: u32 = 0x54;
        const K_A: u32 = 0x00fe88e8;
        const K_B: u32 = 0x00fe8afc;
        const K_A2: u32 = 0x01046b44;
        const K_B2: u32 = 0x00fe8b28;
        const P0: u32 = 0x00eb189c;
        const P1: u32 = 0x00eb18a8;
        const VTBL: u32 = 0x00b6fe10;
        const ONE: u32 = 0x3f80_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }

        let mut slot_a = rdf(lf_checker_rt::relocated(K_A));
        let mut slot_b = rdf(lf_checker_rt::relocated(K_B));
        let mut code: u32 = 0x156;
        match rd32(this + STATE) {
            5 | 6 => code = 0x156,
            7 | 8 => code = 0x157,
            28 => {
                slot_a = rdf(lf_checker_rt::relocated(K_A2));
                slot_b = rdf(lf_checker_rt::relocated(K_B2));
                let effect: u32 = lf_checker_rt::callee_cdecl!(
                    0,
                    u32,
                    lf_checker_rt::relocated(P0),
                    0
                );
                lf_checker_rt::callee_thiscall!(
                    1,
                    u32,
                    arg.wrapping_add(ANCHOR),
                    lf_checker_rt::relocated(P1),
                    1,
                    0,
                    effect,
                    0xffff_ffff,
                    0,
                    0,
                    ONE,
                    0,
                    0
                );
                let probe: u32 =
                    lf_checker_rt::callee_thiscall!(2, u32, rd32(arg + PROBE), arg);
                // neg/sbb/add: -1 when the low byte is non-zero, else 0, plus 0x15b.
                code = 0x15b_u32.wrapping_add(if probe & 0xff != 0 {
                    0xffff_ffff
                } else {
                    0
                });
            }
            _ => {}
        }
        let host = rd32(this + HOST);
        let idx = rd16(host + MODEL) as u16 as i16 as i32;
        let entry = rd32(
            lf_checker_rt::relocated(TABLE).wrapping_add((idx as u32).wrapping_mul(4)),
        );
        let kind = rd32(entry + KIND);
        // As in the sibling constructor: the slot carries the code into
        // callee 3 and its answer out.
        let mut slot = code;
        let ebx: u32 = lf_checker_rt::callee_cdecl!(
            3,
            u32,
            kind,
            &mut slot as *mut u32 as u32,
            arg,
            host,
            0,
            0,
            0,
            1
        );
        let task: u32 = lf_checker_rt::callee_thiscall!(
            4,
            u32,
            rd32(arg + CALLBACK),
            ebx,
            slot,
            slot_b.to_bits(),
            0xffff_ffff
        );
        wr32(this + TASK, task);
        wrf(task + DURATION, slot_a);
        lf_checker_rt::callee_thiscall!(
            5,
            u32,
            task,
            1,
            lf_checker_rt::relocated(VTBL),
            this
        )
    }
});
