// original: 0x0068dd30 anim_apply_pose_masked (proposed)

/// Apply a pose to an animation's tracks, gated by a per-track mask: a fast
/// path over packed (track, bone) pairs from a shared lookup, or a slow
/// path over every track with a per-track bone lookup, then a mutex-guarded
/// reference drop.
///
/// `this` points to the animation (`+0x00` track set, `+0x04` pose holder).
/// `owner` (first stack word) points to the owner (`+0x00` record table of
/// 0xe0-byte records, `+0x2c` ready flag). `mask` (third stack word) points
/// at an array of mask dwords; the second stack word is never read. The
/// track set holds a key at `+0x04`, an enable flag at `+0x08`, a
/// track-pointer array at `+0x0c` and a 16-bit track count at `+0x10`. Each
/// track has a flag byte at `+0x04`, a kind byte at `+0x05`, a key at `+0x06`
/// and four value words at `+0x10`. The pose holder's `+0x0c` points at the
/// pose array of 0x50-byte bone entries whose words at `+0x30..+0x3c` are the
/// source values.
///
/// When the set is enabled, the owner ready and the set key non-null, a
/// shared lookup (callee 1, stdcall, four stack words, two-word out struct:
/// lock, entry) runs. When it yields an entry with a non-null `+0x0c` block,
/// the fast path walks the block (a count followed by packed words: track
/// index in the high half, bone index in the low half). Each pair resolves
/// its mask word by track index and is skipped untouched when its low 31
/// bits are zero; otherwise it resolves its track through the array and its
/// bone through the pose array. Kind 0 copies the bone's four source words
/// into the track verbatim, kind 1 runs them through the quaternion callee
/// (callee 2, out buffer in ecx, one stack word) and copies the 16-byte
/// result, any other kind stores nothing and is left untouched. Every
/// handled track has flag bit 0x10 cleared.
///
/// Otherwise the slow path walks all tracks of the set with a mask cursor
/// advancing one dword per track: kind 1 does a bone-id lookup (callee 3,
/// thiscall, key plus a 16-bit out word) and continues only when the
/// track's mask word has any low-31 bit set and the record's flag byte has
/// bits 0x0e, running the quaternion callee; kind 0 does the same lookup
/// and continues only when the mask word passes and the record's flag dword
/// has bits 0x380, copying the four source words; other kinds are skipped.
/// Handled tracks have flag bit 0x10 cleared.
///
/// When the lookup returned an entry, the tail decrements the entry's
/// reference count at `+0x08` under the lock's mutex at `+0x10`
/// (WaitForSingleObject/ReleaseMutex through the import table, only when the
/// handle is non-zero).
///
/// Original: 0x0068dd30 (thiscall, three stack words; the second is not
/// read). No floating-point arithmetic: all value moves are bitwise.
lf_checker_rt::export!(thiscall, rw_0068dd30(this: u32, owner: u32, _a2: u32, mask: u32) -> u32 {
    unsafe {
        const SET_KEY: u32 = 0x04;
        const SET_ENABLED: u32 = 0x08;
        const SET_TRACKS: u32 = 0x0c;
        const SET_COUNT: u32 = 0x10;
        const HOLDER_POSE: u32 = 0x0c;
        const OWNER_RECORDS: u32 = 0x00;
        const OWNER_READY: u32 = 0x2c;
        const TRACK_FLAGS: u32 = 0x04;
        const TRACK_KIND: u32 = 0x05;
        const TRACK_KEY: u32 = 0x06;
        const TRACK_VALUE: u32 = 0x10;
        const POSE_STRIDE: u32 = 0x50;
        const POSE_SRC: u32 = 0x30;
        const RECORD_STRIDE: u32 = 0xe0;
        const RECORD_FLAGS: u32 = 0x04;
        const KIND1_FLAG_BITS: u8 = 0x0e;
        const KIND0_FLAG_BITS: u32 = 0x380;
        const MASK_BITS: u32 = 0x7fff_ffff;
        const HANDLED_MASK: u8 = 0xef;
        const ENTRY_REFS: u32 = 0x08;
        const ENTRY_BLOCK: u32 = 0x0c;
        const LOCK_MUTEX: u32 = 0x10;
        const LOOKUP: u32 = 1;
        const QUAT: u32 = 2;
        const BONE_LOOKUP: u32 = 3;
        const WAIT: u32 = 4;
        const RELEASE: u32 = 5;

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

        /// Copy the bone's four source words into the track verbatim.
        #[inline(always)]
        unsafe fn copy_bone(track: u32, bone: u32) {
            unsafe {
                for w in 0..4u32 {
                    wr32(track + TRACK_VALUE + w * 4, rd32(bone + POSE_SRC + w * 4));
                }
            }
        }

        /// Run the bone through the quaternion callee into the track.
        #[inline(always)]
        unsafe fn apply_quat(track: u32, bone: u32) {
            unsafe {
                let mut out = [0u32; 4];
                lf_checker_rt::callee_thiscall!(QUAT, u32, out.as_mut_ptr() as u32, bone);
                for w in 0..4u32 {
                    wr32(track + TRACK_VALUE + w * 4, out[w as usize]);
                }
            }
        }

        #[inline(always)]
        unsafe fn clear_handled(track: u32) {
            unsafe {
                let f = rd8(track + TRACK_FLAGS);
                ((track + TRACK_FLAGS) as *mut u8).write(f & HANDLED_MASK);
            }
        }

        let set = rd32(this);
        let holder = rd32(this + 4);
        let pose = rd32(holder + HOLDER_POSE);
        let mut lock = 0u32;
        let mut entry = 0u32;
        if rd32(set + SET_ENABLED) != 0 && rd32(owner + OWNER_READY) != 0 {
            let key = rd32(set + SET_KEY);
            if key != 0 {
                let mut out = [0u32; 2];
                lf_checker_rt::callee_stdcall!(
                    LOOKUP,
                    u32,
                    out.as_mut_ptr() as u32,
                    set,
                    owner,
                    key
                );
                lock = out[0];
                entry = out[1];
            }
        }
        // Fast path over the packed pairs, or the slow path over every track
        // of the set when the lookup gave no usable block. Never both.
        let block = if entry != 0 { rd32(entry + ENTRY_BLOCK) } else { 0 };
        if block != 0 {
            let tracks = rd32(set + SET_TRACKS);
            let count = rd32(block) as i32;
            if count > 0 {
                for i in 0..count as u32 {
                    let packed = rd32(block + 4 + i * 4);
                    let idx = packed >> 16;
                    let m = rd32(mask + idx * 4);
                    let track = rd32(tracks + idx * 4);
                    if m & MASK_BITS == 0 {
                        continue;
                    }
                    let bone = pose.wrapping_add((packed & 0xffff) * POSE_STRIDE);
                    match rd8(track + TRACK_KIND) {
                        0 => {
                            copy_bone(track, bone);
                            clear_handled(track);
                        }
                        1 => {
                            apply_quat(track, bone);
                            clear_handled(track);
                        }
                        _ => {}
                    }
                }
            }
        } else {
            let total = rd16(set + SET_COUNT) as i32;
            if total > 0 {
                let array = rd32(set + SET_TRACKS);
                let mut mcur = mask;
                for i in 0..total as u32 {
                    let track = rd32(array + i * 4);
                    let m = rd32(mcur);
                    mcur = mcur.wrapping_add(4);
                    let kind = rd8(track + TRACK_KIND);
                    if kind != 0 && kind != 1 {
                        continue;
                    }
                    if m & MASK_BITS == 0 {
                        continue;
                    }
                    // Out slot is a full word; only the low 16 bits are read back.
                    let mut id: u32 = 0;
                    let ok: u32 = lf_checker_rt::callee_thiscall!(
                        BONE_LOOKUP,
                        u32,
                        owner,
                        rd16(track + TRACK_KEY),
                        &mut id as *mut u32 as u32
                    );
                    if (ok & 0xff) == 0 {
                        continue;
                    }
                    let id = id & 0xffff;
                    let rec = rd32(owner + OWNER_RECORDS).wrapping_add(id * RECORD_STRIDE);
                    if kind == 1 {
                        if rd8(rec + RECORD_FLAGS) & KIND1_FLAG_BITS == 0 {
                            continue;
                        }
                        apply_quat(track, pose.wrapping_add(id * POSE_STRIDE));
                    } else {
                        if rd32(rec + RECORD_FLAGS) & KIND0_FLAG_BITS == 0 {
                            continue;
                        }
                        copy_bone(track, pose.wrapping_add(id * POSE_STRIDE));
                    }
                    clear_handled(track);
                }
            }
        }
        // Mutex-guarded reference drop.
        if entry != 0 {
            let mutex = rd32(lock + LOCK_MUTEX);
            if mutex != 0 {
                lf_checker_rt::callee_stdcall!(WAIT, u32, mutex, 0xffff_ffff);
            }
            wr32(entry + ENTRY_REFS, rd32(entry + ENTRY_REFS).wrapping_sub(1));
            let mutex = rd32(lock + LOCK_MUTEX);
            if mutex != 0 {
                lf_checker_rt::callee_stdcall!(RELEASE, u32, mutex);
            }
        }
        0
    }
});
