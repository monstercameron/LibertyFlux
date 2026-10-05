// original: 0x008A97E0 aud_acquire_slot_sound (proposed)

/// Acquire a playing-sound slot for a request: pop a free entry, resolve
/// the sound, and either attach to its existing entry or configure and
/// link a new one.
///
/// `this` is the audio manager, `a0` a request descriptor, `a1`/`a2`/`a3`
/// opaque words passed to helpers. Two lock tokens guard the work: token
/// 1 (callee id 1 takes it with `this+0x3210`, callee id 2 releases it)
/// spans the whole call, token 2 a short free-list update. A null `a0`,
/// a type word of 3 at `a0+6`, an empty free list (`this+0x320C` holding
/// 0xFFFF), a null first answer from helper 3 (callee id 3, thiscall on
/// `a0`, called twice), or a null sound from helper 4 (callee id 4,
/// thiscall on `a0` with `a1`) all release token 1 and return 0; the
/// popped slot is handed back through helper 5 (callee id 5, thiscall on
/// `this` with the slot) on the sound-null path.
///
/// Otherwise the kind word at answer+4 selects a slot head at
/// `this+0xFA4+kind*8`, and a chain of u16-linked entries in `this`
/// (next at `+0`, bank bytes at `+2`/`+3`) is walked for one whose
/// pointer `table1[byte2] + STRIDE1 * byte3` equals the sound
/// (`table1` = bank offset `BANK_ENTRY1` = 0x6F10 in the table at global
/// `AUD_TABLE` = 0x115D988, banks of `BANK_STRIDE` = 0x6F40, stride from
/// global `AUD_STRIDE1` = 0x115D964). A match hands the popped slot back
/// through helper 5, releases token 1 and returns the sound.
///
/// With no entry, helper 6 (callee id 6, thiscall on the global manager
/// at file address 0x115D8A0 with the sound's byte at `+0x40`) runs
/// first. A marker byte other than 0xFF at sound `+5` runs helper 7
/// (callee id 7, thiscall on the sound; a zero low byte also runs helper
/// 8, id 8), and a sub-index of 0xFF at sound `+4` runs helper 9
/// (id 9). A sub-index other than 0xFF then sets bit 0x40 at
/// `table2[byte40] + STRIDE2 * sub + 0xE8` (`table2` = bank offset
/// `BANK_ENTRY2` = 0x6F14, stride from global `AUD_STRIDE2` = 0x115D968).
/// A sub-index of 0xFF instead tears down through helper 13 (id 13,
/// thiscall on the sound with 0), helper 5 and helper 12 (id 12,
/// thiscall on the manager with byte `+0x40`) and returns 0, as does a
/// null second lookup (unreachable without faulting on the first, kept
/// for fidelity).
///
/// The success path stores byte `+0x40` and helper 10's low byte
/// (callee id 10, thiscall on the manager with byte `+0x40` and the
/// sound) into the popped entry, links it at the slot head, runs helper
/// 11 (id 11, thiscall on the sound with `a2`, `a3`) and helper 12,
/// releases token 1 and returns the sound.
///
/// The original reuses its incoming `a1` argument slot as scratch for
/// the slot pointer, so this contract runs with the stack check off.
///
/// Original: 0x008A97E0 (thiscall, `this` in ECX, four stack words,
/// callee pops 16, sound pointer or null in EAX).
lf_checker_rt::export!(thiscall, rw_008A97E0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const AUD_STRIDE1: u32 = 0x115D964;
        const AUD_STRIDE2: u32 = 0x115D968;
        const AUD_TABLE: u32 = 0x115D988;
        const AUD_MGR: u32 = 0x115D8A0;
        const FREE_HEAD: u32 = 0x320C;
        const LOCK_ARG: u32 = 0x3210;
        const SLOT_BASE: u32 = 0xFA4;
        const SLOT_STRIDE: u32 = 8;
        const BANK_STRIDE: u32 = 0x6F40;
        const BANK_ENTRY1: u32 = 0x6F10;
        const BANK_ENTRY2: u32 = 0x6F14;
        const FLAG_OFF: u32 = 0xE8;
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
        unsafe fn bank_entry(idx1: u8, bank_off: u32) -> u32 {
            unsafe {
                let base = lf_checker_rt::global::<u32>(AUD_TABLE).read();
                let bank = (idx1 as u32).wrapping_mul(BANK_STRIDE);
                rd32(base.wrapping_add(bank).wrapping_add(bank_off))
            }
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
        #[inline(always)]
        unsafe fn give_back(this: u32, slot: u32) {
            unsafe {
                let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, this, slot);
            }
        }

        let mut tok1: u32 = 0;
        lock(&mut tok1 as *mut u32 as u32, this.wrapping_add(LOCK_ARG));
        if a0 == 0 {
            unlock(&mut tok1 as *mut u32 as u32);
            return 0;
        }
        if rd16(a0 + 6) == 3 {
            unlock(&mut tok1 as *mut u32 as u32);
            return 0;
        }
        let mut tok2: u32 = 0;
        lock(&mut tok2 as *mut u32 as u32, this.wrapping_add(LOCK_ARG));
        let mut head = rd32(this + FREE_HEAD);
        if head != NONE16 {
            let next = rd16(this.wrapping_add(head.wrapping_mul(4)));
            (this.wrapping_add(FREE_HEAD) as *mut u32).write_unaligned(next as u32);
        }
        unlock(&mut tok2 as *mut u32 as u32);
        if head == NONE16 {
            unlock(&mut tok1 as *mut u32 as u32);
            return 0;
        }
        let r3a: u32 = lf_checker_rt::callee_thiscall!(3, u32, a0);
        if r3a == 0 {
            unlock(&mut tok1 as *mut u32 as u32);
            return 0;
        }
        let r3b: u32 = lf_checker_rt::callee_thiscall!(3, u32, a0);
        let kind = rd16(r3b + 4);
        let snd: u32 = lf_checker_rt::callee_thiscall!(4, u32, a0, a1);
        if snd == 0 {
            give_back(this, head);
            unlock(&mut tok1 as *mut u32 as u32);
            return 0;
        }
        let slot = this
            .wrapping_add(SLOT_BASE)
            .wrapping_add((kind as u32).wrapping_mul(SLOT_STRIDE));
        let first = rd16(slot);
        if first as u32 != NONE16 {
            let stride1 = lf_checker_rt::global::<u32>(AUD_STRIDE1).read();
            let mut idx = first;
            loop {
                let e = this.wrapping_add((idx as u32).wrapping_mul(4));
                let b1 = rd8(e + 2);
                let b2 = rd8(e + 3);
                let p = bank_entry(b1, BANK_ENTRY1)
                    .wrapping_add(stride1.wrapping_mul(b2 as u32));
                if p == snd {
                    give_back(this, head);
                    unlock(&mut tok1 as *mut u32 as u32);
                    return snd;
                }
                idx = rd16(e);
                if idx as u32 == NONE16 {
                    break;
                }
            }
        }
        let mgr = lf_checker_rt::relocated(AUD_MGR);
        let i40 = rd8(snd + 0x40);
        let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, mgr, i40 as u32);
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
            let stride2 = lf_checker_rt::global::<u32>(AUD_STRIDE2).read();
            let flag_base = bank_entry(i40, BANK_ENTRY2)
                .wrapping_add(stride2.wrapping_mul(i4 as u32));
            let fp = flag_base.wrapping_add(FLAG_OFF) as *mut u8;
            fp.write(fp.read() | 0x40);
        }
        if rd8(snd + 4) == NONE8 {
            let _: u32 = lf_checker_rt::callee_thiscall!(13, u32, snd, 0);
            give_back(this, head);
            let _: u32 = lf_checker_rt::callee_thiscall!(12, u32, mgr, i40 as u32);
            unlock(&mut tok1 as *mut u32 as u32);
            return 0;
        }
        {
            let i4 = rd8(snd + 4);
            let stride2 = lf_checker_rt::global::<u32>(AUD_STRIDE2).read();
            let probe = bank_entry(i40, BANK_ENTRY2)
                .wrapping_add(stride2.wrapping_mul(i4 as u32));
            if probe == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(13, u32, snd, 0);
                give_back(this, head);
                let _: u32 = lf_checker_rt::callee_thiscall!(12, u32, mgr, i40 as u32);
                unlock(&mut tok1 as *mut u32 as u32);
                return 0;
            }
        }
        let ent = this.wrapping_add(head.wrapping_mul(4));
        ((ent + 2) as *mut u8).write(i40);
        let r10: u32 = lf_checker_rt::callee_thiscall!(10, u32, mgr, i40 as u32, snd);
        ((ent + 3) as *mut u8).write(r10 as u8);
        let old = rd16(slot);
        (ent as *mut u16).write_unaligned(old);
        (slot as *mut u16).write_unaligned(head as u16);
        let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, snd, a2, a3);
        let _: u32 = lf_checker_rt::callee_thiscall!(12, u32, mgr, i40 as u32);
        unlock(&mut tok1 as *mut u32 as u32);
        snd
    }
});
