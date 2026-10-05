// original: 0x00D72B90 CReplayOverlay::vf2

/// Second virtual slot of the replay overlay: refresh the overlay's text and
/// marker state for the current replay frame.
///
/// `this` is the overlay object. The control word at file address
/// `RELAY_CTRL` points at a relay object whose word at `RELAY_MODE` is set to
/// `MODE_REPLAY` (62) unconditionally. The refresh then runs only when the
/// overlay is active (byte at `FLAG_ACTIVE` nonzero), its inner object
/// (pointer at `INNER`) is attached (byte at `INNER_READY` zero), and the
/// readiness callee confirms (nonzero answer).
///
/// The refresh has two parts. First, two rounds of text lookup: each round
/// asks the text callee for a string slot (with a scratch buffer, a bank and
/// a size), resolves it through the pair of format callees, and emits it
/// through the sink callee. Second, the marker pass: the mode callee (the
/// function itself at slot 6, shared with the neighbouring entry) is primed
/// with modes 0 and 1, then one marker per lane for lanes 1..=17 is drawn:
/// the lane callee decides whether the lane is live; a live lane picks a
/// style from the per-object style words (`STYLE_A`, `STYLE_B`: style 62,
/// 1 or 59), draws its styled marker, positions it through the placement
/// pair, optionally pings it, and a final per-lane commit callee decides
/// whether an extra placement call is needed. A closing callee runs once
/// after the loop.
///
/// Scratch buffers passed to callees are uninitialised on both sides (the
/// checker fills them with a defined pattern); their addresses are skipped
/// in the call comparison and their contents snap-compared. Floats are only
/// moved, never computed. The CRT security-cookie check at the end is issued
/// through the stub table like any other callee.
///
/// The placement pair shares its stack frame: the two -1 words pushed ahead
/// of the placement call are the draw call's trailing arguments, and the
/// placement callee takes only the table word (plus its object in ECX).
///
/// Original: 0x00D72B90 (thiscall, no stack arguments; the two earliest
/// exits return the relay pointer still sitting in EAX, every other path
/// returns the last scripted callee answer).
lf_checker_rt::export!(thiscall, rw_00d72b90(this: u32) -> u32 {
    unsafe {
        const RELAY_CTRL: u32 = 0x0118E868;
        const RELAY_MODE: u32 = 0x44;
        const MODE_REPLAY: u32 = 0x3e;
        const INNER: u32 = 0x04;
        const INNER_READY: u32 = 0x1b;
        const FLAG_ACTIVE: u32 = 0x48;
        const STYLE_A: u32 = 0x3c;
        const STYLE_B: u32 = 0x40;
        const POS_Y: u32 = 0x5c;
        const POS_X: u32 = 0x60;
        const LANE_POS: u32 = 0x64;
        const LANE_FIRST: u32 = 1;
        const LANE_END: u32 = 0x12;
        const PLACE_TABLE: u32 = 0x01056908;
        const STYLE_MATCH_A: u32 = 0x3e;
        const STYLE_MATCH_B: u32 = 1;
        const STYLE_OTHER: u32 = 0x3b;

        const C_READY: u32 = 1;
        const C_TEXT: u32 = 2;
        const C_FMT_A: u32 = 3;
        const C_FMT_B: u32 = 4;
        const C_SINK: u32 = 5;
        const C_MODE: u32 = 6;
        const C_PLACE: u32 = 7;
        const C_DRAW: u32 = 8;
        const C_LANE: u32 = 9;
        const C_STYLE: u32 = 10;
        const C_PING: u32 = 11;
        const C_COMMIT: u32 = 12;
        const C_FINAL: u32 = 13;
        const C_CLOSE: u32 = 14;
        const C_COOKIE: u32 = 15;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let relay = rd32(lf_checker_rt::relocated(RELAY_CTRL));
        ((relay + RELAY_MODE) as *mut u32).write(MODE_REPLAY);

        if rd8(this + FLAG_ACTIVE) == 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return relay;
        }
        let inner = rd32(this + INNER);
        if rd8(inner + INNER_READY) != 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return relay;
        }
        let ready: u32 = lf_checker_rt::callee_thiscall!(C_READY, u32, inner);
        if ready == 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 0;
        }

        // Two rounds of text lookup and emit. The second round spills the
        // looked-up word into the scratch slot the second format call reads.
        let mut scratch = [0u32; 2];
        let sp = scratch.as_mut_ptr() as u32;
        lf_checker_rt::callee_cdecl!(C_TEXT, u32, sp, 2, 0xc8);
        lf_checker_rt::callee_cdecl!(C_FMT_A, u32, 0, 1);
        lf_checker_rt::callee_cdecl!(C_FMT_B, u32, this + 0x1c, sp);
        lf_checker_rt::callee_cdecl!(C_SINK, u32,);
        let slot: u32 = lf_checker_rt::callee_cdecl!(C_TEXT, u32, sp, 0x3d, 0xff);
        let word = rd32(slot);
        lf_checker_rt::callee_cdecl!(C_FMT_A, u32, 0, 1);
        scratch[0] = word;
        lf_checker_rt::callee_cdecl!(C_FMT_B, u32, this + 0x2c, sp);
        lf_checker_rt::callee_cdecl!(C_SINK, u32,);
        let mut lane_scratch = [0u32; 2];
        let lsp = lane_scratch.as_mut_ptr() as u32;

        // Marker pass.
        lf_checker_rt::callee_thiscall!(C_MODE, u32, this, 0);
        let place_obj = lf_checker_rt::relocated(0x0116BFF0);
        let place0 = rd32(lf_checker_rt::relocated(PLACE_TABLE));
        let h: u32 = lf_checker_rt::callee_thiscall!(C_PLACE, u32, place_obj, place0);
        lf_checker_rt::callee_cdecl!(C_DRAW, u32, rd32(this + POS_Y), rd32(this + POS_X), h, 0xffff_ffff, 0xffff_ffff);
        lf_checker_rt::callee_thiscall!(C_MODE, u32, this, 1);

        let mut lane = LANE_FIRST;
        let mut pos = this + LANE_POS;
        while lane < LANE_END {
            let live: u32 = lf_checker_rt::callee_thiscall!(C_LANE, u32, this, lane);
            if live == 0 {
                let style = if lane == rd32(this + STYLE_A) {
                    STYLE_MATCH_A
                } else if lane == rd32(this + STYLE_B) {
                    STYLE_MATCH_B
                } else {
                    STYLE_OTHER
                };
                let p: u32 = lf_checker_rt::callee_cdecl!(C_STYLE, u32, lsp, style);
                lf_checker_rt::callee_cdecl!(C_PING, u32, rd32(p));
                lf_checker_rt::callee_cdecl!(C_COMMIT, u32, 1);
                let g = rd32(lf_checker_rt::relocated(PLACE_TABLE + lane * 4));
                let h2: u32 = lf_checker_rt::callee_thiscall!(C_PLACE, u32, place_obj, g);
                lf_checker_rt::callee_cdecl!(C_DRAW, u32, rd32(pos), rd32(pos + 4), h2, 0xffff_ffff, 0xffff_ffff);
                lf_checker_rt::callee_cdecl!(C_COMMIT, u32, 2);
                let extra: u32 = lf_checker_rt::callee_thiscall!(C_FINAL, u32, this, lane, lsp);
                if extra != 0 {
                    lf_checker_rt::callee_cdecl!(C_DRAW, u32, rd32(pos), rd32(pos + 4), extra, 0xffff_ffff, 0xffff_ffff);
                }
            }
            lane += 1;
            pos += 8;
        }
        lf_checker_rt::callee_cdecl!(C_CLOSE, u32,);
        lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        0
    }
});
