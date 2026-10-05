// original: 0x00a95890 stream_bump_counters_by_kind (proposed)

/// Bump per-kind counters selected by a query, then fix the kind bits.
///
/// The low two bits of the word at `this+0x08` pick a path with the
/// argument's low byte: both clear with a nonzero byte increments
/// (path A), both set with a zero byte decrements (path B), anything else
/// does nothing. On path A/B the entry index `(this-table)/24` (table from
/// its global) and a stack out-array go to the query (callee 1, thiscall/2
/// on a fixed object); each of the returned count of out-words selects a
/// counter (`counter_table[word*24+0x0c]`, table from its global) to
/// increment (A) or decrement (B). Finally the argument's low two bits are
/// installed into `this+0x08`.
///
/// Returns those installed bits. Thiscall: object in ecx, one stack word,
/// callee pops 4. (The stack-cookie check is mirrored as a no-op call.)
lf_checker_rt::export!(thiscall, rw_00a95890(this: u32, arg: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 0x08;
        const TABLE_GLOBAL: u32 = 0x012fb3a8;
        const QUERY_OBJ: u32 = 0x01305350;
        const COUNTER_GLOBAL: u32 = 0x0103e8d0;
        const COUNTER_OFF: u32 = 0x0c;
        const KIND_MASK: u32 = 3;
        let word = ((this + KIND_OFF) as *const u32).read_unaligned();
        let bl = arg as u8;
        let path_a = (word & KIND_MASK) == 0 && bl != 0;
        let path_b = (word & KIND_MASK) != 0 && word != 0 && bl == 0;
        if path_a || path_b {
            let base = lf_checker_rt::global::<u32>(TABLE_GLOBAL).read_unaligned();
            let idx = ((this.wrapping_sub(base) as i32) / 24) as u16 as u32;
            let mut out = [0u32; 3];
            let n = lf_checker_rt::callee_thiscall!(
                1,
                u32,
                lf_checker_rt::relocated(QUERY_OBJ),
                idx,
                out.as_mut_ptr() as u32
            );
            let table = lf_checker_rt::global::<u32>(COUNTER_GLOBAL).read_unaligned();
            for i in 0..n {
                let v = out[i as usize];
                let slot = (table + v.wrapping_mul(3).wrapping_mul(8) + COUNTER_OFF)
                    as *mut u16;
                if path_a {
                    slot.write_unaligned(slot.read_unaligned().wrapping_add(1));
                } else {
                    slot.write_unaligned(slot.read_unaligned().wrapping_sub(1));
                }
            }
        }
        let mask = ((bl as u32) ^ ((this + KIND_OFF) as *const u32).read_unaligned())
            & KIND_MASK;
        let w = (this + KIND_OFF) as *mut u32;
        w.write_unaligned(w.read_unaligned() ^ mask);
        lf_checker_rt::callee_cdecl!(4, u32,);
        mask
    }
});
