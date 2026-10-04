// original: 0x00d959c0 resolve_cell_and_box (proposed)

/// Resolve a query point to a table entry, then report an 8x-quantised
/// bounding box for it.
///
/// `esi` points to the query: center at `+0x20`, spread vector at `+0x30`,
/// radius at `+0x40`. The lookup callee (id 0) is asked with the center
/// pointer and two out-slots; a nonzero answer, or a null table or entry
/// out-slot, takes the fail path (flag bit 2 set at `+0x10`, status 0 at
/// `+0x44`, the answer at `+0x8`, al 0). On success the refresh callee
/// (id 1) runs, the entry's tag word (`this+0xC42`) is copied to the entry
/// at `+0x8`, status 1 and the entry's low nibble are recorded, the entry
/// index (byte distance from the table base divided by 40, via the
/// multiply-shift idiom) is passed to the measure callee (id 2, float
/// result on the x87 stack, stored at `+0x8C`), the fill callee (id 3)
/// runs, and finally the report callee (id 4) receives the query, table,
/// entry and six halfwords holding `(center +/- extent) * 8` truncated
/// toward zero (unrepresentable conversions yield 0, matching cvttss2si
/// low-half behaviour). The extent is the stored radius, or 1.0 when the
/// spread's squared length exceeds 0.25 (a NaN spread keeps the radius).
///
/// Original: 0x00d959c0 (thiscall, one stack word, full-eax 0/1-ish result
/// whose high bytes above AL come from the report callee's answer).
lf_checker_rt::export!(thiscall, rw_00d959c0(this: u32, esi: u32) -> u32 {
    unsafe {
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
        const LOOKUP: u32 = 0;
        const REFRESH: u32 = 1;
        const MEASURE: u32 = 2;
        const FILL: u32 = 3;
        const REPORT: u32 = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        // Truncate toward zero with x86 cvttss2si semantics: NaN, infinities
        // and out-of-range values convert to 0x80000000; only the low half
        // is observed.
        #[inline(always)]
        fn cvtt16(v: f32) -> u16 {
            let t = v.trunc();
            if t.is_nan() || t >= 2147483648.0 || t < -2147483648.0 {
                0
            } else {
                (t as i32) as u16
            }
        }
        let mut table_out: u32 = 0;
        let mut entry_out: u32 = 0;
        let ans: u32 = lf_checker_rt::callee_thiscall!(
            LOOKUP, u32, this, esi.wrapping_add(0x20),
            core::ptr::addr_of_mut!(table_out) as u32,
            core::ptr::addr_of_mut!(entry_out) as u32,
            0, 0x40000000, 0);
        let (table, entry) = (table_out, entry_out);
        if ans != 0 || table == 0 || entry == 0 {
            let flags = (rd32(esi + 0x10) & 0xfffffffd) | 4;
            wr32(esi + 0x10, flags);
            wr32(esi + 0x44, 0);
            wr32(esi + 0x8, ans);
            return flags & 0xffffff00;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(REFRESH, u32, this);
        wr16(entry + 8, rd16(this + 0xc42));
        wr32(esi + 0x44, 1);
        wr32(esi + 0x4c, rd32(entry + 0x1c) & 0xf);
        let diff = (entry as i32).wrapping_sub(rd32(table + 0x6c) as i32);
        let hi = ((diff as i64).wrapping_mul(0x66666667) >> 32) as i32;
        let idx = hi >> 4;
        let idx = idx.wrapping_add((idx as u32 >> 31) as i32);
        let m: f32 = lf_checker_rt::callee_thiscall!(MEASURE, f32, table, idx as u32);
        wr32(esi + 0x8c, m.to_bits());
        let _: u32 = lf_checker_rt::callee_thiscall!(FILL, u32, table, entry, esi.wrapping_add(0xd0));
        let sx = f32::from_bits(rd32(esi + 0x30));
        let sy = f32::from_bits(rd32(esi + 0x34));
        let sz = f32::from_bits(rd32(esi + 0x38));
        let spread2 = fadd(fadd(fmul(sx, sx), fmul(sy, sy)), fmul(sz, sz));
        let extent = if spread2 > 0.25 {
            1.0f32
        } else {
            f32::from_bits(rd32(esi + 0x40))
        };
        let cx = f32::from_bits(rd32(esi + 0x20));
        let cy = f32::from_bits(rd32(esi + 0x24));
        let cz = f32::from_bits(rd32(esi + 0x28));
        let boxw = [
            cvtt16(fmul(fsub(cx, extent), 8.0)),
            cvtt16(fmul(fadd(cx, extent), 8.0)),
            cvtt16(fmul(fsub(cy, extent), 8.0)),
            cvtt16(fmul(fadd(cy, extent), 8.0)),
            cvtt16(fmul(fsub(cz, extent), 8.0)),
            cvtt16(fmul(fadd(cz, extent), 8.0)),
        ];
        let mut packed = [0u32; 3];
        for k in 0..3usize {
            packed[k] = boxw[2 * k] as u32 | ((boxw[2 * k + 1] as u32) << 16);
        }
        let r4: u32 = lf_checker_rt::callee_thiscall!(
            REPORT, u32, this, packed.as_ptr() as u32, esi, table, entry);
        (r4 & 0xffffff00) | 1
    }
});
