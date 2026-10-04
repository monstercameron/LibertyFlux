// original: 0x0068EA60 anim_fetch_tracks_masked (proposed)

/// Copy stored per-bone records into an animation's tracks, keeping only the
/// bones a mask array enables.
///
/// `this` holds two pointers: the track set (`+0x00`) and the record holder
/// (`+0x04`, whose record array pointer is at `+0x00`, 0xe0 bytes per record).
/// A track set has a lookup key (`+0x04`), an enable word (`+0x08`), the track
/// pointer array (`+0x0c`) and a 16-bit track count (`+0x10`). A track has a
/// flags byte (`+0x04`), a kind byte (`+0x05`: 0 = translation, 1 = rotation),
/// an id word (`+0x06`) and four value words at `+0x10..+0x1c`. `owner` carries
/// the record table (`+0x00`) and a ready word (`+0x2c`); `mask` holds one word
/// per track position.
///
/// When the set is enabled, has a key, `owner` is ready, and the shared lookup
/// answers with an entry whose block (`+0x0c`) is present, the block lists
/// packed (track index << 16 | record index) words: for each item the mask
/// word at the track index must have any of the low 31 bits set, and the
/// record's words are copied to the track (kind 0: `+0x20..+0x2c`; kind 1:
/// `+0x40..+0x4f`). Otherwise every track is visited in order with the mask
/// cursor advancing alongside: a passing mask word admits the track to the
/// owner's record-id lookup (callee), whose 16-bit answer indexes a record
/// that must carry the kind's mask bits (0x380 for translation, 0x0e for
/// rotation) before the same copy runs. After either copy the track's flag bit
/// 0x10 is cleared. When the lookup supplied an entry, its reference count is
/// dropped under the owner's mutex at the end.
///
/// The mask word is read before the track pointer in the fast path, and the
/// track pointer is loaded even when the mask fails, matching the original's
/// fault order. The record-id lookup's out word aliases the caller's incoming
/// argument slot in the original; here it is a local, so the stack comparison
/// is off for this function.
///
/// Original: 0x0068EA60 (thiscall, three stack words; only the first, `owner`,
/// and the third, `mask`, are read; returns nothing).
lf_checker_rt::export!(thiscall, rw_0068ea60(this: u32, owner: u32, _a2: u32, mask: u32) -> u32 {
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
        const MASK_BITS: u32 = 0x7fff_ffff;
        const MASK_TRANSLATION: u32 = 0x380;
        const MASK_ROTATION: u8 = 0x0e;
        const TRANS_SRC: u32 = 0x20;
        const ROT_SRC: u32 = 0x40;
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
                    let hi = item >> 16;
                    let maskw = rd32(mask + hi * 4);
                    let track = rd32(tracks + hi * 4);
                    if maskw & MASK_BITS == 0 {
                        continue;
                    }
                    let src = rd32(rd32(this + 4) + HOLDER_RECORDS) + (item & 0xffff) * RECORD_STRIDE;
                    match rd8(track + TRACK_KIND) {
                        KIND_TRANSLATION => copy_translation(track, src),
                        KIND_ROTATION => copy_rotation(track, src),
                        _ => continue,
                    }
                    clear_skip(track);
                }
            }
        } else {
            let n = rd16(set + SET_COUNT);
            if n > 0 {
                let tracks = rd32(set + SET_TRACKS);
                let mut cursor = mask;
                for i in 0..n {
                    let m = rd32(cursor);
                    cursor += 4;
                    let track = rd32(tracks + i * 4);
                    let kind = rd8(track + TRACK_KIND);
                    if kind != KIND_TRANSLATION && kind != KIND_ROTATION {
                        continue;
                    }
                    if m & MASK_BITS == 0 {
                        continue;
                    }
                    // Record-id lookup on the owner: answers a bool and writes
                    // the record index (16 bits used) through its out pointer.
                    let mut slot = 0u32;
                    let found = lf_checker_rt::callee_thiscall!(
                        2,
                        u32,
                        owner,
                        rd16(track + TRACK_ID),
                        core::ptr::addr_of_mut!(slot) as u32
                    ) as u8;
                    if found == 0 {
                        continue;
                    }
                    let idx = slot & 0xffff;
                    let rec = rd32(owner) + idx * RECORD_STRIDE + 4;
                    let accepted = if kind == KIND_TRANSLATION {
                        rd32(rec) & MASK_TRANSLATION != 0
                    } else {
                        rd8(rec) & MASK_ROTATION != 0
                    };
                    if !accepted {
                        continue;
                    }
                    let src = rd32(rd32(this + 4) + HOLDER_RECORDS) + idx * RECORD_STRIDE;
                    if kind == KIND_TRANSLATION {
                        copy_translation(track, src);
                    } else {
                        copy_rotation(track, src);
                    }
                    clear_skip(track);
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
