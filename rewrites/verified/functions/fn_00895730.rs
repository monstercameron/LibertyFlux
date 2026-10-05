// original: 0x00895730 audio_voice_update
//! Update one audio voice: refresh its slot assignment, rebuild its parameter
//! block when the slot is newly acquired, then hand it to the renderer.
//!
//! Returns 0 when the voice is released or has nothing to render, otherwise
//! the renderer's result. (One early path returns entry-EAX with the low byte
//! cleared, which safe Rust cannot observe; the contract therefore compares
//! only AL, which is 0 on every path but the render path.)
// Voice-table globals (file VAs; relocated at run time).
const VOICE_TABLE: u32 = 0x115d988; // base of the voice parameter table
const VOICE_STRIDE: u32 = 0x115d968; // bank-index stride multiplier
const VOICE_DIVISOR: u32 = 0x115d964; // index divisor (never zero in practice)
const AUDIO_MGR: u32 = 0x115d8a0; // audio manager singleton (`this` for engine calls)

// Callee ids (see the fn_00895730 contract for conventions and scripts).
const C_RELEASE: u32 = 1; // release voice slot (manager, voice, slot)
const C_LOOKUP_A: u32 = 2; // voice lookup, first site (this)
const C_NOTIFY: u32 = 3; // index-change notification (manager, b, a, same)
const C_ACQUIRE: u32 = 4; // acquire voice slot (manager, voice) -> slot or 0xff
const C_BUILD: u32 = 5; // build param block into caller buffer (buf)
const C_MEASURE: u32 = 6; // voice measurement (this, 0)
const C_GATE: u32 = 7; // gate test (this) -> nonzero keeps slot
const C_MODE: u32 = 8; // mode query (this)
const C_COPY_BLOCK: u32 = 9; // param-block copy, fn_00895590 (dst, src)
const C_LOOKUP_B: u32 = 10; // voice lookup, second site (this)
const C_RENOTIFY: u32 = 11; // slot-change notification (manager, voice, handle)
const C_RENDER: u32 = 12; // voice render, fn_00895a00 (this, arg)

/// Frame scratch mirror: the original keeps its temporaries in one aligned
/// frame region; several slots overlay (a dword written then patched by byte),
/// so the mirror is byte-exact.
struct Frame {
    raw: [u8; Frame::LEN],
}
impl Frame {
    const BASE: usize = 0x15;
    const LEN: usize = 0x8f - 0x15;
    fn new() -> Frame {
        Frame { raw: [0; Frame::LEN] }
    }
    #[inline]
    fn r8(&self, off: usize) -> u8 {
        self.raw[off - Frame::BASE]
    }
    #[inline]
    fn w8(&mut self, off: usize, v: u8) {
        self.raw[off - Frame::BASE] = v;
    }
    #[inline]
    fn r32(&self, off: usize) -> u32 {
        let i = off - Frame::BASE;
        u32::from_le_bytes([self.raw[i], self.raw[i + 1], self.raw[i + 2], self.raw[i + 3]])
    }
    #[inline]
    fn w32(&mut self, off: usize, v: u32) {
        let i = off - Frame::BASE;
        let b = v.to_le_bytes();
        self.raw[i..i + 4].copy_from_slice(&b);
    }
    #[inline]
    fn w16(&mut self, off: usize, v: u16) {
        let i = off - Frame::BASE;
        let b = v.to_le_bytes();
        self.raw[i..i + 2].copy_from_slice(&b);
    }
    #[inline]
    fn ptr(&mut self, off: usize) -> u32 {
        unsafe { self.raw.as_mut_ptr().add(off - Frame::BASE) as u32 }
    }
}

