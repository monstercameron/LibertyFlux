// original: 0x00c2dd70 audfire_voice_bind (proposed name)
// Binds one fire-audio voice slot and refreshes its range endpoints.
//
// The slot at `arg0` selects a voice id through a global table; the id must be
// live (or the setup flag must allow a cold bind) before the emitter state is
// initialized from `arg1`..`arg4` and the per-voice codes are resolved through
// three calls. A second lookup then locates the voice record, and the first of
// three flag-selected channel pairs whose endpoint differs from zero refreshes
// the stored range: the endpoint with the larger magnitude wins and the other
// endpoint's bits are kept alongside. Returns nothing meaningful.
export!(thiscall, rw_00c2dd70(
    this_ptr: u32,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn voice_id(arg0: u32, arg1: u32) -> i32 {
            let w = load_i16(arg0 + 0x2e) as i32;
            let g1 = lu((relocated(0x01295cd8) as i32 + w * 4) as u32);
            lu(lu(g1 + 0xcc).wrapping_add(arg1.wrapping_mul(4))) as i32
        }
        let v0 = voice_id(arg0, arg1);
        if v0 == -1 {
            if arg4 & 0x100000 == 0 {
                return 0;
            }
        }
        store_f(this_ptr + 8, f32::from_bits(arg2));
        store_f(this_ptr + 0x0c, f32::from_bits(arg3));
        let flags = arg4 | 0x100;
        store_u(this_ptr, arg1);
        store_u(this_ptr + 0x14, 0);
        store_u(this_ptr + 0x18, 0);
        store_u(this_ptr + 0x1c, 0);
        store_u(this_ptr + 0x20, 0);
        store_u(this_ptr + 0x24, 0);
        store_u(this_ptr + 0x10, flags);
        let outer = lu(arg0 + 0xdc4);
        if outer != 0 {
            let v = voice_id(arg0, arg1);
            if v > -1 {
                let code = callee_cdecl!(1, u32, v as u32);
                let tag = callee_cdecl!(2, u32, code);
                store_u8(this_ptr + 4, tag as u8);
                let v2 = voice_id(arg0, arg1);
                let code2 = callee_cdecl!(1, u32, v2 as u32);
                let tag2 = callee_cdecl!(3, u32, code2);
                store_u8(this_ptr + 5, tag2 as u8);
                if (tag2 as u8) as i8 > 0 {
                    let outer2 = lu(arg0 + 0xdc4);
                    callee_thiscall!(4, u32, outer2, (tag2 as u8) as i8 as i32 as u32);
                }
            }
        }
        if arg4 & 0x100000 != 0 {
            return 0;
        }
        let vt = lu(arg0);
        let slot5: u32 = lu(vt + 0xa0);
        let fetch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot5 as usize);
        let mut picked = fetch(arg0);
        if picked != 0 {
            picked = fetch(arg0);
            let slot6: u32 = lu(lu(picked) + 0xe0);
            let finish: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot6 as usize);
            picked = finish(picked);
        } else {
            picked = lu(arg0 + 0x100);
        }
        if picked == 0 {
            return 0;
        }
        let idx = voice_id(arg0, arg1);
        let picked2 = fetch(arg0);
        let base_holder;
        if picked2 != 0 {
            let again = fetch(arg0);
            let slot6b: u32 = lu(lu(again) + 0xe0);
            let finish2: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot6b as usize);
            base_holder = finish2(again);
        } else {
            base_holder = lu(arg0 + 0x100);
        }
        let base = lu(lu(base_holder + 4));
        let entry = base.wrapping_add((idx as u32).wrapping_mul(0xe0));
        if entry == 0 {
            return 0;
        }
        let mode = lu(entry + 4);
        if mode == 0 {
            return 0;
        }
        let cl = mode as u8;
        // Each channel pair is eligible when its two mode bits are set and at
        // least one endpoint is nonzero (the original tests this with ucomiss
        // plus a lahf parity check, which treats NaN as nonzero, exactly what
        // IEEE inequality does). The first eligible pair wins.
        let mut wrote = false;
        // (mask, tag, hi_off, lo_off) per pair.
        const PAIRS: [(u32, u32, u32, u32); 3] = [
            (0xfffffff2, 0x00040002, 0xc8, 0xb8),
            (0xfffffff0, 0x00040000, 0xc0, 0xb0),
            (0xfffffff1, 0x00040001, 0xc4, 0xb4),
        ];
        const BITS: [(u8, u8); 3] = [(0x40, 0x08), (0x10, 0x02), (0x20, 0x04)];
        let abs_mask = lu(relocated(0x00fe8f80));
        let mut pi = 0;
        while pi < 3 {
            let (b0, b1) = BITS[pi];
            if !wrote && (cl & b0 != 0) && (cl & b1 != 0) {
                let (mask, tag, hi_off, lo_off) = PAIRS[pi];
                let hi = load_f(entry + hi_off);
                let lo = load_f(entry + lo_off);
                if hi != 0.0 || lo != 0.0 {
                    store_u(this_ptr + 0x10, (lu(this_ptr + 0x10) & mask) | tag);
                    let a_hi = f32::from_bits(hi.to_bits() & abs_mask);
                    let a_lo = f32::from_bits(lo.to_bits() & abs_mask);
                    let other_bits = if a_hi > a_lo {
                        store_f(this_ptr + 8, hi);
                        lo.to_bits()
                    } else {
                        store_f(this_ptr + 8, lo);
                        hi.to_bits()
                    };
                    store_u(this_ptr + 0x0c, other_bits);
                    wrote = true;
                }
            }
            pi += 1;
        }
        if load_u8(this_ptr + 4) == 0xff || load_u8(this_ptr + 5) == 0xff {
            store_u(this_ptr + 0x10, lu(this_ptr + 0x10) & 0xfffbffff);
        }
        0
    }
});
