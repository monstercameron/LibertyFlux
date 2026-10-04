// original: 0x0099eaf0 audio_voice_attach
/// Attach a voice buffer to the mixer slot, resolving and validating it.
///
/// Behavior: pick the candidate object from `this+0x94`, falling back to
/// `this+0x60`. When the enable flag is set, the candidate is non-null,
/// the slot (`this+0xC`) is empty, and the candidate's tag byte/word match,
/// resolve it through three lookup calls; a null resolution ends the
/// attempt. When the low two bits of the resolved pointer are set, hash
/// the payload pointer and copy it through two more calls, then re-read
/// the handle. When the handle's kind word already equals `0xAC44` the
/// payload at `+0x14` is stored straight into the slot; otherwise, when
/// its flag bit is clear and its version byte equals 10, convert the
/// flags word through another call and, when the converted low word is 1,
/// build a small descriptor, register the payload through two calls, stamp
/// the kind word and tear the descriptor down. Any other shape clears the
/// enable flag. Finally the slot value is written through `arg0`; when the
/// slot stayed empty but a stale handle remains, the handle is released
/// through the release call, the slot cleared, and the release answer
/// returned.
///
/// All heap accesses use unaligned reads/writes: one fabricated handle in
/// the contract sits at an odd address (to set the low pointer bits that
/// select the hash-and-copy branch), and x86 tolerates the unaligned words.
export!(thiscall, rw_0099eaf0(this: u32, arg0: u32) -> u32 {
    unsafe {
        let mut cand = core::ptr::read_unaligned(this.wrapping_add(0x94) as *const u32);
        if cand == 0 {
            cand = core::ptr::read_unaligned(this.wrapping_add(0x60) as *const u32);
        }
        let gate = core::ptr::read_unaligned(global::<u8>(0x01038D15) as u32 as *const u8) != 0
            && cand != 0
            && core::ptr::read_unaligned(this.wrapping_add(0xC) as *const u32) == 0
            && core::ptr::read_unaligned(cand.wrapping_add(0x3B) as *const u8) == 4
            && core::ptr::read_unaligned(cand.wrapping_add(6) as *const u16) == 2;
        if gate {
            let sx = (core::ptr::read_unaligned(cand.wrapping_add(0x3C) as *const u16) as i16) as i32 as u32;
            let s: u32 = callee_cdecl!(1, u32, sx);
            let t: u32 = callee_thiscall!(2, u32, cand);
            let mut h: u32 = callee_cdecl!(3, u32, s, t, 0x485C1FA4);
            if h != 0 {
                let mut esi = core::ptr::read_unaligned(h.wrapping_add(4) as *const u32);
                if (h as u8) & 3 != 0 {
                    esi = esi.wrapping_add(8);
                    let hh: u32 = callee_cdecl!(4, u32, esi);
                    core::ptr::write_unaligned(this.wrapping_add(0x204) as *mut u32, hh);
                    let _: u32 = callee_cdecl!(5, u32, hh, h, esi);
                    h = core::ptr::read_unaligned(this.wrapping_add(0x204) as *const u32);
                }
                if core::ptr::read_unaligned(h.wrapping_add(8) as *const u32) == 0xAC44 {
                    core::ptr::write_unaligned(this.wrapping_add(0xC) as *mut u32, h.wrapping_add(0x14));
                } else if core::ptr::read_unaligned(h.wrapping_add(0x10) as *const u32) & 0x40000000 == 0
                    && core::ptr::read_unaligned(h.wrapping_add(0xC) as *const u8) == 0x0A
                {
                    let mut converted = 0u32;
                    let _: u32 = callee_cdecl!(6, u32, core::ptr::read_unaligned(h.wrapping_add(0x10) as *const u32),
                        &mut converted as *mut u32 as u32);
                    if (converted as u16) == 1 {
                        let target = h.wrapping_add(0x14);
                        let mut zone = [0u32; 8];
                        let _: u32 = callee_thiscall!(7, u32, zone.as_mut_ptr() as u32,
                            &mut converted as *mut u32 as u32, relocated(0x00E90968), 0);
                        core::ptr::write_unaligned(this.wrapping_add(0xC) as *mut u32, target);
                        let _: u32 = callee_cdecl!(8, u32, target, zone.as_mut_ptr() as u32);
                        core::ptr::write_unaligned(h.wrapping_add(8) as *mut u32, 0xAC44);
                        let _: u32 = callee_thiscall!(9, u32, zone.as_mut_ptr() as u32);
                    }
                } else {
                    core::ptr::write_unaligned(global::<u8>(0x01038D15), 0);
                }
            }
        }
        let slot = core::ptr::read_unaligned(this.wrapping_add(0xC) as *const u32);
        core::ptr::write_unaligned(arg0 as *mut u32, slot);
        if slot == 0 {
            let stale = core::ptr::read_unaligned(this.wrapping_add(0x204) as *const u32);
            if stale != 0 {
                let r: u32 = callee_cdecl!(10, u32, stale);
                core::ptr::write_unaligned(this.wrapping_add(0x204) as *mut u32, 0);
                return r;
            }
        }
        slot
    }
});
