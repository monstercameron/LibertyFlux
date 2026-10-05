// original: 0x008970f0 rage::audEnvironmentSound::vf7
/// Apply a config block to an environment sound (thiscall, 3 stack args).
///
/// Gate call first (stdcall/3, stubbed as id 1): a zero low byte returns the
/// gate answer unchanged. Otherwise run two refresh steps (thiscall/0, ids 2
/// and 3), copy config words into the object, run the range setup
/// (thiscall/2 with 1000.0f twice, id 4), merge config bits into the mode and
/// flag bytes, resolve the cached table slot through a lazily initialised
/// shared lookup (cdecl/2 id 5, thiscall/1 id 6, globals at file VAs
/// 0x115F7FC/0x115F7F8/0x115F810; the slot index is a signed division by 100
/// by magic multiply), notify through the resolved entry (thiscall/0, id 7),
/// and finish the flag merges. Returns (config[0xC] & ~0xFF) | 1, matching
/// the original's (an instruction of the original).
export!(thiscall, rw_008970f0(this: *mut u8, _a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        let obj = this;
        let cfg = a3 as *const u8;
        // Gate call; a zero low byte means nothing to do (full eax returned).
        let r1 = callee_stdcall!(1, u32, relocated(0x010303C0), a2, a3);
        if r1 & 0xFF == 0 {
            return r1;
        }
        let flags = obj.add(0xEE);
        let r2 = callee_thiscall!(2, u32, obj as u32);
        *flags ^= (*flags ^ r2 as u8) & 3;
        *(obj.add(0xD0) as *mut u32) = *(cfg.add(8) as *const u32);
        *(obj.add(0xCC) as *mut u32) = *(cfg.add(0x10) as *const u32);
        let r3 = callee_thiscall!(3, u32, obj as u32);
        *obj.add(0xEA) = r3 as u8;
        const ONE_K: u32 = 0x447A0000; // 1000.0f, passed twice by bits
        callee_thiscall!(4, u32, obj.add(0xB0) as u32, ONE_K, ONE_K);
        // Mode byte: keep low 6 bits, take top 2 from the config; force both
        // top bits when neither is set.
        let mode = obj.add(0xED);
        let mut m = (*mode & 0x3F) | (*cfg.add(0x15) << 6);
        if m & 0xC0 == 0 {
            m |= 0xC0;
        }
        *mode = m;
        // Cached table slot; 0xFFFF means unassigned.
        let slot = obj.add(0x3E) as *mut u16;
        if *slot == 0xFFFF {
            let mut ptr = *(cfg as *const u32);
            if ptr == 0 {
                // Lazily created shared lookup, guarded by a flag word.
                let init_flag = global::<u32>(0x0115F7FC);
                let fv = *init_flag;
                if fv & 1 == 0 {
                    *init_flag = fv | 1;
                    ptr = callee_cdecl!(5, u32, relocated(0x00E78E60), 0);
                    *global::<u32>(0x0115F7F8) = ptr;
                } else {
                    ptr = *global::<u32>(0x0115F7F8);
                }
            }
            let q = callee_thiscall!(6, u32, relocated(0x0115D9A0), ptr);
            if q == 0 {
                *slot = 0xFFFF;
            } else {
                // Signed division by 100 by magic multiply.
                let base = *global::<u32>(0x0115F810);
                let d = q.wrapping_sub(base) as i32;
                let hi = ((d as i64).wrapping_mul(0x92492493u32 as i32 as i64) >> 32) as i32;
                let mut e = hi.wrapping_add(d);
                e >>= 6;
                let eax = ((e as u32) >> 31).wrapping_add(e as u32);
                *slot = (eax & 0xFFFF) as u16;
            }
        }
        // Resolve the slot to a table entry (null when unassigned) and notify.
        let ax = *slot;
        let entry = if ax == 0xFFFF {
            0
        } else {
            let base = *global::<u32>(0x0115F810);
            (ax as i16 as i32).wrapping_mul(0x70).wrapping_add(base as i32) as u32
        };
        let r7 = callee_thiscall!(7, u32, entry);
        *flags ^= ((r7 as u8) << 6 ^ *flags) & 0x40;
        *obj.add(0xE9) = *cfg.add(0x14);
        let w = *(cfg.add(0xC) as *const u32);
        *flags &= 0xF7;
        *(obj.add(0xD8) as *mut u32) = w;
        // Merge config bits 2-3 into flag bits 4-5.
        let c2 = *cfg.add(0x15) >> 2;
        let f0 = *flags;
        let cl = ((c2 ^ f0) & 0x10) ^ f0;
        *flags = cl;
        *flags = ((c2 ^ cl) & 0x20) ^ cl;
        // (an instruction of the original).
        (w & 0xFFFFFF00) | 1
    }
});
