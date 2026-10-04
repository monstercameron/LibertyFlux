// original: 0x0088f0d0 rage::audVoicePcAdpcm::vf1
#[inline(always)]
unsafe fn r32(base: *mut u8, off: u32) -> u32 {
    *(base.add(off as usize) as *const u32)
}

#[inline(always)]
unsafe fn w32(base: *mut u8, off: u32, v: u32) {
    *(base.add(off as usize) as *mut u32) = v;
}

/// Second virtual method of the PC ADPCM audio voice: attach a voice to a
/// playback request.
///
/// Same shape as the software voice's attach, with the voice state kept at
/// wider offsets: forward the request to the base implementation, create the
/// mixer's per-voice state, snapshot the stream format block, then either
/// allocate a fixed 128 KiB mixing buffer (streamed voices) or size a buffer
/// from the stream and resample into it. Finally probe the global effect
/// chain for the voice kind and attach the chain's tail effect if present.
export!(thiscall, rw_0088f0d0(this: *mut u8, req: u32) -> u32 {
    unsafe {
        const MGR_SLOT: u32 = 0x115A448;
        const FX_CHAIN: u32 = 0x115DA4C;
        w32(this, 0x144, 0);
        let _: u32 = callee_thiscall!(1, u32, this as u32, req);
        let mgr: u32 = *global::<u32>(MGR_SLOT);
        let voice: u32 = callee_thiscall!(2, u32, mgr, r32(this, 0x88));
        w32(this, 0x140, voice);
        let _: u32 = callee_cdecl!(3, u32, (this as u32).wrapping_add(0x90), 0, 0x80);
        *this.add(0x14C) = 0;
        w32(this, 0x110, 0);
        w32(this, 0x114, 0);
        w32(this, 0x118, 0);
        w32(this, 0x11C, 0);
        let fmt = *(r32(this, 0x10) as *const u32).add(0x28 / 4);
        w32(this, 0x128, 0);
        w32(this, 0x124, 0);
        w32(this, 0x120, fmt);
        let src = r32(this, 0x14);
        w32(this, 0x12C, *((src.wrapping_add(0xC)) as *const u32));
        w32(this, 0x130, *((src.wrapping_add(0x10)) as *const u32));
        w32(this, 0x134, *((src.wrapping_add(0x20)) as *const u32));
        w32(this, 0x138, *((src.wrapping_add(0x34)) as *const u32));
        let conv = (this as u32).wrapping_add(0x14E);
        let tag = if r32(this, 0x138) != 0 {
            let p = r32(this, 0x134);
            *((conv) as *mut u16) = *((p) as *const u16);
            *(((p).wrapping_add(2)) as *const u8)
        } else {
            *((conv) as *mut u16) = 0;
            0
        };
        *this.add(0x150) = tag;
        if *this.add(0x8C) & 0x10 != 0 {
            let buf: u32 = callee_cdecl!(4, u32, 0x20000, 0x10);
            w32(this, 0x148, buf);
            let _: u32 = callee_cdecl!(3, u32, buf, 0, 0x20000);
            let _: u32 = callee_thiscall!(5, u32, r32(this, 0x140), r32(this, 0x148), 0x20000);
            let _: u32 = callee_thiscall!(6, u32, r32(this, 0x140), 0);
        } else {
            let n = *((src.wrapping_add(0xC)) as *const u32) << 2;
            let buf: u32 = callee_cdecl!(4, u32, n, 0x10);
            w32(this, 0x148, buf);
            let r: u32 = callee_thiscall!(
                9,
                u32,
                conv,
                buf,
                *(src as *const u32),
                0,
                *((src.wrapping_add(0xC)) as *const u32)
            );
            let _: u32 = callee_thiscall!(5, u32, r32(this, 0x140), r32(this, 0x148), r);
            let state = *((src.wrapping_add(0x14)) as *const u32);
            let mut flag: u8 = if state != 0xFFFF_FFFF { 1 } else { 0 };
            flag = flag.wrapping_add(flag);
            flag ^= *this.add(0x8C);
            flag &= 2;
            *this.add(0x8C) ^= flag;
            if *this.add(0x8C) & 2 != 0 {
                let _: u32 = callee_thiscall!(6, u32, r32(this, 0x140), state.wrapping_add(state));
            }
        }
        let fx = relocated(FX_CHAIN);
        let probe = |slot: u32| {
            let found: u32 = callee_thiscall!(7, u32, fx, slot);
            if found != 0 {
                let effect = *((found.wrapping_add(0x20)) as *const u32);
                let _: u32 = callee_thiscall!(8, u32, r32(this, 0x140), effect);
            }
        };
        match *(((r32(this, 4)) as *const u8).add(0x6C)) {
            1 => probe(3),
            2 => probe(4),
            _ => {
                for slot in [5u32, 0, 1, 2] {
                    probe(slot);
                }
            }
        }
        let tail = *((r32(this, 4).wrapping_add(0xC)) as *const u32);
        if tail != 0 {
            let effect = *((tail.wrapping_add(0x20)) as *const u32);
            w32(this, 0x144, effect);
            let r: u32 = callee_thiscall!(8, u32, r32(this, 0x140), effect);
            r
        } else {
            tail
        }
    }
});
