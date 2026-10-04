// original: 0x0068FAE0 anim_apply_static_tracks (proposed)

/// Write an animation's constant (static) track values into a pose's bone
/// matrices.
///
/// `this` holds two pointers: the track set (`+0x00`) and the pose target
/// (`+0x04`, whose matrix array pointer is at `+0x0c`, 0x50 bytes per bone).
/// A track set has a lookup key (`+0x04`), an enable word (`+0x08`), the
/// track pointer array (`+0x0c`) and a 16-bit track count (`+0x10`). A track
/// has a flags byte (`+0x04`, bit 0x10 means skip), a kind byte (`+0x05`:
/// 0 = translation, 1 = rotation) and four floats at `+0x10..+0x1c`.
///
/// Fast path: when the track set is enabled, has a key, `owner` has its
/// `+0x2c` field set, and the shared lookup answers with an entry whose block
/// (`+0x0c`) is present, the block lists packed (track index << 16 | bone
/// index) words and each unskipped track is written to its bone. Otherwise
/// every track is visited and its bone is found through the owner's bone-id
/// lookup (callee), accepted only if the owner's per-bone record (0xe0 bytes,
/// flags at `+4`) has the kind's mask bits set (0x380 for translation, 0x0e
/// for rotation).
///
/// A translation is copied bit for bit to bone offsets 0x30..0x3c. A rotation
/// (quaternion x, y, z, w) is scaled by sqrt(2), then expanded into the 3x3
/// rotation part of the bone matrix (rows at 0x00, 0x10, 0x20) with the
/// original's float operation order. When the lookup supplied an entry, its
/// reference count is dropped under the owner's mutex at the end.
///
/// Original: 0x0068FAE0 (thiscall, three stack words; only the first, `owner`,
/// is read).
lf_checker_rt::export!(thiscall, rw_0068fae0(this: u32, owner: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        // Field offsets and constants.
        const TRACKSET_KEY: u32 = 0x04;
        const TRACKSET_ENABLED: u32 = 0x08;
        const TRACKSET_TRACKS: u32 = 0x0c;
        const TRACKSET_COUNT: u32 = 0x10;
        const POSE_MATRICES: u32 = 0x0c;
        const BONE_STRIDE: u32 = 0x50;
        const OWNER_READY: u32 = 0x2c;
        const OWNER_RECORD_STRIDE: u32 = 0xe0;
        const TRACK_FLAGS: u32 = 0x04;
        const TRACK_KIND: u32 = 0x05;
        const TRACK_ID: u32 = 0x06;
        const TRACK_VALUE: u32 = 0x10;
        const SKIP_FLAG: u8 = 0x10;
        const MASK_TRANSLATION: u32 = 0x380;
        const MASK_ROTATION: u8 = 0x0e;
        const KIND_TRANSLATION: u8 = 0;
        const KIND_ROTATION: u8 = 1;
        const SQRT2: f32 = f32::from_bits(0x3fb5_04f3);
        const ONE: f32 = 1.0;
        const ENTRY_REFS: u32 = 0x08;
        const ENTRY_BLOCK: u32 = 0x0c;
        const OWNER_MUTEX_OFFSET: u32 = 0x10;
        const INFINITE: u32 = 0xffff_ffff;
        // Import-address-table slots (file addresses) of the two mutex calls.
        const IAT_WAIT: u32 = 0x00e7_3188;
        const IAT_RELEASE: u32 = 0x00e7_31b0;

        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(addr: u32) -> u32 {
            unsafe { (addr as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(addr: u32) -> u8 {
            unsafe { (addr as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(addr: u32, v: u32) {
            unsafe { (addr as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(addr: u32) -> f32 {
            unsafe { f32::from_bits(rd32(addr)) }
        }
        #[inline(always)]
        unsafe fn wrf(addr: u32, v: f32) {
            unsafe { wr32(addr, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        /// Copy a translation track's four words to the bone's translation row.
        unsafe fn put_translation(track: u32, bone: u32) {
            unsafe {
                wr32(bone + 0x30, rd32(track + TRACK_VALUE));
                wr32(bone + 0x34, rd32(track + TRACK_VALUE + 4));
                wr32(bone + 0x38, rd32(track + TRACK_VALUE + 8));
                wr32(bone + 0x3c, rd32(track + TRACK_VALUE + 12));
            }
        }

        /// Expand a track's quaternion into the bone's 3x3 rotation block.
        unsafe fn put_rotation(track: u32, bone: u32) {
            unsafe {
                let qa = mul(rdf(track + TRACK_VALUE), SQRT2);
                let qb = mul(rdf(track + TRACK_VALUE + 4), SQRT2);
                let qc = mul(rdf(track + TRACK_VALUE + 8), SQRT2);
                let qd = mul(rdf(track + TRACK_VALUE + 12), SQRT2);
                let ba = mul(qb, qa);
                let dc = mul(qd, qc);
                wrf(bone + 0x10, sub(ba, dc));
                wrf(bone + 0x04, add(ba, dc));
                let ca = mul(qc, qa);
                let db = mul(qd, qb);
                let da = mul(qd, qa);
                wrf(bone + 0x08, sub(ca, db));
                wrf(bone + 0x20, add(db, ca));
                let aa = mul(qa, qa);
                let cb = mul(qc, qb);
                let cc = mul(qc, qc);
                wrf(bone + 0x18, add(da, cb));
                wrf(bone + 0x24, sub(cb, da));
                let bb = mul(qb, qb);
                let cc_bb = add(cc, bb);
                let cc_aa = add(cc, aa);
                let bb_aa = add(bb, aa);
                wrf(bone + 0x00, sub(ONE, cc_bb));
                wrf(bone + 0x14, sub(ONE, cc_aa));
                wrf(bone + 0x28, sub(ONE, bb_aa));
            }
        }

        let track_set = rd32(this);
        // The shared lookup fills these two words: the owning object (holds the
        // mutex handle) and the entry (holds the reference count and block).
        let mut lookup_out = [0u32; 2];
        if rd32(track_set + TRACKSET_ENABLED) != 0 && rd32(owner + OWNER_READY) != 0 {
            let key = rd32(track_set + TRACKSET_KEY);
            if key != 0 {
                lf_checker_rt::callee_stdcall!(
                    1,
                    u32,
                    core::ptr::addr_of_mut!(lookup_out) as u32,
                    track_set,
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
                let tracks = rd32(rd32(this) + TRACKSET_TRACKS);
                let mut item_addr = block + 4;
                for _ in 0..count {
                    let item = rd32(item_addr);
                    item_addr += 4;
                    let track = rd32(tracks + (item >> 16) * 4);
                    if rd8(track + TRACK_FLAGS) & SKIP_FLAG != 0 {
                        continue;
                    }
                    let bone = rd32(rd32(this + 4) + POSE_MATRICES) + (item & 0xffff) * BONE_STRIDE;
                    match rd8(track + TRACK_KIND) {
                        KIND_TRANSLATION => put_translation(track, bone),
                        KIND_ROTATION => put_rotation(track, bone),
                        _ => {}
                    }
                }
            }
        } else {
            let set = rd32(this);
            let n = rd16(set + TRACKSET_COUNT);
            let tracks = rd32(set + TRACKSET_TRACKS);
            for i in 0..n {
                let track = rd32(tracks + i * 4);
                if rd8(track + TRACK_FLAGS) & SKIP_FLAG != 0 {
                    continue;
                }
                let kind = rd8(track + TRACK_KIND);
                if kind != KIND_TRANSLATION && kind != KIND_ROTATION {
                    continue;
                }
                // Bone-id lookup on the owner: answers a bool and writes the
                // bone index (16 bits used) through its out pointer.
                let mut bone_slot = 0u32;
                let found = lf_checker_rt::callee_thiscall!(
                    2,
                    u32,
                    owner,
                    rd16(track + TRACK_ID),
                    core::ptr::addr_of_mut!(bone_slot) as u32
                ) as u8;
                if found == 0 {
                    continue;
                }
                let idx = bone_slot & 0xffff;
                let record_flags = rd32(rd32(owner) + idx * OWNER_RECORD_STRIDE + 4);
                let accepted = if kind == KIND_TRANSLATION {
                    record_flags & MASK_TRANSLATION != 0
                } else {
                    (record_flags as u8) & MASK_ROTATION != 0
                };
                if !accepted {
                    continue;
                }
                let bone = rd32(rd32(this + 4) + POSE_MATRICES) + idx * BONE_STRIDE;
                if kind == KIND_TRANSLATION {
                    put_translation(track, bone);
                } else {
                    put_rotation(track, bone);
                }
            }
        }

        // Drop the entry's reference count under the owner's mutex.
        if entry != 0 {
            let mutex = rd32(lock_owner + OWNER_MUTEX_OFFSET);
            if mutex != 0 {
                let wait: extern "stdcall" fn(u32, u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::global::<u32>(IAT_WAIT).read() as usize);
                wait(mutex, INFINITE);
            }
            wr32(entry + ENTRY_REFS, rd32(entry + ENTRY_REFS).wrapping_sub(1));
            let mutex = rd32(lock_owner + OWNER_MUTEX_OFFSET);
            if mutex != 0 {
                let release: extern "stdcall" fn(u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::global::<u32>(IAT_RELEASE).read() as usize);
                release(mutex);
            }
        }
        0
    }
});
