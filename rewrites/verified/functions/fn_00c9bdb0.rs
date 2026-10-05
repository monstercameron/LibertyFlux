// original: 0x00c9bdb0 skipset_gated_accumulator

/// Accumulate a float argument into per-index records, gated by a skip-set.
///
/// `this` points at a task whose word at `+0x40` is the subject object;
/// `farg` is a float passed by value. Returns eax: 0 after any loop pass,
/// otherwise the low 6 bits of the last table answer.
///
/// Behaviour: a shared virtual probe (slot `+0xa0` on the subject, null
/// answer selects the `+0x100` fallback, otherwise a second `+0xa0` call
/// followed by slot `+0xe0` on its answer) yields a record whose word at
/// `+4` points at a header; the header's halfword at `+0x14` is the index
/// count N. Nine global dwords are each pushed to a virtual call (slot
/// `+0x38`) on the object selected by signing-extending the subject's
/// halfword at `+0x2e` into the global object table; each answer's low six
/// bits set one bit of a 128-bit set kept on the stack (answers stay below
/// 128 by contract, so the set index stays in range). Then for each index
/// below N a 64-bit mask tests the set: indexes whose bit is set are
/// skipped, the rest run the direct callee with the index and add `farg`
/// to the answer's word at `+0x38`. The mask shifts out after 64 indexes,
/// so indexes at and past 64 always run.
///
/// Original: 0x00c9bdb0 (thiscall, one stack word; returns eax).
#[allow(clippy::all)]
#[allow(unsafe_code)]
unsafe fn run_00c9bdb0(this: u32, farg: u32, mutate_gate: bool) -> u32 {
    unsafe {
        const SLOT_PROBE: u32 = 0xa0;
        const SLOT_RESOLVE: u32 = 0xe0;
        const SLOT_TABLE: u32 = 0x38;
        const DIRECT_CALLEE: u32 = 4;
        const G_WORDS: u32 = 0x1050c74;
        const G_TABLE: u32 = 0x1295cd8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let tgt: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                tgt(obj)
            }
        }
        #[inline(always)]
        unsafe fn vcall1(obj: u32, slot: u32, arg: u32) -> u32 {
            unsafe {
                let tgt: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                tgt(obj, arg)
            }
        }

        let obj = rd32(this.wrapping_add(0x40));
        let first = vcall0(obj, SLOT_PROBE);
        let rec = if first == 0 {
            rd32(obj.wrapping_add(0x100))
        } else {
            let second = vcall0(obj, SLOT_PROBE);
            vcall0(second, SLOT_RESOLVE)
        };
        let n = rd16(rd32(rec.wrapping_add(4)).wrapping_add(0x14)) as u32;

        let mut set = [0u32; 4];
        let mut last_e0 = 0u32;
        let mut k = 0u32;
        while k < 9 {
            let g: u32 = lf_checker_rt::global::<u32>(G_WORDS.wrapping_add(4 * k)).read();
            let idx = rd16(obj.wrapping_add(0x2e)) as i16 as i32 as u32;
            let tobj: u32 =
                lf_checker_rt::global::<u32>(G_TABLE.wrapping_add(idx.wrapping_mul(4))).read();
            let ans = vcall1(tobj, SLOT_TABLE, g);
            let e0 = ans & 0x3f;
            last_e0 = e0;
            let bit = 1u32 << (e0 & 31);
            // Answers below 128 keep this index at 0 or 1 (contract).
            let word = (ans as i32 >> 6) as usize;
            let mut lo = bit;
            let mut hi = 0u32;
            if e0 >= 0x20 {
                hi = lo;
                lo ^= hi;
            }
            if e0 >= 0x40 {
                // Dead: e0 holds six bits. Replicated for fidelity.
                hi = lo;
            }
            set[word * 2] |= lo;
            set[word * 2 + 1] |= hi;
            k += 1;
        }

        let f = f32::from_bits(farg);
        let mut lo = 1u32;
        let mut hi = 0u32;
        let mut i = 0u32;
        while i < n {
            // Indexes below 128 keep this at 0 or 1 (contract).
            let w = (i >> 6) as usize;
            let t = (set[w * 2] & lo) | (set[w * 2 + 1] & hi);
            // MUTANT (mut_00c9bdb0): run the gated indexes instead.
            let gated = if mutate_gate { t != 0 } else { t == 0 };
            if gated {
                let r: u32 = lf_checker_rt::callee_thiscall!(DIRECT_CALLEE, u32, obj, i);
                let slot = r.wrapping_add(0x38);
                let sum = core::hint::black_box(f)
                    + core::hint::black_box(f32::from_bits(rd32(slot)));
                (slot as *mut u32).write_unaligned(sum.to_bits());
            }
            let old_hi = hi;
            hi = (hi << 1) | (lo >> 31);
            let carry = old_hi >> 31;
            lo = (lo << 1) | carry;
            i += 1;
        }
        if n == 0 { last_e0 } else { 0 }
    }
}

lf_checker_rt::export!(thiscall, rw_00c9bdb0(this: u32, farg: u32) -> u32 {
    unsafe { run_00c9bdb0(this, farg, false) }
});

lf_checker_rt::export!(thiscall, mut_00c9bdb0(this: u32, farg: u32) -> u32 {
    unsafe { run_00c9bdb0(this, farg, true) }
});
