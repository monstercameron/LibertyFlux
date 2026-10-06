// original: 0x0068EC90 anim_fetch_tracks_gated (proposed)
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated};
/// Copy stored per-bone records into an animation's tracks, letting a callback
/// object accept or reject each track first.
///
/// `this` holds two pointers: the track set (`+0x00`) and the record holder
/// (`+0x04`, whose record array pointer is at `+0x00`, 0xe0 bytes per record).
/// A track set has a lookup key (`+0x04`), an enable word (`+0x08`), the track
/// pointer array (`+0x0c`) and a 16-bit track count (`+0x10`). A track has a
/// flags byte (`+0x04`), a kind byte (`+0x05`: 0 = translation, 1 = rotation),
/// an id word (`+0x06`) and four value words at `+0x10..+0x1c`. `owner` carries
/// the record table (`+0x00`) and a ready word (`+0x2c`); `gate` is an object
/// whose virtual slot at `+0x0c` is called per track with (kind, id, weight
/// slot) and answers accept/reject.
///
/// When the set is enabled, has a key, `owner` is ready, and the shared lookup
/// answers with an entry whose block (`+0x0c`) is present, the block lists
/// packed (track index << 16 | record index) words: each named track is
/// offered to the gate, and on accept its record's words are copied to it
/// (kind 0: `+0x20..+0x2c`; kind 1: `+0x40..+0x4f`). Otherwise every track of a
/// known kind is offered to the gate and, on accept, its record is found
/// through the owner's record-id lookup (callee; each site keeps its own
/// out-word across loop iterations, so a later call re-presents the
/// callee's previous answer), accepted only if the record
/// carries the kind's mask bits (0x380 for translation, 0x0e for rotation)
/// before the same copy runs. The gate's weight write is never read back. The
/// kind is re-read after the gate and after the record check. After either
/// copy the track's flag bit 0x10 is cleared. When the lookup supplied an
/// entry, its reference count is dropped under the owner's mutex at the end.
///
/// The gate's weight slot aliases the caller's incoming argument slot on the
/// fast path in the original; here it is a local, so the stack comparison is
/// off for this function. The kind reaches the gate as one byte of a stack
/// word whose upper bytes are frame leftovers, so only its low byte is
/// compared.
///
/// Original: 0x0068EC90 (thiscall, three stack words; only the first, `owner`,
/// and the second, `gate`, are read; returns nothing).
lf_checker_rt::export!(thiscall, rw_0068ec90(this: u32, owner: u32, gate: u32, _a3: u32) -> u32 {
    unsafe {
        const SET_KEY: u32 = 0x04;
        const SET_ENABLED: u32 = 0x08;
        const SET_TRACKS: u32 = 0x0c;
        const SET_COUNT: u32 = 0x10;
        const HOLDER_RECORDS: u32 = 0x00;
        const RECORD_STRIDE: u32 = 0xe0;
        const OWNER_READY: u32 = 0x2c;
        const TRACK_FLAGS: u32 = 0x04;
        const TRACK_KIND: u32 = 0x05;
        const TRACK_ID: u32 = 0x06;
        const TRACK_VALUE: u32 = 0x10;
        const SKIP_BIT: u8 = 0x10;
        const KIND_TRANSLATION: u8 = 0;
        const KIND_ROTATION: u8 = 1;
        const MASK_TRANSLATION: u32 = 0x380;
        const MASK_ROTATION: u8 = 0x0e;
        const TRANS_SRC: u32 = 0x20;
        const ROT_SRC: u32 = 0x40;
        const GATE_SLOT: u32 = 0x0c;
        const ENTRY_REFS: u32 = 0x08;
        const ENTRY_BLOCK: u32 = 0x0c;
        const OWNER_MUTEX: u32 = 0x10;
        const INFINITE: u32 = 0xffff_ffff;
        const IAT_WAIT: u32 = 0x00e7_3188;
        const IAT_RELEASE: u32 = 0x00e7_31b0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        /// Copy the four translation words of a record to a track.
        unsafe fn copy_translation(track: u32, src: u32) {
            unsafe {
                wr32(track + TRACK_VALUE, rd32(src + TRANS_SRC));
                wr32(track + TRACK_VALUE + 4, rd32(src + TRANS_SRC + 4));
                wr32(track + TRACK_VALUE + 8, rd32(src + TRANS_SRC + 8));
                wr32(track + TRACK_VALUE + 12, rd32(src + TRANS_SRC + 12));
            }
        }

        /// Copy the four rotation words of a record to a track.
        unsafe fn copy_rotation(track: u32, src: u32) {
            unsafe {
                wr32(track + TRACK_VALUE, rd32(src + ROT_SRC));
                wr32(track + TRACK_VALUE + 4, rd32(src + ROT_SRC + 4));
                wr32(track + TRACK_VALUE + 8, rd32(src + ROT_SRC + 8));
                wr32(track + TRACK_VALUE + 12, rd32(src + ROT_SRC + 12));
            }
        }

        #[inline(always)]
        unsafe fn clear_skip(track: u32) {
            unsafe { wr8(track + TRACK_FLAGS, rd8(track + TRACK_FLAGS) & !SKIP_BIT) }
        }

        /// Offer a track to the gate; the weight slot starts at 1.0 and the
        /// gate's answer is its low byte.
        unsafe fn ask_gate(gate: u32, track: u32) -> u8 {
            unsafe {
                let mut weight = 1.0f32;
                let hook: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(gate) + GATE_SLOT) as usize);
                hook(
                    gate,
                    rd8(track + TRACK_KIND) as u32,
                    rd16(track + TRACK_ID),
                    core::ptr::addr_of_mut!(weight) as u32,
                ) as u8
            }
        }

        /// Copy the record at `idx` to `track` by its re-read kind, then clear
        /// the flag bit. The kind cannot change between the reads, so the
        /// fallthrough arm never runs on either side.
        unsafe fn copy_by_kind(track: u32, base: u32, idx: u32) {
            unsafe {
                let src = base + idx * RECORD_STRIDE;
                match rd8(track + TRACK_KIND) {
                    KIND_TRANSLATION => {
                        copy_translation(track, src);
                        clear_skip(track);
                    }
                    KIND_ROTATION => {
                        copy_rotation(track, src);
                        clear_skip(track);
                    }
                    _ => {}
                }
            }
        }

        let set = rd32(this);
        // The shared lookup fills these two words: the owning object (holds the
        // mutex handle) and the entry (holds the reference count and block).
        let mut lookup_out = [0u32; 2];
        if rd32(set + SET_ENABLED) != 0 && rd32(owner + OWNER_READY) != 0 {
            let key = rd32(set + SET_KEY);
            if key != 0 {
                lf_checker_rt::callee_stdcall!(
                    1,
                    u32,
                    core::ptr::addr_of_mut!(lookup_out) as u32,
                    set,
                    owner,
                    key
                );
            }
        }
        let lock_owner = lookup_out[0];
        let entry = lookup_out[1];
        let block = if entry != 0 { rd32(entry + ENTRY_BLOCK) } else { 0 };

        if block != 0 {
            let count = rd32(block) as i32;
            if count > 0 {
                let tracks = rd32(set + SET_TRACKS);
                let mut item_addr = block + 4;
                for _ in 0..count {
                    let item = rd32(item_addr);
                    item_addr += 4;
                    let track = rd32(tracks + (item >> 16) * 4);
                    if ask_gate(gate, track) == 0 {
                        continue;
                    }
                    let base = rd32(rd32(this + 4) + HOLDER_RECORDS);
                    copy_by_kind(track, base, item & 0xffff);
                }
            }
        } else {
            // One out-word per record-id lookup site, kept across
            // iterations: the original's kind-1 and kind-0 sites use
            // different frame slots, each still holding the callee's
            // previous answer on a later call.
            let mut slot_k0: u32 = 0;
            let mut slot_k1: u32 = 0;
            let n = rd16(set + SET_COUNT);
            if n > 0 {
                let tracks = rd32(set + SET_TRACKS);
                for i in 0..n {
                    let track = rd32(tracks + i * 4);
                    let kind = rd8(track + TRACK_KIND);
                    if kind != KIND_TRANSLATION && kind != KIND_ROTATION {
                        continue;
                    }
                    if ask_gate(gate, track) == 0 {
                        continue;
                    }
                    // Record-id lookup on the owner: answers a bool and writes
                    // the record index (16 bits used) through its out pointer.
                    // Each site re-presents its own previous answer (see above).
                    let outp: u32 = if kind == KIND_TRANSLATION {
                        core::ptr::addr_of_mut!(slot_k0) as u32
                    } else {
                        core::ptr::addr_of_mut!(slot_k1) as u32
                    };
                    let found = lf_checker_rt::callee_thiscall!(
                        2,
                        u32,
                        owner,
                        rd16(track + TRACK_ID),
                        outp
                    ) as u8;
                    if found == 0 {
                        continue;
                    }
                    let idx = rd32(outp) & 0xffff;
                    let rec = rd32(owner) + idx * RECORD_STRIDE + 4;
                    let accepted = if kind == KIND_TRANSLATION {
                        rd32(rec) & MASK_TRANSLATION != 0
                    } else {
                        rd8(rec) & MASK_ROTATION != 0
                    };
                    if !accepted {
                        continue;
                    }
                    copy_by_kind(track, rd32(rd32(this + 4) + HOLDER_RECORDS), idx);
                }
            }
        }

        // Drop the entry's reference count under the owner's mutex.
        if entry != 0 {
            let mutex = rd32(lock_owner + OWNER_MUTEX);
            if mutex != 0 {
                let wait: extern "stdcall" fn(u32, u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::global::<u32>(IAT_WAIT).read() as usize);
                wait(mutex, INFINITE);
            }
            wr32(entry + ENTRY_REFS, rd32(entry + ENTRY_REFS).wrapping_sub(1));
            let mutex = rd32(lock_owner + OWNER_MUTEX);
            if mutex != 0 {
                let release: extern "stdcall" fn(u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::global::<u32>(IAT_RELEASE).read() as usize);
                release(mutex);
            }
        }
        0
    }
});
