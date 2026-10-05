// original: 0x00AD5A00 audio_tick_or_reset (proposed)

/// Advance the audio tick or reset the stalled frame counter.
///
/// Reads the window-minimised state (imported IsIconic on the saved window)
/// combined with two override bytes into a flag: minimised, or either
/// override set, means active. When inactive, or when active with the latch
/// set, the epoch words are refreshed (the latch stores the current tick
/// and clears). Otherwise the saved frame base must still be within 0x10
/// below the tick or the base is reset to zero and its stale value plus
/// 0x10 returned. The refresh divides the tick by the rate helper's answer
/// (cdecl/0, never zero in the proof), folds a zero remainder into one
/// epoch word, bumps the tick and derives the other epoch word. Takes no
/// arguments (cdecl/0).
lf_checker_rt::export!(cdecl, rw_00ad5a00() -> u32 {
    unsafe {
        const IS_ICONIC: u32 = 1;
        const RATE: u32 = 2;
        const WINDOW: u32 = 0x017ACCD8;
        const OVERRIDE_A: u32 = 0x0105B48F;
        const OVERRIDE_B: u32 = 0x017ED8D1;
        const LATCH_A: u32 = 0x01173590;
        const LATCH_B: u32 = 0x01173591;
        const TICK: u32 = 0x0103F4CC;
        const LATCH: u32 = 0x01550E7C;
        const FRAME_BASE: u32 = 0x0154E2A0;
        const EPOCH_A: u32 = 0x01550DF4;
        const EPOCH_B: u32 = 0x01550DF8;
        const STALE_WINDOW: u32 = 0x10;
        let hwnd = lf_checker_rt::global::<u32>(WINDOW).read();
        let iconic = lf_checker_rt::callee_stdcall!(IS_ICONIC, u32, hwnd);
        let mut active = if iconic != 0 {
            1u8
        } else if lf_checker_rt::global::<u8>(OVERRIDE_A).read() == 0 {
            0u8
        } else if lf_checker_rt::global::<u8>(OVERRIDE_B).read() != 0 {
            1u8
        } else {
            0u8
        };
        active |= lf_checker_rt::global::<u8>(LATCH_A).read();
        let tick = lf_checker_rt::global::<u32>(TICK).read();
        active |= lf_checker_rt::global::<u8>(LATCH_B).read();
        if active != 0 {
            if lf_checker_rt::global::<u8>(LATCH).read() == 0 {
                let stale = lf_checker_rt::global::<u32>(FRAME_BASE).read().wrapping_add(STALE_WINDOW);
                if (stale as i32) < (tick as i32) {
                    lf_checker_rt::global::<u32>(FRAME_BASE).write(0);
                    return stale;
                }
            } else {
                lf_checker_rt::global::<u32>(FRAME_BASE).write(tick);
                lf_checker_rt::global::<u8>(LATCH).write(0);
            }
        }
        let rate = lf_checker_rt::callee_cdecl!(RATE, u32,);
        let quot = tick / rate;
        let rem = tick % rate;
        let ret = if rem == 0 {
            let v = lf_checker_rt::global::<u32>(EPOCH_A).read();
            let nv = 1u32.wrapping_sub(v);
            lf_checker_rt::global::<u32>(EPOCH_A).write(nv);
            nv
        } else {
            quot
        };
        let w = lf_checker_rt::global::<u32>(EPOCH_B).read();
        lf_checker_rt::global::<u32>(TICK).write(tick.wrapping_add(1));
        lf_checker_rt::global::<u32>(EPOCH_B).write(1u32.wrapping_sub(w));
        ret
    }
});
