// original: 0x0088ee70 audio ring-buffer mix pass (unnamed in symbols)
#[inline(always)]
unsafe fn r8(base: *mut u8, off: u32) -> u8 {
    *base.add(off as usize)
}

#[inline(always)]
unsafe fn r32(base: *mut u8, off: u32) -> u32 {
    *(base.add(off as usize) as *const u32)
}

#[inline(always)]
unsafe fn w32(base: *mut u8, off: u32, v: u32) {
    *(base.add(off as usize) as *mut u32) = v;
}

/// One mix pass over the voice's ring buffer: render up to a quantum of
/// samples from the current slot through the resampler, advance to the next
/// slot when it runs dry, and clear whatever part of the quantum is left
/// over. A nonzero flag word starts a fresh loud pass instead of continuing
/// the quiet one.
export!(thiscall, rw_0088ee70(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        let fresh = (arg & 0xFF) != 0;
        let mut quantum = if fresh {
            w32(this, 0x110, 0);
            0x8000u32
        } else {
            w32(this, 0x124, r32(this, 0x124).wrapping_add(1));
            0x4000u32
        };
        let mut ebx = r32(this, 0x110);
        let stride = r32(this, 0x11C) << 6;
        ebx = (ebx << 17) >> 1;
        let mut take = *((this as u32 + stride + 0xCC) as *const u32);
        ebx = ebx.wrapping_add(r32(this, 0x148));
        if quantum < take {
            take = quantum;
        }
        let mut ret = 0u32;
        if take != 0 {
            let base = (this as u32).wrapping_add(stride);
            let src0 = *((base + 0xC8) as *const u32);
            let src = src0.wrapping_add(*((base + 0x90) as *const u32));
            let _: u32 = callee_thiscall!(
                1,
                u32,
                (this as u32).wrapping_add(0x14E),
                ebx,
                src,
                0,
                take
            );
            let a = r32(this, 0x11C) << 6;
            let c8 = (this as u32 + a + 0xC8) as *mut u32;
            *c8 = (*c8).wrapping_add(take);
            let cc = (this as u32 + a + 0xCC) as *mut u32;
            *cc = (*cc).wrapping_sub(take);
            let slot = r32(this, 0x11C);
            if *((this as u32 + (slot << 6) + 0xCC) as *const u32) == 0 {
                let n = slot.wrapping_add(1);
                let d = r32(this, 0x120);
                w32(this, 0x11C, n % d);
                ret = n / d;
            } else {
                ret = slot << 6;
            }
        }
        if take < quantum {
            let row = (this as u32).wrapping_add(r32(this, 0x11C) << 6);
            let avail = *((row + 0xCC) as *const u32);
            let need = quantum.wrapping_sub(take);
            let take2 = if need < avail { need } else { avail };
            if take2 != 0 {
                let src0 = *((row + 0xC8) as *const u32);
                let src = src0.wrapping_add(*((row + 0x90) as *const u32));
                let out = ebx.wrapping_add(take.wrapping_mul(4));
                let _: u32 = callee_thiscall!(
                    1,
                    u32,
                    (this as u32).wrapping_add(0x14E),
                    out,
                    src,
                    0,
                    take2
                );
                let a = r32(this, 0x11C) << 6;
                let c8 = (this as u32 + a + 0xC8) as *mut u32;
                *c8 = (*c8).wrapping_add(take2);
                let cc = (this as u32 + a + 0xCC) as *mut u32;
                *cc = (*cc).wrapping_sub(take2);
                let slot = r32(this, 0x11C);
                if *((this as u32 + (slot << 6) + 0xCC) as *const u32) == 0 {
                    let n = slot.wrapping_add(1);
                    let d = r32(this, 0x120);
                    w32(this, 0x11C, n % d);
                }
            }
            let filled = take.wrapping_mul(4).wrapping_add(take2.wrapping_mul(4));
            ret = take2;
            quantum <<= 2;
            if filled < quantum {
                quantum = quantum.wrapping_sub(filled);
                let dst = ebx.wrapping_add(filled);
                let cleared: u32 = callee_cdecl!(2, u32, dst, 0, quantum);
                ret = cleared;
                if r8(this, 0x14C) == 0 {
                    w32(this, 0x114, 1);
                    *this.add(0x14C) = 1;
                }
            }
        }
        if !fresh {
            let v = r32(this, 0x110).wrapping_sub(1) & 1;
            w32(this, 0x110, v);
            ret = v;
        }
        ret
    }
});
