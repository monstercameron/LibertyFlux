// original: 0x0088e280 rage::audVoiceSoft::vf1
#[inline(always)]
unsafe fn r32(base: *mut u8, off: u32) -> u32 {
    *(base.add(off as usize) as *const u32)
}

#[inline(always)]
unsafe fn w32(base: *mut u8, off: u32, v: u32) {
    *(base.add(off as usize) as *mut u32) = v;
}

/// Second virtual method of the software audio voice: attach a voice to a
/// playback request.
///
/// Forwards the request word to the base implementation, creates the mixer's
/// per-voice state through the audio manager, clears the voice's cached
/// counters, then either allocates a fixed 128 KiB mixing buffer (streamed
/// voices) or uses the caller's sample block directly. Finally it probes the
/// global effect chain for the voice kind (kinds 1 and 2 probe one slot each,
/// anything else probes slots 5, 0, 1, 2 in order) and attaches the chain's
/// tail effect when one is present.
export!(thiscall, rw_0088e280(this: *mut u8, req: u32) -> u32 {
    unsafe {
        const MGR_SLOT: u32 = 0x115A448;
        const FX_CHAIN: u32 = 0x115DA4C;
        w32(this, 0x134, 0);
        let _: u32 = callee_thiscall!(1, u32, this as u32, req);
        let mgr: u32 = *global::<u32>(MGR_SLOT);
        let voice: u32 = callee_thiscall!(2, u32, mgr, r32(this, 0x88));
        w32(this, 0x130, voice);
        let _: u32 = callee_cdecl!(3, u32, (this as u32).wrapping_add(0x90), 0, 0x80);
        *this.add(0x13C) = 0;
        w32(this, 0x110, 0);
        w32(this, 0x114, 0);
        w32(this, 0x118, 0);
        w32(this, 0x11C, 0);
        let fmt = *(r32(this, 0x10) as *const u32).add(0x28 / 4);
        w32(this, 0x128, 0);
        w32(this, 0x124, 0);
        w32(this, 0x120, fmt);
        if *this.add(0x8C) & 0x10 != 0 {
            let buf: u32 = callee_cdecl!(4, u32, 0x20000, 0x10);
            w32(this, 0x138, buf);
            let _: u32 = callee_cdecl!(3, u32, buf, 0, 0x20000);
            let _: u32 = callee_thiscall!(5, u32, r32(this, 0x130), r32(this, 0x138), 0x20000);
            let _: u32 = callee_thiscall!(6, u32, r32(this, 0x130), 0);
        } else {
            let src = r32(this, 0x14);
            let blk = *(src as *const u32);
            let len = *((src.wrapping_add(0xC)) as *const u32);
            let _: u32 = callee_thiscall!(5, u32, r32(this, 0x130), blk, len);
            let state = *((src.wrapping_add(0x14)) as *const u32);
            let mut flag: u8 = if state != 0xFFFF_FFFF { 1 } else { 0 };
            flag = flag.wrapping_add(flag);
            flag ^= *this.add(0x8C);
            flag &= 2;
            *this.add(0x8C) ^= flag;
            if *this.add(0x8C) & 2 != 0 {
                let _: u32 = callee_thiscall!(6, u32, r32(this, 0x130), state.wrapping_add(state));
            }
        }
        let fx = relocated(FX_CHAIN);
        let probe = |slot: u32| {
            let found: u32 = callee_thiscall!(7, u32, fx, slot);
            if found != 0 {
                let effect = *((found.wrapping_add(0x20)) as *const u32);
                let _: u32 = callee_thiscall!(8, u32, r32(this, 0x130), effect);
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
            w32(this, 0x134, effect);
            let r: u32 = callee_thiscall!(8, u32, r32(this, 0x130), effect);
            r
        } else {
            tail
        }
    }
});
