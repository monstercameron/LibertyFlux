// original: 0x009E4B30 ped_task_availability_gate (proposed)

/// Decide whether a ped task of a given kind may run, by kind id.
///
/// `kind` selects one of two checks; `obj` points at the task object whose
/// first word is its type tag; `aux` is a helper object passed to the lookup
/// callee and, on the table path, holding a 16-bit table index at `+0x2e`.
///
/// Kind `0x53` probes two task slots (5 and 7) through a lookup callee: a slot
/// counts as usable only when the first lookup answers non-null, a stored
/// limit (a float constant in read-only data) is strictly greater than the
/// second answer's float at `+0x1c`, and a check callee called on that
/// answer returns a non-zero low byte. The second answer is dereferenced
/// without a null check, so a null there faults the same way. When slot 5 is not usable and the tag is `0x13f` or
/// `0x14c`, or slot 7 is not usable and the tag is `0x140` or `0x14d`, the
/// answer is 0; when slot 7 is usable the answer is 1. Kind `0x54` reads a
/// flags word at `+0x94` of the table entry selected by the signed index and
/// answers 1 when bit 5 is set, otherwise 0 for fourteen listed tags and 1
/// for anything else. Any other kind answers 1.
///
/// The low byte carries the boolean; the upper three bytes are whatever the
/// last value in `eax` held (the callee answer, the tag, or the kind), with
/// the low byte replaced. The float comparison is an exact greater-than, so
/// a NaN on either side counts as not greater, matching `comiss`/`jbe`.
///
/// Original: 0x009E4B30 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_009E4B30(kind: u32, obj: u32, aux: u32) -> u32 {
    unsafe {
        const KIND_SLOTS: u32 = 0x53;
        const KIND_TABLE: u32 = 0x54;
        const SLOT_FIRST: u32 = 5;
        const SLOT_SECOND: u32 = 7;
        const LIMIT_ADDR: u32 = 0x00FE870C;
        const TABLE_ADDR: u32 = 0x01295CD8;
        const CANDIDATE_VALUE: u32 = 0x1c;
        const INDEX_OFF: u32 = 0x2e;
        const FLAGS_OFF: u32 = 0x94;
        const FLAG_BIT: u32 = 5;
        const LOOKUP_FIRST: u32 = 1;
        const CHECK_FIRST: u32 = 2;
        const LOOKUP_SECOND: u32 = 3;
        const CHECK_SECOND: u32 = 4;
        const LOW_MASK: u32 = 0xFFFF_FF00;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }

        /// Probe one slot: the lookup must answer the first call non-null, the
        /// limit must exceed the second answer's value, and the check must
        /// answer non-zero. The second answer is dereferenced without a null
        /// check, so a null there faults. Returns (usable, eax after the probe).
        unsafe fn probe(aux: u32, slot: u32, lookup: u32, check: u32) -> (bool, u32) {
            unsafe {
                let first: u32 = lf_checker_rt::callee_thiscall!(lookup, u32, aux, slot);
                if first == 0 {
                    return (false, 1);
                }
                // No null check here: the original compares against the
                // pointed-to float immediately, faulting on a null answer.
                let cand: u32 = lf_checker_rt::callee_thiscall!(lookup, u32, aux, slot);
                let limit = f32::from_bits(rd32(lf_checker_rt::relocated(LIMIT_ADDR)));
                let got = f32::from_bits(rd32(cand.wrapping_add(CANDIDATE_VALUE)));
                if !(limit > got) {
                    return (false, (cand & LOW_MASK) | 1);
                }
                let verdict: u32 = lf_checker_rt::callee_thiscall!(check, u32, cand, 0);
                if verdict & 0xFF == 0 {
                    (false, (verdict & LOW_MASK) | 1)
                } else {
                    (true, verdict & LOW_MASK)
                }
            }
        }

        if kind == KIND_SLOTS {
            let (usable5, _) = probe(aux, SLOT_FIRST, LOOKUP_FIRST, CHECK_FIRST);
            let (usable7, eax) = probe(aux, SLOT_SECOND, LOOKUP_SECOND, CHECK_SECOND);
            let tag = rd32(obj);
            if !usable5 && (tag == 0x13F || tag == 0x14C) {
                return eax & LOW_MASK;
            }
            if usable7 {
                return (eax & LOW_MASK) | 1;
            }
            if tag == 0x140 || tag == 0x14D {
                return tag & LOW_MASK;
            }
            return (tag & LOW_MASK) | 1;
        }
        if kind == KIND_TABLE {
            let idx = rd16(aux.wrapping_add(INDEX_OFF)) as i16 as i32 as u32;
            let table = lf_checker_rt::relocated(TABLE_ADDR);
            let entry = rd32(table.wrapping_add(idx.wrapping_mul(4)));
            let shifted = rd32(entry.wrapping_add(FLAGS_OFF)) >> FLAG_BIT;
            if shifted & 1 == 1 {
                return (shifted & LOW_MASK) | 1;
            }
            let tag = rd32(obj);
            match tag {
                0x141 | 0x142 | 0x14F | 0x14E | 0x139 | 0x13A | 0x13D | 0x13E | 0x147
                | 0x148 | 0x153 | 0x154 | 0x177 | 0x178 => tag & LOW_MASK,
                _ => (tag & LOW_MASK) | 1,
            }
        } else {
            (kind & LOW_MASK) | 1
        }
    }
});
