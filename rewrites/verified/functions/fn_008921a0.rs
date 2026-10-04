// original: 0x008921a0 audSound_start_stop
/// Start or stop this sound's voice for the given timeline position.
///
/// Polls the voice state, optionally notifies the registered hook, then
/// walks a chain of guards: an early-out when the voice is already gone
/// (marks the sound stopped and returns 0), a limit check against the
/// stored window, and a final hook round that either commits the new
/// window or parks the sound. Returns a small status code in AL (with the
/// helper answers' upper bytes preserved in EAX exactly as the original
/// leaves them).
export!(thiscall, rw_008921a0(sound: u32, pos: u32) -> u32 {
    unsafe {
        let flag38 = |s: u32| *((s + 0x38) as *const u8);
        let flag39 = |s: u32| *((s + 0x39) as *const u8);
        let idx = |s: u32| *((s + 0x3b) as *const u8) as u32;
        let hook2 = |table: u32, s: u32, a: u32| -> u32 {
            let tgt = *global::<u32>(table + idx(s) * 4);
            let f: extern "cdecl" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(s, a)
        };
        callee_thiscall!(1, u32, sound);
        let f = flag38(sound);
        if f & 0x10 != 0 && f & 0x40 == 0 {
            hook2(0x115d7d4, sound, pos);
        }
        if flag39(sound) & 8 != 0 {
            let ans: u32 = callee_thiscall!(2, u32, sound);
            if (ans as u8) == 0 {
                let tgt = *global::<u32>(0x115d5f4 + idx(sound) * 4);
                let f1: extern "cdecl" fn(u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                f1(sound);
                *((sound + 6) as *mut u16) = 3;
                return 0;
            }
        } else if flag38(sound) & 0x20 != 0 {
            let ans: u32 = callee_thiscall!(3, u32, sound, pos);
            if (ans as u8) == 0 {
                let tgt = *global::<u32>(0x115d5f4 + idx(sound) * 4);
                let f1: extern "cdecl" fn(u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                f1(sound);
                *((sound + 6) as *mut u16) = 3;
                return 0;
            }
        }
        let mut defer = false;
        if flag38(sound) & 0x20 == 0 {
            let ans: u32 = callee_thiscall!(4, u32, sound, pos);
            if (ans as u8) == 0 && *((sound + 0x3a) as *const u8) & 1 != 0 {
                defer = true;
            }
        }
        if flag39(sound) & 2 != 0 {
            let ans: u32 = callee_thiscall!(5, u32, sound);
            if (ans as u8) != 0 {
                let key = *((sound + 0x3c) as *const i16) as i32 as u32;
                let mapped: u32 = callee_cdecl!(6, u32, key);
                let tgt = *global::<u32>(0x115d654 + idx(sound) * 4);
                let f3: extern "cdecl" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                let verdict = f3(sound, mapped, 0);
                if verdict != 1 {
                    return (verdict & 0xffffff00) | 1;
                }
                *((sound + 0x39) as *mut u8) &= 0xfd;
                *((sound + 0x84) as *mut u32) = pos;
                if *((sound + 0x50) as *const u32) != 0 {
                    *((sound + 0x42) as *mut u16) = 1;
                    return 1;
                }
                *((sound + 0x42) as *mut u16) = 2;
                let tail: u32 = hook2(0x115d594, sound, pos);
                return (tail & 0xffffff00) | 1;
            }
        }
        if *((sound + 0x42) as *const u16) == 1 {
            let end = (*((sound + 0x84) as *const u32))
                .wrapping_add(*((sound + 0x50) as *const u32));
            if end <= pos {
                *((sound + 0x42) as *mut u16) = 2;
                let tail: u32 = hook2(0x115d594, sound, pos);
                return (tail & 0xffffff00) | 1;
            }
            return (end & 0xffffff00) | 1;
        }
        let probe: u32 = hook2(0x115d534, sound, pos);
        if (probe as u8) == 0 {
            *((sound + 6) as *mut u16) = 3;
            // The original spills AL to its incoming stack slot here and
            // reloads it after the optional call below; the local keeps it.
            return probe;
        }
        if defer {
            let refresh: u32 = callee_thiscall!(7, u32, sound);
            return (refresh & 0xffffff00) | ((probe as u8) as u32);
        }
        probe
    }
});
