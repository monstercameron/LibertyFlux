// original: 0x0068e530 anim_remap_tracks_masked (proposed)

/// Remap an animation's tracks through record and index lookups, gated by a
/// per-track mask and dispatching on the track kind.
///
/// `this` points to the animation (`+0x00` track set, `+0x04` owner). The
/// first stack word is never read; `mask` (second stack word) points at an
/// array of mask dwords consumed one per track. The track set holds a
/// track-pointer array at `+0x0c` and a 16-bit track count at `+0x10`. Each
/// track has a flag byte at `+0x04`, a kind byte at `+0x05` and a key at
/// `+0x06`. The owner's `+0x00` points at the record table of 0xe0-byte
/// records (flag word at `+0x04`, index word at `+0x18`).
///
/// Tracks whose flag byte has bit 0x10 set, or whose mask word has no low-31
/// bit set, are skipped entirely. The rest dispatch on the kind: 0 and 1
/// run the lookup path below (with different record flag tests), 2 to 4
/// and anything above 6 are skipped, 5 and 6 only make the track's virtual
/// call.
///
/// The lookup path does a record-id lookup (callee 3, thiscall on the
/// owner, track key plus a 16-bit out word) and continues only when it
/// answers true and the record's flag word passes (bits 0x0e of the flag
/// byte for kind 1, bits 0x380 of the flag dword for kind 0). It then makes
/// the track's virtual call (slot `+0x20`, thiscall with one zero word) and,
/// when the record's index word differs from the looked-up id, an index
/// lookup (callee 8, thiscall on the owner, index plus a 16-bit out word).
/// When that answers true and its out word is below the track key, a final
/// remap call runs (callee 9, thiscall on the set, kind, key, kind and the
/// full out word).
///
/// Note: the kind-1 record-id lookup writes its out word over the incoming
/// second stack word (dead by then: the mask cursor lives in a register),
/// so the original's incoming stack differs from any rewrite's there; the
/// contract compares everything else.
///
/// Original: 0x0068e530 (thiscall, two stack words; the first is not read).
/// Pure integer code, no floating point.
lf_checker_rt::export!(thiscall, rw_0068e530(this: u32, _a1: u32, mask: u32) -> u32 {
    unsafe {
        const SET_TRACKS: u32 = 0x0c;
        const SET_COUNT: u32 = 0x10;
        const OWNER_RECORDS: u32 = 0x00;
        const TRACK_FLAGS: u32 = 0x04;
        const TRACK_KIND: u32 = 0x05;
        const TRACK_KEY: u32 = 0x06;
        const TRACK_VCALL: u32 = 0x20;
        const RECORD_STRIDE: u32 = 0xe0;
        const RECORD_FLAGS: u32 = 0x04;
        const RECORD_INDEX: u32 = 0x18;
        const KIND1_FLAG_BITS: u8 = 0x0e;
        const KIND0_FLAG_BITS: u32 = 0x380;
        const SKIP_FLAG: u8 = 0x10;
        const MASK_BITS: u32 = 0x7fff_ffff;
        const BONE_LOOKUP: u32 = 3;
        const INDEX_LOOKUP: u32 = 8;
        const REMAP: u32 = 9;

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

        /// The track's virtual call: slot `+0x20`, thiscall with one zero word.
        #[inline(always)]
        unsafe fn track_vcall(track: u32) {
            unsafe {
                let target = rd32(rd32(track) + TRACK_VCALL);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                f(track, 0);
            }
        }

        let set = rd32(this);
        let owner = rd32(this + 4);
        let total = rd16(set + SET_COUNT) as i32;
        if total > 0 {
            let array = rd32(set + SET_TRACKS);
            let mut mcur = mask;
            for i in 0..total as u32 {
                let track = rd32(array + i * 4);
                let m = rd32(mcur);
                mcur = mcur.wrapping_add(4);
                if rd8(track + TRACK_FLAGS) & SKIP_FLAG != 0 {
                    continue;
                }
                if m & MASK_BITS == 0 {
                    continue;
                }
                let kind = rd8(track + TRACK_KIND);
                if kind != 0 && kind != 1 {
                    if kind == 5 || kind == 6 {
                        track_vcall(track);
                    }
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
                } else if rd32(rec + RECORD_FLAGS) & KIND0_FLAG_BITS == 0 {
                    continue;
                }
                track_vcall(track);
                let index = rd16(rec + RECORD_INDEX);
                if index == id {
                    continue;
                }
                let mut out: u32 = 0;
                let ok: u32 = lf_checker_rt::callee_thiscall!(
                    INDEX_LOOKUP,
                    u32,
                    owner,
                    index,
                    &mut out as *mut u32 as u32
                );
                if (ok & 0xff) == 0 {
                    continue;
                }
                let key = rd16(track + TRACK_KEY);
                if (out & 0xffff) >= key {
                    continue;
                }
                lf_checker_rt::callee_thiscall!(REMAP, u32, set, kind as u32, key, kind as u32, out);
            }
        }
        0
    }
});
