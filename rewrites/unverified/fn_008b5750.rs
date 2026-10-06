// original: 0x008B5750 FRONTEND_MENU_HIGHLIGHT

/// Poll two frontend menu highlight inputs and play the highlight sound on a
/// rising edge.
///
/// The function takes no arguments and returns nothing (both callers ignore
/// `eax`). It runs the same two-step sequence twice, once per highlight
/// channel, each channel owning one flag byte and sharing one counter dword:
///
/// * Channel A uses menu index `0x0F` with fallback index `2`, flag byte
///   `FLAG_A` and sound `SOUND_A`. Channel B uses menu index `0x10` with
///   fallback index `3`, flag byte `FLAG_B` and sound `SOUND_B`.
/// * Each channel first asks the menu-toggle callee about its menu index. A
///   zero low byte means "not pressed": the pad object (from the pad callee
///   with argument 1) is then checked at byte `PAD_OFFSET`, and only if that
///   byte is non-zero is the fallback index asked. Any zero along that chain
///   clears the flag and ends the channel.
/// * A non-zero answer means "pressed". If the flag is already set nothing
///   further happens (no repeated sound). Otherwise the shared counter is
///   stepped (channel A: `0 -> 2`, else decrement; channel B: `2 -> 0`, else
///   increment, all wrapping), the flag is set, and the sound callee is
///   called with the audio object and the channel's sound.
///
/// The toggle answers are 8-bit statuses: only the low byte is tested, so an
/// answer such as `0x100` behaves as zero. The counter comparisons are pure
/// equality against `0` (channel A) and `2` (channel B); no signed ordering
/// is involved. All arithmetic wraps.
///
/// Original: 0x008B5750 (cdecl, no stack words, `eax` ignored by callers).
lf_checker_rt::export!(cdecl, rw_008B5750() -> u32 {
    unsafe {
        const FLAG_A: u32 = 0x0103_0BCC;
        const FLAG_B: u32 = 0x0103_0BCD;
        const COUNTER: u32 = 0x0116_09E4;
        const AUDIO_OBJ: u32 = 0x0117_6888;
        const SOUND_A: u32 = 0x00E7_DFE0;
        const SOUND_B: u32 = 0x00E7_DFF8;
        const MENU_A: u32 = 0x0F;
        const MENU_A_FALLBACK: u32 = 0x02;
        const MENU_B: u32 = 0x10;
        const MENU_B_FALLBACK: u32 = 0x03;
        const PAD_OFFSET: u32 = 0x328D;

        #[inline(always)]
        unsafe fn toggle(id: u32, index: u32) -> bool {
            unsafe {
                let ans: u32 = match id {
                    1 => lf_checker_rt::callee_cdecl!(1, u32, index, 0, 0, 0, 0, 0, 0),
                    3 => lf_checker_rt::callee_cdecl!(3, u32, index, 0, 0, 0, 0, 0, 0),
                    5 => lf_checker_rt::callee_cdecl!(5, u32, index, 0, 0, 0, 0, 0, 0),
                    _ => lf_checker_rt::callee_cdecl!(7, u32, index, 0, 0, 0, 0, 0, 0),
                };
                // 8-bit status: only the low byte is tested.
                (ans & 0xFF) != 0
            }
        }

        #[inline(always)]
        unsafe fn pad_pressed(id: u32) -> bool {
            unsafe {
                let pad: u32 = match id {
                    2 => lf_checker_rt::callee_cdecl!(2, u32, 1),
                    _ => lf_checker_rt::callee_cdecl!(6, u32, 1),
                };
                ((pad.wrapping_add(PAD_OFFSET)) as *const u8).read() != 0
            }
        }

        // Channel A.
        let pressed_a = toggle(1, MENU_A) || (pad_pressed(2) && toggle(3, MENU_A_FALLBACK));
        if !pressed_a {
            lf_checker_rt::global::<u8>(FLAG_A).write(0);
        } else if lf_checker_rt::global::<u8>(FLAG_A).read() == 0 {
            let n = lf_checker_rt::global::<u32>(COUNTER).read();
            lf_checker_rt::global::<u32>(COUNTER).write(if n == 0 { 2 } else { n.wrapping_sub(1) });
            lf_checker_rt::global::<u8>(FLAG_A).write(1);
            lf_checker_rt::callee_thiscall!(
                4,
                u32,
                lf_checker_rt::relocated(AUDIO_OBJ),
                lf_checker_rt::relocated(SOUND_A)
            );
        }

        // Channel B.
        let pressed_b = toggle(5, MENU_B) || (pad_pressed(6) && toggle(7, MENU_B_FALLBACK));
        if !pressed_b {
            lf_checker_rt::global::<u8>(FLAG_B).write(0);
        } else if lf_checker_rt::global::<u8>(FLAG_B).read() == 0 {
            let n = lf_checker_rt::global::<u32>(COUNTER).read();
            lf_checker_rt::global::<u32>(COUNTER).write(if n == 2 { 0 } else { n.wrapping_add(1) });
            lf_checker_rt::global::<u8>(FLAG_B).write(1);
            lf_checker_rt::callee_thiscall!(
                8,
                u32,
                lf_checker_rt::relocated(AUDIO_OBJ),
                lf_checker_rt::relocated(SOUND_B)
            );
        }
        0
    }
});