export!(thiscall, rw_00895730(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        let t = this as u32;
        let r8 = |o: u32| -> u8 { *(t as *const u8).add(o as usize) };
        let r16 = |o: u32| -> u16 { *(t as *const u16).add((o / 2) as usize) };
        let r32 = |o: u32| -> u32 { *(t as *const u32).add((o / 4) as usize) };
        let w8 = |o: u32, v: u8| { *(t as *mut u8).add(o as usize) = v; };
        let g = |va: u32| -> u32 { *global::<u32>(va) };
        let mgr = relocated(AUDIO_MGR);
        let table = g(VOICE_TABLE);
        let voice = r8(0x40) as u32;

        let mut fr = Frame::new();
        fr.w8(0x17, 0);
        fr.w8(0x15, 0xff);
        let flags = r8(0xee);

        // Fast path: resolve the voice's current table row.
        let mut row_ok = false;
        if flags & 0x10 != 0 {
            let tab = table.wrapping_add(voice.wrapping_mul(0x6f40));
            let idx = (r8(0xec) as u32).wrapping_mul(g(VOICE_STRIDE));
            fr.w32(0x18, tab);
            let row = *((tab.wrapping_add(0x6f14)) as *const u32);
            let sel = *((row.wrapping_add(idx).wrapping_add(0xe6)) as *const u8);
            fr.w8(0x15, sel);
            fr.w8(0x1c, sel);
            if sel != 0xff && r8(0xed) & 0xc0 != 0x40 {
                row_ok = true;
                if flags & 8 != 0 {
                    // Detached voice: release its slot and stop.
                    let slot = r8(0xe8);
                    if slot != 0xff {
                        // Push order is (slot, voice).
                        callee_thiscall!(C_RELEASE, u32, mgr, voice, slot as u32);
                        w8(0xe8, 0xff);
                    }
                    return 0;
                }
                // Attached voice: derive both index bytes and notify.
                let saved = fr.r32(0x18);
                let q = (t.wrapping_sub(*((saved.wrapping_add(0x6f10)) as *const u32)))
                    / g(VOICE_DIVISOR);
                fr.w8(0x18, q as u8);
                let handle = callee_thiscall!(C_LOOKUP_A, u32, t);
                let cur = if handle != 0 {
                    let base2 = table.wrapping_add(voice.wrapping_mul(0x6f40));
                    let d = handle
                        .wrapping_sub(*((base2.wrapping_add(0x6f10)) as *const u32))
                        / g(VOICE_DIVISOR);
                    d as u8
                } else {
                    0xff
                };
                let same = (cur == fr.r8(0x15)) as u32;
                // Push order is (same, a, b), so C order is (b, a, same).
                callee_thiscall!(C_NOTIFY, u32, mgr, fr.r32(0x1c), fr.r32(0x18), same);
            }
        }
        if !row_ok && r32(0xd4) == 0 {
            return 0;
        }

        // Main path: make sure the voice owns a slot, building its parameter
        // block when the slot is newly acquired.
        if r8(0xe8) == 0xff {
            let slot = callee_thiscall!(C_ACQUIRE, u32, mgr, voice) as u8;
            w8(0xe8, slot);
            if slot != 0xff {
                callee_thiscall!(C_BUILD, u32, fr.ptr(0x20));
                fr.w32(0x28, r32(0xd4));
                // After `(an instruction of the original)).
                fr.w16(0x34, r16(0xe6));
                let m = callee_thiscall!(C_MEASURE, u32, t, 0);
                fr.w32(0x24, m);
                fr.w8(0x8c, r8(0xed) >> 6);
                fr.w32(0x2c, r32(0xcc));
                if r8(0xee) & 0x20 != 0 {
                    fr.w8(0x16, 1);
                } else {
                    let kept = callee_thiscall!(C_GATE, u32, t) as u8;
                    fr.w8(0x16, (kept != 0) as u8);
                }
                let mode = callee_thiscall!(C_MODE, u32, t) as u8;
                let mut tag = (mode & 1).wrapping_add(mode & 1);
                tag |= fr.r8(0x16) & 1;
                let cfg = fr.r8(0x38);
                fr.w8(0x39, fr.r8(0x39) | 1);
                tag = (tag << 2) | (cfg & 0xf3);
                fr.w8(0x38, tag);
                let slot2 = r8(0xe8) as u32;
                let voice2 = r8(0x40) as u32;
                fr.w8(0x8e, voice2 as u8);
                let present = *((table
                    .wrapping_add(voice2.wrapping_mul(0x6f40))
                    .wrapping_add(slot2)) as *const u8)
                    & 1;
                let dst = if present != 0 {
                    table
                        .wrapping_add(voice2.wrapping_mul(0x6f40))
                        .wrapping_add(slot2.wrapping_mul(0x70))
                        .wrapping_add(0xc0)
                } else {
                    0
                };
                callee_thiscall!(C_COPY_BLOCK, u32, dst, fr.ptr(0x20));
                let tag_addr = table
                    .wrapping_add(
                        (voice2.wrapping_mul(0x37a).wrapping_add(slot2)) << 5,
                    )
                    .wrapping_add(0x54c8);
                let mut stamp = r32(0x54) ^ *((tag_addr) as *const u32);
                stamp &= 0x1fff_ffff;
                *((tag_addr) as *mut u32) ^= stamp;
            }
        }

        // End check: a stamped slot renders, an unstamped one is released.
        let slot3 = r8(0xe8);
        if slot3 == 0xff {
            return fr.r8(0x17) as u32;
        }
        let tag_addr2 = table.wrapping_add(
            ((r8(0x40) as u32)
                .wrapping_mul(0x37a)
                .wrapping_add(slot3 as u32))
                << 5,
        );
        let tag_word: u32 = *((tag_addr2.wrapping_add(0x54c8)) as *const u32);
        let tag_val = ((tag_word << 3) as i32) >> 3;
        if tag_val != -1 {
            return callee_thiscall!(C_RENDER, u32, t, arg);
        }
        if fr.r8(0x15) != 0xff {
            let handle = callee_thiscall!(C_LOOKUP_B, u32, t);
            // Push order is (handle, voice).
            let cur = callee_thiscall!(C_RENOTIFY, u32, mgr, voice, handle) as u8;
            if cur != fr.r8(0x15) {
                return callee_thiscall!(C_RENDER, u32, t, arg);
            }
        }
        callee_thiscall!(C_RELEASE, u32, mgr, r8(0x40) as u32, slot3 as u32);
        w8(0xe8, 0xff);
        0
    }
});
