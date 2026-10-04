// original: 0x88c5a0 rage::audVoiceDSound::vf11
// Refill hook for a DirectSound voice: stream pending audio into the locked
// buffer region, track the play cursor against the queued span, forward the
// voice gain, and, for 3D voices, normalize the head-relative offset and
// push it to the 3D buffer. All callee answers are scripted by the checker.

lf_k2_rt::export!(thiscall, rw_0088c5a0(this: *mut u8) -> () {
    unsafe {
        // Byte/word/dword field views of the voice object.
        let rb = |off: usize| -> u8 { *this.add(off) };
        let wb = |off: usize, v: u8| { *this.add(off) = v; };
        let rd = |off: usize| -> u32 { *(this.add(off) as *const u32) };
        let wd = |off: usize, v: u32| { *(this.add(off) as *mut u32) = v; };

        // Play-cursor tracking and buffer refill.
        if rb(0x9c) != 0 {
            let buf = rd(0x90);
            let vt = *(buf as *const u32);
            let get_pos: extern "stdcall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(*((vt as *const u8).add(0x10) as *const u32));
            let mut play = 0u32;
            get_pos(buf, &mut play as *mut u32 as u32, 0);
            let span = rd(0xc0).wrapping_sub(rd(0xa0));
            if play < span {
                let lock: extern "stdcall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(*((vt as *const u8).add(0x2c) as *const u32));
                let mut locked: u32 = 0;
                let mut locked_bytes: u32 = 0;
                lock(buf, rd(0xb4), rd(0xb8),
                     &mut locked as *mut u32 as u32,
                     &mut locked_bytes as *mut u32 as u32, 0, 0, 0);
                let count = rd(0xc4);
                let mut i = 0u32;
                while i < count {
                    let size = rd(0xb8);
                    let src = rd(0xd0).wrapping_add(rd(0xa0));
                    let dst = locked.wrapping_add(size.wrapping_mul(i));
                    core::ptr::copy(src as *const u8, dst as *mut u8, size as usize);
                    i = i.wrapping_add(1);
                }
                let unlock: extern "stdcall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(*((vt as *const u8).add(0x4c) as *const u32));
                unlock(buf, locked, locked_bytes, 0, 0);
                wb(0x9c, 0);
            }
        }

        // Loop-region flag maintenance from the play cursor.
        if rb(0x8c) & 0x10 != 0 {
            let buf = rd(0x90);
            let vt = *(buf as *const u32);
            let get_pos: extern "stdcall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(*((vt as *const u8).add(0x10) as *const u32));
            let mut play = 0u32;
            get_pos(buf, &mut play as *mut u32 as u32, 0);
            let half = if play >= 0x10000 { 1u32 } else { 0u32 };
            if half != rd(0xd8) {
                if rb(0x9d) != 0 {
                    if rd(0xdc) > 0 {
                        lf_k2_rt::callee_thiscall!(5, u32, this as u32, 0);
                        wd(0xdc, rd(0xdc).wrapping_sub(1));
                    } else {
                        let self_vt = *(this as *const u32);
                        let tgt = *((self_vt as *const u8).add(0x14) as *const u32);
                        let f: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(tgt as usize);
                        f(this as u32);
                    }
                } else {
                    lf_k2_rt::callee_thiscall!(5, u32, this as u32, 0);
                }
            }
        }

        // Forward the voice gain.
        let params = rd(4);
        let gain_bits = *(params as *const u32);
        lf_k2_rt::callee_thiscall!(7, u32, this as u32, gain_bits);

        // Positional update for 3D voices.
        let params = rd(4);
        if *((params as *const u8).add(0x6c)) == 3 {
            let m = |off: usize| -> f32 {
                f32::from_bits(*((params as *const u8).add(off) as *const u32))
            };
            let cos45 = f32::from_bits(0x3f34fdf4);
            let neg_cos45 = f32::from_bits(0xbf34fdf4);
            // Rotate the (right, up, front) triple; several lanes are scaled
            // by zero here and only contribute signed zeros downstream.
            let s34 = m(0x28) * cos45;
            let s28 = m(0x28) * 0.0;
            let p10 = m(0x24) * neg_cos45;
            let s3c = m(0x24) * cos45;
            let m1z = m(0x24) * 0.0;
            let m2z = m(0x38) * 0.0;
            let m3c0 = m(0x34) * neg_cos45;
            let m3z = m(0x34) * 0.0;
            let m2c1 = m(0x38) * cos45;
            let m4z = m(0x2c) * 0.0;
            let m2c0 = m(0x38) * neg_cos45;
            let mut acc10 = p10 + s34;
            let s40a = m2z;
            let mut acc3 = s28 + m1z;
            let s10 = acc10;
            acc10 = s3c + s34;
            let acc4 = m2c1 + s10;
            let mut acc7 = acc3 + s40a;
            acc3 = m4z;
            let mut acc5 = m2c0 + acc10;
            acc10 = m3c0 + acc4;
            acc7 = acc7 + m3z;
            let acc6 = m3c0 + acc5;
            acc5 = m(0x2c);
            acc3 = acc3 + acc10;
            acc7 = acc7 + m4z;
            acc5 = acc5 + acc6;
            let s40 = acc3;
            // Squared length of the offset.
            let mut len2 = s40 * s40;
            let t5 = acc5 * acc5;
            len2 = len2 + t5;
            let t7 = acc7 * acc7;
            len2 = len2 + t7;
            // Near-field selector: the tuned floor when audibly far, else 0.
            let floor = f32::from_bits(*lf_k2_rt::global::<u32>(0x17ad148));
            let near0 = f32::from_bits(*lf_k2_rt::global::<u32>(0x110dad8));
            let near1 = f32::from_bits(*lf_k2_rt::global::<u32>(0x110dad4));
            let near2 = f32::from_bits(*lf_k2_rt::global::<u32>(0x110dad0));
            let sel4 = if len2 > near0 { floor } else { 0.0 };
            let sel3 = if len2 > near1 { floor } else { 0.0 };
            let sel0 = if len2 > near2 { floor } else { 0.0 };
            // Normalize the offset by its length.
            let inv = 1.0 / len2.sqrt();
            let n40 = inv * s40;
            let n5 = inv * acc5;
            let n7 = inv * acc7;
            // Lane-blend against the tuned constant, then publish x, y, y.
            let blend_base = lf_k2_rt::relocated(0x110db50);
            let mut cb = [0u32; 4];
            let mut i = 0;
            while i < 4 {
                cb[i] = *((blend_base + (i as u32) * 4) as *const u32);
                i += 1;
            }
            let v0 = n40.to_bits();
            let v1 = n5.to_bits();
            let m0 = sel0.to_bits();
            let m1 = sel3.to_bits();
            let o0 = (v0 & m0) | ((!m0) & cb[0]);
            let o1 = (v1 & m1) | ((!m1) & cb[1]);
            let buf3d = rd(0x94);
            let vt3d = *(buf3d as *const u32);
            let set_pos: extern "stdcall" fn(u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(*((vt3d as *const u8).add(0x4c) as *const u32));
            set_pos(buf3d, o0, o1, o1, 0);
            let _ = (acc4, acc6, n7, sel4);
        }

        // Tail: refresh the voice, then report the buffer status word.
        lf_k2_rt::callee_thiscall!(9, u32, this as u32);
        let buf = rd(0x90);
        let vt = *(buf as *const u32);
        let get_status: extern "stdcall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vt as *const u8).add(0x24) as *const u32));
        get_status(buf, (this as u32).wrapping_add(0x98));
    }
});
