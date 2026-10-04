// original: 0x0068F680 anim_apply_tracks_gated (proposed)

/// Write an animation's track values into a pose's bone matrices, letting a
/// callback object accept or reject each track first.
///
/// `this` holds two pointers: the track set (`+0x00`) and the pose target
/// (`+0x04`, whose matrix array pointer is at `+0x0c`, 0x50 bytes per bone).
/// A track set has a lookup key (`+0x04`), an enable word (`+0x08`), the track
/// pointer array (`+0x0c`) and a 16-bit track count (`+0x10`). A track has a
/// flags byte (`+0x04`, bit 0x10 means skip), a kind byte (`+0x05`: 0 =
/// translation, 1 = rotation), an id word (`+0x06`) and four floats at
/// `+0x10..+0x1c`. `owner` carries the bone table (`+0x00`) and a ready word
/// (`+0x2c`); `gate` is an object whose virtual slot at `+0x0c` is called per
/// track with (kind, id, weight slot) and answers accept/reject.
///
/// When the set is enabled, has a key, `owner` is ready, and the shared lookup
/// answers with an entry whose block (`+0x0c`) is present, the block lists
/// packed (track index << 16 | bone index) words: each unskipped named track
/// is offered to the gate, and on accept it is written to its bone. Otherwise
/// every unskipped track of a known kind is offered to the gate and, on
/// accept, its bone is found through the owner's bone-id lookup (callee),
/// accepted only if the bone record carries the kind's mask bits (0x380 for
/// translation, 0x0e for rotation) before the track is written to that bone's
/// matrix. The gate's weight write is never read back, and the kind is
/// re-read after the gate.
///
/// A translation is copied bit for bit to bone offsets 0x30..0x3c. A rotation
/// (quaternion x, y, z, w) is scaled by the root-2 constant, then expanded
/// into the 3x3 rotation part of the bone matrix (rows at 0x00, 0x10, 0x20)
/// with the original's float operation order. When the lookup supplied an
/// entry, its reference count is dropped under the owner's mutex at the end.
///
/// The bone-id lookup's out word aliases the caller's incoming argument slot
/// in the original; here it is a local, so the stack comparison is off for
/// this function. The kind reaches the gate as one byte of a stack word whose
/// upper bytes are frame leftovers, so only its low byte is compared.
///
/// Original: 0x0068F680 (thiscall, three stack words; only the first, `owner`,
/// and the second, `gate`, are read; returns nothing).
lf_checker_rt::export!(thiscall, rw_0068f680(this: u32, owner: u32, gate: u32, _a3: u32) -> u32 {
    unsafe {
        const SET_KEY: u32 = 0x04;
        const SET_ENABLED: u32 = 0x08;
        const SET_TRACKS: u32 = 0x0c;
        const SET_COUNT: u32 = 0x10;
        const POSE_MATRICES: u32 = 0x0c;
        const BONE_STRIDE: u32 = 0x50;
        const OWNER_READY: u32 = 0x2c;
        const OWNER_RECORD_STRIDE: u32 = 0xe0;
        const TRACK_FLAGS: u32 = 0x04;
        const TRACK_KIND: u32 = 0x05;
        const TRACK_ID: u32 = 0x06;
        const TRACK_VALUE: u32 = 0x10;
        const SKIP_FLAG: u8 = 0x10;
        const KIND_TRANSLATION: u8 = 0;
        const KIND_ROTATION: u8 = 1;
        const MASK_TRANSLATION: u32 = 0x380;
        const MASK_ROTATION: u8 = 0x0e;
        const GATE_SLOT: u32 = 0x0c;
        const ROOT2: f32 = f32::from_bits(0x3fb5_04f3);
        const ONE: f32 = 1.0;
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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

        /// Expand a track's quaternion into the bone's 3x3 rotation block, in
        /// the original's operation order.
        unsafe fn put_rotation(track: u32, bone: u32) {
            unsafe {
                let qb = mul(rdf(track + TRACK_VALUE + 4), ROOT2);
                let qa = mul(rdf(track + TRACK_VALUE), ROOT2);
                let qc = mul(rdf(track + TRACK_VALUE + 8), ROOT2);
                let qd = mul(rdf(track + TRACK_VALUE + 12), ROOT2);
                let ba = mul(qb, qa);
                let dc = mul(qd, qc);
                wrf(bone + 0x10, sub(ba, dc));
                wrf(bone + 0x04, add(ba, dc));
                let db = mul(qc, qa);
                let ca = mul(qd, qb);
                let cb = mul(qd, qa);
                wrf(bone + 0x08, sub(db, ca));
                wrf(bone + 0x20, add(ca, db));
                let aa = mul(qa, qa);
                let x1 = mul(qc, qb);
                let cc = mul(qc, qc);
                wrf(bone + 0x18, add(cb, x1));
                wrf(bone + 0x24, sub(x1, cb));
                let bb = mul(qb, qb);
                wrf(bone + 0x00, sub(ONE, add(cc, bb)));
                wrf(bone + 0x14, sub(ONE, add(cc, aa)));
                wrf(bone + 0x28, sub(ONE, add(bb, aa)));
            }
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

        /// Write `track` to bone `idx` by its re-read kind.
        unsafe fn put_by_kind(track: u32, mats: u32, idx: u32) {
            unsafe {
                let bone = mats + idx * BONE_STRIDE;
                match rd8(track + TRACK_KIND) {
                    KIND_TRANSLATION => put_translation(track, bone),
                    KIND_ROTATION => put_rotation(track, bone),
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
                    if rd8(track + TRACK_FLAGS) & SKIP_FLAG != 0 {
                        continue;
                    }
                    if ask_gate(gate, track) == 0 {
                        continue;
                    }
                    let mats = rd32(rd32(this + 4) + POSE_MATRICES);
                    put_by_kind(track, mats, item & 0xffff);
                }
            }
        } else {
            let n = rd16(set + SET_COUNT);
            if n > 0 {
                let tracks = rd32(set + SET_TRACKS);
                for i in 0..n {
                    let track = rd32(tracks + i * 4);
                    if rd8(track + TRACK_FLAGS) & SKIP_FLAG != 0 {
                        continue;
                    }
                    let kind = rd8(track + TRACK_KIND);
                    if kind != KIND_TRANSLATION && kind != KIND_ROTATION {
                        continue;
                    }
                    if ask_gate(gate, track) == 0 {
                        continue;
                    }
                    // Bone-id lookup on the owner: answers a bool and writes
                    // the bone index (16 bits used) through its out pointer.
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
                    let rec = rd32(owner) + idx * OWNER_RECORD_STRIDE + 4;
                    let accepted = if kind == KIND_TRANSLATION {
                        rd32(rec) & MASK_TRANSLATION != 0
                    } else {
                        rd8(rec) & MASK_ROTATION != 0
                    };
                    if !accepted {
                        continue;
                    }
                    put_by_kind(track, rd32(rd32(this + 4) + POSE_MATRICES), idx);
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
