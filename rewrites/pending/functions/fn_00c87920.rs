// original: 0x00c87920 audio_bank_refresh
// Refresh one audio emitter bank.
//
// `this` points at the bank object. When the low byte of `a1` is set the
// bank's live set (a u16 chain starting at `this+2` through 96-byte slots
// rooted at `this-0x30`) is walked and every slot whose voice pointer is
// missing or idle is handed to the voice starter. Otherwise the bank first
// runs one enumeration pass over the global mixer (collecting voices that
// match the bank's position/falloff request), then walks the same live set
// and starts only slots still tagged with the bank's own generation
// (`this+0xf6c`). Always returns 1. `a0` is unused.
export!(thiscall, rw_rb09_f1(this: u32, _a0: u32, a1: u32) -> u32 {
    /// Slot address for chain index `idx`: slots are 96 bytes, rooted 0x30
    /// below the bank object.
    #[inline(always)]
    fn slot(this: u32, idx: u16) -> u32 {
        this.wrapping_sub(0x30)
            .wrapping_add((idx as u32).wrapping_mul(3).wrapping_shl(5))
    }
    /// True when the slot's voice needs (re)starting: no voice pointer, or
    /// the voice's state word says idle.
    #[inline(always)]
    fn needs_start(el: u32) -> bool {
        let voice = unsafe { *((el + 0x44) as *const u32) };
        voice == 0 || unsafe { *((voice + 0x38) as *const u32) } == 0
    }

    let generation = unsafe { *((this + 0xf6c) as *const u16) };
    if (a1 & 0xff) != 0 {
        // Fast path: start every unvoiced slot in the live chain.
        if unsafe { *((this + 2) as *const u16) } != 0 {
            let mut idx = unsafe { *((this + 2) as *const u16) };
            loop {
                let el = slot(this, idx);
                idx = unsafe { *((el + 0x58) as *const u16) };
                if needs_start(el) {
                    callee_thiscall!(1, u32, el);
                }
                if idx == 0 {
                    break;
                }
            }
        }
    } else {
        // Full path: enumerate the mixer, then start tagged slots.
        let mixer = unsafe { *global::<u32>(0x012b9c78) };
        callee_thiscall!(2, u32, relocated(0x01b4a8b0));
        let mut req = [0u32; 26];
        req[0x14] = 0xffff_ffff;
        req[0x15] = 0xffff_ffff;
        req[0x16] = 0;
        req[0x17] = 0xffff_ffff;
        req[0x18] = 7;
        callee_thiscall!(3, u32, req.as_ptr() as u32);
        let falloff = unsafe { *((this + 0x1c) as *const u32) };
        // NB: the stores below run after `push arg`, so esp-relative slots
        // sit 4 bytes lower than their face value against the struct base.
        req[0] = unsafe { *((this + 0xf40) as *const u32) };
        req[1] = unsafe { *((this + 0xf44) as *const u32) };
        req[2] = unsafe { *((this + 0xf48) as *const u32) };
        req[4] = falloff;
        req[5] = falloff;
        req[6] = falloff;
        req[0x14] = 0;
        req[0x19] = 4;
        let mut id = callee_thiscall!(4, u32, mixer, req.as_ptr() as u32) as u16;
        if id != 0xffff {
            loop {
                let table = unsafe { *((mixer + 0x70) as *const u32) };
                let entry = unsafe {
                    *((table + (id as u32) * 8 + 4) as *const u32)
                } & 0xffff_fff0;
                if entry != 0 {
                    let inst = unsafe { *((entry + 4) as *const u32) };
                    if inst != 0 {
                        let key = unsafe { *((inst + 0xc) as *const u32) };
                        callee_thiscall!(5, u32, this, entry, key, generation as u32);
                    }
                }
                id = callee_thiscall!(6, u32, mixer, req.as_ptr() as u32) as u16;
                if id == 0xffff {
                    break;
                }
            }
        }
        if unsafe { *((this + 2) as *const u16) } != 0 {
            let mut idx = unsafe { *((this + 2) as *const u16) };
            loop {
                let el = slot(this, idx);
                idx = unsafe { *((el + 0x58) as *const u16) };
                if unsafe { *((el + 0x56) as *const u16) } != generation {
                    callee_thiscall!(1, u32, el);
                } else if needs_start(el) {
                    callee_thiscall!(1, u32, el);
                }
                if idx == 0 {
                    break;
                }
            }
        }
        callee_thiscall!(7, u32, relocated(0x01b4a8b0));
    }
    1
});
