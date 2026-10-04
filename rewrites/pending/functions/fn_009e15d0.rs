// original: 0x009e15d0 audio_mode_switch_notify
/// Switch the mode byte; on a change, run the matching
/// attach/detach helper, then (as always afterwards) notify the voice through
/// the matching channel while the link record carries the active flag.
/// Returns the voice word, or the last notify answer. (thiscall/1, byte arg)
export!(thiscall, rw_009e15d0(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        let bl = (arg & 0xFF) as u8;
        if *this.add(0x45) != bl {
            if bl == 1 {
                callee_cdecl!(1, u32, this as u32);
            } else {
                callee_cdecl!(2, u32, this as u32);
            }
            *this.add(0x45) = bl;
            let voice = *((this.add(0x30)) as *const u32);
            if voice != 0 {
                let link = *((this.add(0x34)) as *const u32);
                if link != 0
                    && *((link.wrapping_add(0x24)) as *const u32) & 0x8000000 != 0
                {
                    if bl != 0 {
                        callee_cdecl!(3, u32, voice);
                    } else {
                        callee_cdecl!(4, u32, voice);
                    }
                }
            }
        }
        *this.add(0x45) = bl;
        let voice = *((this.add(0x30)) as *const u32);
        if voice == 0 {
            return 0;
        }
        let link = *((this.add(0x34)) as *const u32);
        if link == 0 {
            return voice;
        }
        if *((link.wrapping_add(0x24)) as *const u32) & 0x8000000 == 0 {
            return voice;
        }
        if bl != 0 {
            callee_cdecl!(3, u32, voice)
        } else {
            callee_cdecl!(4, u32, voice)
        }
    }
});
