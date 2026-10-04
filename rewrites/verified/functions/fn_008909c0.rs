// original: 0x008909c0 rage::audSound::vf7
/// Attach a voice description to this sound and reconcile its flag bytes.
///
/// Links the sound to a voice record (`a0`), copies the identity word and
/// selector from the descriptor (`a1`), then runs three initialisation
/// helpers over a zeroed scratch struct. If the sound's second selector is
/// clear, it marks the referenced voice row present and, when the option
/// byte requests full setup, resolves the category object through a cached
/// hash lookup (computing the hash once and memoising it in globals) and
/// folds the probe result into the option byte. Otherwise it merges the
/// referenced row's option bits into the sound. It finishes by merging the
/// descriptor's flag byte into the sound and returns 1, or 0 when either
/// input pointer is null or the voice column is unassigned.
export!(thiscall, rw_008909c0(sound: u32, voice: u32, desc: u32, extra: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6f40;
        const NO_VOICE: u8 = 0xff;
        if desc == 0 {
            return 0;
        }
        if voice == 0 {
            return 0;
        }
        *((sound + 0x98) as *mut u32) = voice;
        *((sound + 0x3c) as *mut u32) = *(desc as *const u32);
        *((sound + 0x40) as *mut u16) = *((desc + 4) as *const u16);
        let mut scratch = [0u32; 16];
        let frame = scratch.as_mut_ptr() as u32;
        callee_thiscall!(1, u32, frame);
        callee_thiscall!(2, u32, sound, voice, 0, frame);
        callee_thiscall!(3, u32, sound, extra, frame);
        *((sound + 0x3b) as *mut u8) = *(voice as *const u8);
        let sel2 = *((sound + 5) as *const u8);
        if sel2 == NO_VOICE {
            callee_thiscall!(4, u32, sound);
            let col = *((sound + 4) as *const u8);
            if col == NO_VOICE {
                return 0;
            }
            let row = *((sound + 0x40) as *const u8) as u32;
            let stride = *global::<u32>(0x115d968);
            let table = *global::<u32>(0x115d988);
            let slot = table
                .wrapping_add(row.wrapping_mul(ROW_STRIDE))
                .wrapping_add(0x6f14);
            let rec = (*(slot as *const u32))
                .wrapping_add((col as u32).wrapping_mul(stride));
            *((rec + 0xe7) as *mut u8) |= 0x38;
            if *((sound + 0x41) as *const u8) & 0x3f == 0x3f {
                let mut cat = *(extra as *const u32);
                if cat == 0 {
                    let flag = *global::<u32>(0x115d994);
                    if flag & 1 == 0 {
                        *global::<u32>(0x115d994) = flag | 1;
                        let h: u32 =
                            callee_cdecl!(5, u32, relocated(0xe784f8), 0);
                        *global::<u32>(0x115d990) = h;
                        cat = h;
                    } else {
                        cat = *global::<u32>(0x115d990);
                    }
                }
                let found: u32 =
                    callee_thiscall!(6, u32, relocated(0x115d9a0), cat);
                if found != 0 {
                    let ok: u32 = callee_thiscall!(7, u32, found);
                    if (ok as u8) != 0 {
                        let ob = *((sound + 0x41) as *const u8);
                        *((sound + 0x41) as *mut u8) = (ob & 0xc1) | 1;
                    } else {
                        *((sound + 0x41) as *mut u8) &= 0xc0;
                    }
                }
            }
        } else {
            let row = *((sound + 0x40) as *const u8) as u32;
            let stride = *global::<u32>(0x115d964);
            let table = *global::<u32>(0x115d988);
            let slot = table
                .wrapping_add(row.wrapping_mul(ROW_STRIDE))
                .wrapping_add(0x6f10);
            let rec = (*(slot as *const u32))
                .wrapping_add((sel2 as u32).wrapping_mul(stride));
            let diff =
                (*((rec + 0x41) as *const u8) ^ *((sound + 0x41) as *const u8)) & 0x3f;
            *((sound + 0x41) as *mut u8) ^= diff;
        }
        let df = *((desc + 5) as *const u8);
        let merged = ((df ^ *((sound + 0x39) as *const u8)) & 0x7f) ^ df;
        *((sound + 0x39) as *mut u8) = merged;
        1
    }
});
