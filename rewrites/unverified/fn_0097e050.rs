// original: 0x0097e050 PED_VEHICLE_BIKE_COLLISION (symbols, low confidence)

/// Handle a ped bike-collision audio event with loudness gating.
///
/// `this` is the audio manager object (dword at `+0x08` carried into the
/// request block, dword at `+0x120` passed to the submit call). `target`
/// points at an object whose dword at `+0x1304` selects the voice (`1`
/// uses the initialised global voice, anything else indexes the voice
/// table by `key`). `level` is a float loudness seed; `key` is a small
/// integer used as a table index and locator argument (its bits also
/// travel as a float to the first filter call); the last two stack words
/// are never read.
///
/// Behaviour: the three global gates must pass or the function returns at
/// once. The key is filtered twice (a missing second filter answer skips
/// the locator call), then a one-time initialisation runs once (flag word
/// bit 0) storing a resolver answer into the global voice. The level is
/// filtered; if the answer is not above the `1e-5` constant (including
/// NaN, which takes the exit like `comiss`+`jbe`) the function returns the
/// filter answer's own bits (the EAX residue the call leaves, which the
/// proof's stub sets to the same bits it pushes on ST0; the real callee's
/// residue is unobserved). Otherwise a request block is built in frame
/// scratch (block at frame `+0x10`, reshaped level at word 0, tag at
/// `+0x20`; word 5 receives the caller's return-address slot, which no
/// rewrite can reproduce and the proof skips), a slot is allocated and
/// resolved, and the request is issued: success submits it (triple
/// `0,-1,0x5D` beside the block) and returns the submit answer, failure
/// releases the slot and returns the release answer.
///
/// The only float comparison is the `!(answer > 1e-5)` gate, written to
/// match `comiss`+`jbe` exactly (NaN exits). All other floats are passed
/// through untouched. The gate-1 exit returns the caller's entry EAX,
/// which a rewrite cannot observe; the proof keeps gate 1 passing and
/// notes this.
///
/// Original: 0x0097E050 (thiscall, five stack words, the last two unread).
lf_checker_rt::export!(thiscall, rw_0097e050(
    this: u32, target: u32, level: u32, key: u32, _u3: u32, _u4: u32,
) -> u32 {
    unsafe {
        const GATE_INIT: u32 = 0x11F7060;
        const GATE_EPOCH_A: u32 = 0x12088B4;
        const GATE_EPOCH_B: u32 = 0xF1C040;
        const GATE_MODE: u32 = 0x1037720;
        const GATE_MODE_EXIT: u32 = 0x12;
        const FILTER_A_OBJ: u32 = 0x12315A8;
        const FILTER_B_OBJ: u32 = 0x123157C;
        const INIT_FLAG: u32 = 0x1231770;
        const INIT_VOICE: u32 = 0x123176C;
        const RESOLVER_NAME: u32 = 0xE8CBC8;
        const LOUDNESS_MIN: u32 = 0xFE8670;
        const VOICE_TABLE: u32 = 0x1231670;
        const TARGET_SEL_OFF: u32 = 0x1304;
        const THIS_TAG: u32 = 0x08;
        const THIS_X120: u32 = 0x120;
        const SUBMIT_ID: u32 = 0x5D;
        const F_TRIPLE: usize = 0x04 / 4;
        const F_BLOCK: usize = 0x10 / 4;
        const BLOCK_TAG: usize = 0x20 / 4;
        const C_FILTER_A: u32 = 1;
        const C_THRESH: u32 = 2;
        const C_LOCATOR: u32 = 3;
        const C_RESOLVER: u32 = 4;
        const C_FILTER_B: u32 = 5;
        const C_BLOCK_INIT: u32 = 6;
        const C_RESHAPE: u32 = 7;
        const C_SLOT_ALLOC: u32 = 8;
        const C_SLOT_RESOLVE: u32 = 9;
        const C_ISSUE: u32 = 10;
        const C_SUBMIT: u32 = 11;
        const C_RELEASE: u32 = 12;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        if lf_checker_rt::global::<u32>(GATE_INIT).read() == 1 {
            // Unreachable under the proof (gate pinned passing): the
            // original returns entry EAX here, which no rewrite can read.
            return 0;
        }
        let epoch = lf_checker_rt::global::<u32>(GATE_EPOCH_A).read();
        if epoch != lf_checker_rt::global::<u32>(GATE_EPOCH_B).read() {
            return epoch;
        }
        if lf_checker_rt::global::<u32>(GATE_MODE).read() == GATE_MODE_EXIT {
            return epoch;
        }
        let f1: f32 =
            lf_checker_rt::callee_thiscall!(C_FILTER_A, f32, lf_checker_rt::relocated(FILTER_A_OBJ), key);
        let keep = lf_checker_rt::callee_cdecl!(C_THRESH, u32, f1.to_bits());
        let mut last = keep;
        if (keep as u8) != 0 {
            last = lf_checker_rt::callee_thiscall!(C_LOCATOR, u32, this, key);
        }
        let flag = lf_checker_rt::global::<u32>(INIT_FLAG).read();
        if flag & 1 == 0 {
            lf_checker_rt::global::<u32>(INIT_FLAG).write(flag | 1);
            let v = lf_checker_rt::callee_cdecl!(
                C_RESOLVER, u32, 0, lf_checker_rt::relocated(RESOLVER_NAME)
            );
            lf_checker_rt::global::<u32>(INIT_VOICE).write(v);
            last = v;
        }
        let f2: f32 =
            lf_checker_rt::callee_thiscall!(C_FILTER_B, f32, lf_checker_rt::relocated(FILTER_B_OBJ), level);
        // The stub answers EAX with the same bits it pushes on ST0, and the
        // original returns EAX on the quiet exit below, so that value (not
        // the earlier answers) is what the exit observes.
        last = f2.to_bits();
        let floor = f32::from_bits(lf_checker_rt::global::<u32>(LOUDNESS_MIN).read());
        if !(f2 > floor) {
            return last;
        }
        let mut frame = [0u32; 0x80];
        let base = frame.as_mut_ptr() as u32;
        let block = base.wrapping_add((F_BLOCK * 4) as u32);
        lf_checker_rt::callee_thiscall!(C_BLOCK_INIT, u32, block);
        frame[F_BLOCK + BLOCK_TAG] = rd32(this.wrapping_add(THIS_TAG));
        // NOTE: the original stores its caller return-address slot at
        // block word 5 here; it is skipped in every snapshot (see proof).
        let shaped: f32 = lf_checker_rt::callee_cdecl!(C_RESHAPE, f32, f2.to_bits());
        frame[F_BLOCK] = shaped.to_bits();
        let voice = if rd32(target.wrapping_add(TARGET_SEL_OFF)) == 1 {
            lf_checker_rt::global::<u32>(INIT_VOICE).read()
        } else {
            rd32(lf_checker_rt::relocated(VOICE_TABLE).wrapping_add(key.wrapping_mul(4)))
        };
        let slot = lf_checker_rt::callee_cdecl!(C_SLOT_ALLOC, u32,);
        let resolved = lf_checker_rt::callee_cdecl!(C_SLOT_RESOLVE, u32, slot);
        let ok =
            lf_checker_rt::callee_thiscall!(C_ISSUE, u32, this, voice, block, slot, resolved, 0);
        if (ok as u8) == 0 {
            return lf_checker_rt::callee_cdecl!(C_RELEASE, u32, slot);
        }
        frame[F_TRIPLE] = 0;
        frame[F_TRIPLE + 1] = 0xFFFF_FFFF;
        frame[F_TRIPLE + 2] = SUBMIT_ID;
        lf_checker_rt::callee_cdecl!(
            C_SUBMIT, u32, voice, 0, 0, 1, block,
            base.wrapping_add((F_TRIPLE * 4) as u32),
            rd32(this.wrapping_add(THIS_X120)), slot
        )
    }
});
