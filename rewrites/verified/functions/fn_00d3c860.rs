// original: 0x00d3c860 audio_source_init
/// Initialise an audio source object from ten parameters.
///
/// Stores the category id twice, two child handles, a 3-word position, a
/// gain value and two flag bytes; zeroes the state array; asks the type
/// table whether channel data exists; and, when enabled, builds the spatial
/// parameter block through the audio helpers before finalising the embedded
/// object through its own function table. Returns the object pointer.
export!(thiscall, rw_rb44_f1(this: *mut u8, cat: u32, left: u32, pos: u32, right: u32, gain: u32, chan: u32, flag: u32, aux: u32, b0: u32, b1: u32) -> u32 {
    unsafe {
        w32(this, 0x40, cat);
        w32(this, 0x44, cat);
        w32(this, 0x48, left);
        w32(this, 0x4c, right);
        w32(this, 0x50, r32(pos, 0));
        w32(this, 0x54, r32(pos, 4));
        w32(this, 0x58, r32(pos, 8));
        w32(this, 0x60, gain);
        w32(this, 0x64, 0);
        w32(this, 0x68, 0);
        w32(this, 0x80, b0 & 0xff);
        *this.add(0x84) = b1 as u8;
        *this.add(0x85) = 0;
        // Sub-object at +0x108: construct, then install our own table.
        let sub = this.add(0x108) as u32;
        let _: u32 = callee_thiscall!(1, u32, sub);
        w32(this, 0x108, relocated(0xe8decc));
        // Retain the child handles when present.
        if r32(this as u32, 0x48) != 0 {
            let _: u32 = callee_thiscall!(2, u32, r32(this as u32, 0x48), this.add(0x48) as u32);
        }
        if r32(this as u32, 0x4c) != 0 {
            let _: u32 = callee_thiscall!(2, u32, r32(this as u32, 0x4c), this.add(0x4c) as u32);
        }
        // Zero the state array.
        for i in 0..32u32 {
            w32(this, 0x88 + (i * 4) as usize, 0);
        }
        let mut seed = [0u32, 0u32, 0x3f800000u32, 0u32];
        let has_chan: u32 = callee_thiscall!(3, u32, this as u32);
        if has_chan == 1 {
            let c0 = r32(chan, 0);
            let c1 = r32(chan, 4);
            let c2 = r32(chan, 8);
            let c3 = r32(chan, 12);
            w32(this, 0x70, c0);
            w32(this, 0x74, c1);
            w32(this, 0x78, c2);
            w32(this, 0x7c, c3);
            seed = [c0, c1, c2, c3];
        }
        if flag != 0 {
            let work = [0u32; 12];
            let _: u32 = callee_cdecl!(4, u32, work.as_ptr() as u32, this.add(0x50) as u32, seed.as_ptr() as u32, 0);
            let mut gate = 1u32;
            let edi: u32 = callee_cdecl!(5, u32, flag, aux, 1, &mut gate as *mut u32 as u32);
            if (gate & 0xff) != 0 && edi != 0 {
                let e0 = f32::from_bits(r32(edi, 0));
                let e1 = f32::from_bits(r32(edi, 4));
                let e2 = f32::from_bits(r32(edi, 8));
                // Any nonzero component (NaN counts as nonzero).
                if e0 != 0.0 || e1 != 0.0 || e2 != 0.0 {
                    let mut t = [0u32; 16];
                    t[0] = r32(edi, 0x00);
                    t[1] = r32(edi, 0x04);
                    t[2] = r32(edi, 0x08);
                    t[4] = r32(edi, 0x10);
                    t[5] = r32(edi, 0x14);
                    t[6] = r32(edi, 0x18);
                    t[8] = r32(edi, 0x20);
                    t[9] = r32(edi, 0x24);
                    t[10] = r32(edi, 0x28);
                    t[12] = r32(edi, 0x30);
                    t[13] = r32(edi, 0x34);
                    t[14] = r32(edi, 0x38);
                    let _: u32 = callee_thiscall!(6, u32, t.as_ptr() as u32, t.as_ptr() as u32);
                    // Copy the helper's scratch over the second block,
                    // leaving the four pad words alone.
                    let mut u = [0u32; 16];
                    const DST: [usize; 12] = [0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14];
                    for (i, d) in DST.iter().enumerate() {
                        u[*d] = work[i];
                    }
                    let _: u32 = callee_thiscall!(7, u32, u.as_ptr() as u32, t.as_ptr() as u32);
                    let _: u32 = callee_thiscall!(8, u32, this as u32, flag, aux, edi, u.as_ptr() as u32);
                }
            }
        }
        // Finalise through the sub-object's table (slot +4).
        let vt = r32(this as u32, 0x108);
        let target = r32(vt, 4);
        let fin: extern "thiscall" fn(u32) = core::mem::transmute(target as usize);
        fin(sub);
        this as u32
    }
});
