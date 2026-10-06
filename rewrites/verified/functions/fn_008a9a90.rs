// original: 0x008A9A90 aud_build_and_play_slot (proposed)
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated};
/// Build a play request from a parameter block, resolve it to a sound
/// through the audio manager, and link the sound into a fresh slot.
///
/// `this` is the audio manager. `a2` points at a small descriptor whose
/// word at `+4` is the slot kind; `a4` is the parameter block; `a0`/`a1`
/// are opaque words passed to the resolver; `a3` is an optional
/// out-pointer receiving the sound. Two lock tokens guard the work
/// (callee id 1 takes them with `this+0x3210`, id 2 releases them);
/// each token is four words, and token 1's last two words alias the
/// middle struct's first two (`-1`) words once it is built. A
/// zero global flag at file address 0x115DCE6, a zero byte at
/// `this+0x3231`, an empty free list (`this+0x320C` holding 0xFFFF), or a
/// null answer from the resolver all release token 1 and return 0; the
/// popped slot is handed back through helper 6 (id 6, thiscall on `this`
/// with the slot) on the resolver-null path.
///
/// The request is three structs built in the frame. Header (6 bytes):
/// the resolver-index word, a magic-division index word, a selector byte
/// and a flag byte. The selector is byte `+0x40` of the object at
/// `a4+0x30` when that pointer is set, else byte `a4+0x44`, else helper
/// 4's low byte (id 4, thiscall on the global manager at file address
/// 0x115D8A0, no arguments). The division index is -1 for a null word at
/// `a4+0x1C`, else `(word - G2) * 0x92492493 >> 32`, plus the word,
/// arithmetic-shifted right 6, plus its own top bit (global `G2` at file
/// address 0x115F810; all 32-bit wrapping). The flag byte mixes bits of
/// `a4+0x46` with helper 3's answer (id 3, cdecl, one word: `a4+0x18`)
/// and the low byte of `a4+0x38` (`-1` there forces bits 0-5 on). Middle
/// (24 bytes): two -1 words, two `1.0f` words, `a4+0x10`, a zero byte and
/// a bit-mix of `a4+0x45`/`a4+0x46`. Tail: `a2`, `a3`, words `a4+0xC`,
/// `a4+0x28`, `a4+0x24`, `a4+0x2C`, and two bits derived from `a4+0x46`.
///
/// The resolver (id 5, thiscall on the global at file address 0x115DC18
/// with `a0`, `a1` and the three struct pointers) yields the sound. The
/// success path stores its byte `+0x40` and helper 10's low byte (id 10,
/// thiscall on the manager with byte `+0x40` and the sound) into the
/// popped entry, links the entry at the kind's slot head
/// (`this+0xFA4+kind*8`), runs the marker/sub-index helpers (id 7
/// thiscall on the sound with a zero-low-byte gate for id 8, id 9 for a
/// sub-index of 0xFF at sound `+4`), sets bit 0x40 at
/// `table2[byte40] + STRIDE2 * sub + 0xE8` for any other sub-index
/// (`table2` = bank offset 0x6F14 in the table at global `AUD_TABLE` =
/// 0x115D988, banks of 0x6F40, stride from `AUD_STRIDE2` = 0x115D968),
/// stores the sound through `a3` unless null, releases token 1 and
/// returns the sound.
///
/// Original: 0x008A9A90 (thiscall, `this` in ECX, five stack words,
/// callee pops 20, sound pointer or null in EAX).
lf_checker_rt::export!(thiscall, rw_008A9A90(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const G_FLAG: u32 = 0x115DCE6;
        const AUD_MGR: u32 = 0x115D8A0;
        const AUD_STRIDE2: u32 = 0x115D968;
        const AUD_TABLE: u32 = 0x115D988;
        const AUD_MGR2: u32 = 0x115DC18;
        const G_DIV: u32 = 0x115F810;
        const READY: u32 = 0x3231;
        const FREE_HEAD: u32 = 0x320C;
        const LOCK_ARG: u32 = 0x3210;
        const SLOT_BASE: u32 = 0xFA4;
        const SLOT_STRIDE: u32 = 8;
        const BANK_STRIDE: u32 = 0x6F40;
        const BANK_ENTRY2: u32 = 0x6F14;
        const FLAG_OFF: u32 = 0xE8;
        const DIV_MAGIC: i32 = 0x9249_2493u32 as i32;
        const NONE16: u32 = 0xFFFF;
        const NONE8: u8 = 0xFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn lock(token: u32, arg: u32) {
            unsafe {
                let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, token, arg);
            }
        }
        #[inline(always)]
        unsafe fn unlock(token: u32) {
            unsafe {
                let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, token);
            }
        }

        // Four-word tokens: token 1's last two words alias the middle
        // struct's first two words (see below); token 2 stays zeroed.
        let mut tok1 = [0u32; 4];
        lock(tok1.as_mut_ptr() as u32, this.wrapping_add(LOCK_ARG));
        if rd8(lf_checker_rt::relocated(G_FLAG)) == 0 {
            unlock(tok1.as_mut_ptr() as u32);
            return 0;
        }
        if rd8(this + READY) == 0 {
            unlock(tok1.as_mut_ptr() as u32);
            return 0;
        }
        let mut tok2 = [0u32; 4];
        lock(tok2.as_mut_ptr() as u32, this.wrapping_add(LOCK_ARG));
        let head = rd32(this + FREE_HEAD);
        if head != NONE16 {
            let next = rd16(this.wrapping_add(head.wrapping_mul(4)));
            (this.wrapping_add(FREE_HEAD) as *mut u32).write_unaligned(next as u32);
        }
        unlock(tok2.as_mut_ptr() as u32);
        let kind = rd16(a2 + 4);
        if head == NONE16 {
            unlock(tok1.as_mut_ptr() as u32);
            return 0;
        }
        // Header struct.
        let b46 = rd8(a4 + 0x46);
        let ch1 = (b46 << 2) | 0xBF;
        let sel: u8 = {
            let p30 = rd32(a4 + 0x30);
            if p30 != 0 {
                rd8(p30 + 0x40)
            } else {
                let b44 = rd8(a4 + 0x44);
                if b44 != NONE8 {
                    b44
                } else {
                    let mgr = lf_checker_rt::relocated(AUD_MGR);
                    lf_checker_rt::callee_thiscall!(4, u32, mgr) as u8
                }
            }
        };
        let div_idx: u32 = {
            let w1c = rd32(a4 + 0x1C);
            if w1c == 0 {
                0xFFFF_FFFF
            } else {
                let g = lf_checker_rt::global::<u32>(G_DIV).read();
                let d = (w1c as i32).wrapping_sub(g as i32);
                let prod = (d as i64).wrapping_mul(DIV_MAGIC as i64);
                let mut edx = (prod >> 32) as i32;
                edx = edx.wrapping_add(d);
                edx >>= 6;
                let top = ((edx as u32) >> 31) as i32;
                edx.wrapping_add(top) as u32
            }
        };
        let mut cl = b46 << 2;
        cl ^= ch1;
        cl &= 0x7F;
        cl ^= b46 << 2;
        let r3: u32 = lf_checker_rt::callee_cdecl!(3, u32, rd32(a4 + 0x18));
        let mut hdr = [0u8; 8];
        hdr[0..2].copy_from_slice(&(r3 as u16).to_le_bytes());
        hdr[2..4].copy_from_slice(&(div_idx as u16).to_le_bytes());
        hdr[4] = sel;
        let w38 = rd32(a4 + 0x38);
        if w38 == 0xFFFF_FFFF {
            hdr[5] = cl | 0x3F;
        } else {
            let mut c = cl;
            let mut a = (w38 as u8) ^ c;
            a &= 0x3F;
            c ^= a;
            hdr[5] = c;
        }
        // Middle struct (24 bytes).
        let mut mid = [0u8; 24];
        mid[0..4].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
        mid[4..8].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
        mid[8..12].copy_from_slice(&0x3F80_0000u32.to_le_bytes());
        mid[12..16].copy_from_slice(&0x3F80_0000u32.to_le_bytes());
        mid[16..20].copy_from_slice(&rd32(a4 + 0x10).to_le_bytes());
        mid[20] = 0;
        {
            let mut a = rd8(a4 + 0x45) ^ b46;
            a &= 0x3F;
            a ^= b46;
            mid[21] = a;
        }
        // Token 1's last two words are the middle struct's first two.
        tok1[2] = 0xFFFF_FFFF;
        tok1[3] = 0xFFFF_FFFF;
        // Tail struct.
        let mut tail = [0u8; 28];
        tail[0..4].copy_from_slice(&a2.to_le_bytes());
        tail[4..8].copy_from_slice(&a3.to_le_bytes());
        tail[8..12].copy_from_slice(&rd32(a4 + 0xC).to_le_bytes());
        tail[12..16].copy_from_slice(&rd32(a4 + 0x28).to_le_bytes());
        tail[16..20].copy_from_slice(&rd32(a4 + 0x24).to_le_bytes());
        tail[20..24].copy_from_slice(&rd32(a4 + 0x2C).to_le_bytes());
        tail[24] = (b46 >> 2) & 1;
        tail[25] = (b46 >> 1) & 1;
        let mgr2 = lf_checker_rt::relocated(AUD_MGR2);
        let snd: u32 = lf_checker_rt::callee_thiscall!(
            5,
            u32,
            mgr2,
            a0,
            a1,
            hdr.as_mut_ptr() as u32,
            mid.as_mut_ptr() as u32,
            tail.as_mut_ptr() as u32
        );
        if snd == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, this, head);
            unlock(tok1.as_mut_ptr() as u32);
            return 0;
        }
        let mgr = lf_checker_rt::relocated(AUD_MGR);
        let i40 = rd8(snd + 0x40);
        ((this.wrapping_add(head.wrapping_mul(4))).wrapping_add(2) as *mut u8).write(i40);
        let r10: u32 = lf_checker_rt::callee_thiscall!(10, u32, mgr, i40 as u32, snd);
        ((this.wrapping_add(head.wrapping_mul(4))).wrapping_add(3) as *mut u8)
            .write(r10 as u8);
        let ent = this.wrapping_add(head.wrapping_mul(4));
        let slot = this
            .wrapping_add(SLOT_BASE)
            .wrapping_add((kind as u32).wrapping_mul(SLOT_STRIDE));
        let old = rd16(slot);
        (ent as *mut u16).write_unaligned(old);
        (slot as *mut u16).write_unaligned(head as u16);
        if rd8(snd + 5) != NONE8 {
            let r7: u32 = lf_checker_rt::callee_thiscall!(7, u32, snd);
            if r7 & 0xFF == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, snd);
            }
        }
        if rd8(snd + 4) == NONE8 {
            let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, snd);
        }
        if rd8(snd + 4) != NONE8 {
            let i4 = rd8(snd + 4);
            let base = lf_checker_rt::global::<u32>(AUD_TABLE).read();
            let stride2 = lf_checker_rt::global::<u32>(AUD_STRIDE2).read();
            let bank = (i40 as u32).wrapping_mul(BANK_STRIDE);
            let flag_base = rd32(base.wrapping_add(bank).wrapping_add(BANK_ENTRY2))
                .wrapping_add(stride2.wrapping_mul(i4 as u32));
            let fp = flag_base.wrapping_add(FLAG_OFF) as *mut u8;
            fp.write(fp.read() | 0x40);
        }
        if a3 != 0 {
            (a3 as *mut u32).write_unaligned(snd);
        }
        unlock(tok1.as_mut_ptr() as u32);
        snd
    }
});
