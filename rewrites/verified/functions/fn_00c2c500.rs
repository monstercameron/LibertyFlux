// original: 0x00c2c500 audEntityRadioEmitter::vf8

/// Audibility/state query for a radio emitter bound to an entity.
///
/// `this` points to the emitter object: the vtable pointer at `+0x00` and the
/// bound inner entity at `+0x04` (null when unbound). The inner entity carries
/// a kind word at `+0x28` and two flag bytes at `+0x218`/`+0x219`.
///
/// Behaviour, in order:
/// - With no bound entity the answer is 7.
/// - Let `class` be bits 6..10 of the kind word. When `class` is 3 and the
///   first flag byte is clear, a set second flag byte also answers 7.
/// - When `class` is 2, a helper object is fetched (callee 1); if its word at
///   `+0x26C` has bit 2 set, its link at `+0xB30` is compared against the
///   bound entity, and a match answers 7 (a clear bit compares 0 instead,
///   which never matches a bound entity).
/// - Otherwise the emitter's own position is queried through virtual slot 1
///   (callee 2, out-pointer in the caller's frame) and compared against a
///   listener position: the global at `TLS_INDEX` names a TLS slot whose
///   block holds an entry index at `+0x70`, selecting one 64-byte entry of
///   the table at `POS_TABLE`. The squared distance is accumulated as
///   `(dx*dx + dy*dy) + dz*dz` in that operand order; when it reaches the
///   global limit at `DIST_LIMIT` the answer is 1.
/// - When the kind word instead has the radio-path bits (`0x80` under mask
///   `0x3C0`) and a final gate call (callee 3) answers nonzero, the answer
///   is 4.
/// - Anything reaching the end answers 2.
///
/// NaN distances behave as "less than": the original's `comiss`/`jb` pair
/// takes the below-limit path on unordered results, so only an ordered
/// distance at or above the limit answers 1.
///
/// Original: 0x00c2c500 (thiscall, no stack words, u32 return).
lf_checker_rt::export!(thiscall, rw_00c2c500(this: u32) -> u32 {
    unsafe {
        const INNER: u32 = 0x04;
        const KIND: u32 = 0x28;
        const FLAG_FIRST: u32 = 0x218;
        const FLAG_SECOND: u32 = 0x219;
        const VTABLE_SLOT_POS: u32 = 0x04;
        const TLS_INDEX: u32 = 0x17ABA14;
        const TLS_ENTRY_INDEX: u32 = 0x70;
        const POS_TABLE: u32 = 0x115DF20;
        const POS_STRIDE_SHIFT: u32 = 6;
        const DIST_LIMIT: u32 = 0xFE8C20;
        const RADIO_MASK: u32 = 0x3C0;
        const RADIO_PATH: u32 = 0x80;
        const GATE_BIT: u32 = 4;
        const LINK_A: u32 = 0x26C;
        const LINK_B: u32 = 0xB30;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { (a as *const f32).read_unaligned() }
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let inner = rd32(this.wrapping_add(INNER));
        if inner == 0 {
            return 7;
        }
        let class = (rd32(inner.wrapping_add(KIND)) >> 6) & 0xF;
        if class == 3 {
            if rd8(inner.wrapping_add(FLAG_FIRST)) == 0
                && rd8(inner.wrapping_add(FLAG_SECOND)) != 0
            {
                return 7;
            }
        }
        if class == 2 {
            let obj: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
            let linked = if rd32(obj.wrapping_add(LINK_A)) & GATE_BIT != 0 {
                rd32(obj.wrapping_add(LINK_B))
            } else {
                0
            };
            if linked == rd32(this.wrapping_add(INNER)) {
                return 7;
            }
        }

        let mut pos = [0f32; 3];
        let vptr = rd32(this);
        let slot = rd32(vptr.wrapping_add(VTABLE_SLOT_POS));
        let query: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        query(this, pos.as_mut_ptr() as u32);

        let slot_index = rd32(lf_checker_rt::relocated(TLS_INDEX));
        let block = lf_checker_rt::tls_slot(slot_index as usize);
        let entry = rd32(block.wrapping_add(TLS_ENTRY_INDEX));
        let base = lf_checker_rt::relocated(POS_TABLE)
            .wrapping_add(entry.wrapping_shl(POS_STRIDE_SHIFT));
        let dx = fsub(rdf(base), pos[0]);
        let dy = fsub(rdf(base.wrapping_add(4)), pos[1]);
        let dz = fsub(rdf(base.wrapping_add(8)), pos[2]);
        let dist2 = fadd(fadd(fmul(dx, dx), fmul(dy, dy)), fmul(dz, dz));
        let limit = rdf(lf_checker_rt::relocated(DIST_LIMIT));
        if dist2 >= limit {
            return 1;
        }

        let inner2 = rd32(this.wrapping_add(INNER));
        if rd32(inner2.wrapping_add(KIND)) & RADIO_MASK == RADIO_PATH {
            let gate: u32 = lf_checker_rt::callee_thiscall!(3, u32, inner2);
            if gate & 0xFF != 0 {
                return 4;
            }
        }
        2
    }
});
