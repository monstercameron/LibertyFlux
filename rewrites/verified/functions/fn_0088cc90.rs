// original: 0x88cc90 audio_buffer_fill_region
// Lock one half of the voice's DirectSound buffer, decode queued samples
// from the ring into it (at most two chunks when the read wraps), silence
// any remainder, and unlock. A failed lock releases the buffer instead.
// Pure integer code; the only float is the decoder's, behind its stub.

lf_k2_rt::export!(thiscall, rw_0088cc90(this: *mut u8, arg: u32) -> () {
    unsafe {
        let rd = |off: usize| -> u32 { *(this.add(off) as *const u32) };
        let wd = |off: usize, v: u32| { *(this.add(off) as *mut u32) = v; };
        // Only the low byte of the argument is ever read.
        let fresh = (arg & 0xff) != 0;
        let (samples, flags) = if fresh {
            wd(0xec, 0);
            (0x8000u32, 2u32)
        } else {
            wd(0xd0, rd(0xd0).wrapping_add(1));
            (0x4000u32, 0u32)
        };

        let buf = rd(0x90);
        let vt = *(buf as *const u32);
        let lock: extern "stdcall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*((vt as *const u8).add(0x2c) as *const u32));
        let lock_off = rd(0xec).wrapping_shl(17).wrapping_shr(1);
        let mut locked: u32 = 0;
        let mut locked_bytes: u32 = 0;
        let hr = lock(buf, lock_off, samples,
                      &mut locked as *mut u32 as u32,
                      &mut locked_bytes as *mut u32 as u32, 0, 0, flags);
        if (hr as i32) < 0 {
            let buf = rd(0x90);
            let vt = *(buf as *const u32);
            let release: extern "stdcall" fn(u32) -> u32 =
                core::mem::transmute(*((vt as *const u8).add(8) as *const u32));
            release(buf);
            return;
        }

        // Decode one ring chunk; returns the samples produced.
        let decode = |slot: u32, want: u32, dst: u32| -> u32 {
            let avail = rd((slot.wrapping_shl(6) as usize).wrapping_add(0x13c));
            let n = if want < avail { want } else { avail };
            if n != 0 {
                let base = rd((slot.wrapping_add(4).wrapping_shl(6)) as usize);
                let at = rd((slot.wrapping_shl(6) as usize).wrapping_add(0x138));
                let src = base.wrapping_add(at);
                let state = (this as u32).wrapping_add(0x9e);
                lf_k2_rt::callee_thiscall!(3, u32, state, dst, src, 0, n);
                let off = (slot.wrapping_shl(6) as usize).wrapping_add(0x138);
                wd(off, rd(off).wrapping_add(n));
                let left = (slot.wrapping_shl(6) as usize).wrapping_add(0x13c);
                wd(left, rd(left).wrapping_sub(n));
                if rd(left) == 0 {
                    let div = rd(0xfc);
                    wd(0xf8, slot.wrapping_add(1).wrapping_rem(div));
                }
            }
            n
        };

        let first = decode(rd(0xf8), samples, locked);
        if first < samples {
            // The read wrapped: decode the next slot and silence the tail.
            let mut produced = first.wrapping_mul(4);
            let second = decode(rd(0xf8), samples.wrapping_sub(first),
                                locked.wrapping_add(produced));
            produced = produced.wrapping_add(second.wrapping_mul(4));
            let total = samples.wrapping_mul(4);
            if produced < total {
                let dst = locked.wrapping_add(produced);
                core::ptr::write_bytes(dst as *mut u8, 0,
                                       total.wrapping_sub(produced) as usize);
                if *this.add(0x9d) == 0 {
                    wd(0xf0, 1);
                    *this.add(0x9d) = 1;
                }
            }
        }

        let buf = rd(0x90);
        let vt = *(buf as *const u32);
        let unlock: extern "stdcall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*((vt as *const u8).add(0x4c) as *const u32));
        unlock(buf, locked, locked_bytes, 0, 0);
        if !fresh {
            wd(0xec, rd(0xec).wrapping_sub(1) & 1);
        }
    }
});
