// original: 0x00a494e0 vehicle_transform_fill
/// Fill a 4-word transform from the row's kind through the worker callee.
///
/// Maps the index (5->0, 6->1, 7->2, anything else 3), then for kind 0x54
/// with flag bit 5 clear remaps 1->0 and 3->2. The callee (cdecl/5) runs on
/// `(scratch, kind, slot, this, extra)` and answers a struct pointer; the
/// first three words are scaled by -1.0 into `out`, the fourth copied, and
/// the fourth is also the return value (thiscall, three stack arguments).
/// The scratch-stack argument is skipped in the call comparison.
export!(thiscall, rw_00a494e0(this: u32, out: u32, index: u32, extra: u32) -> u32 {
    unsafe {
        const MODEL_INDEX_OFF: u32 = 0x2e;
        const MODEL_TABLE: u32 = 0x01295cd8;
        const SCALE_ADDR: u32 = 0x00fe8d94;
        let mut slot = match index.wrapping_sub(5) {
            0 => 0u32,
            1 => 1,
            2 => 2,
            _ => 3,
        };
        let model =
            ((this.wrapping_add(MODEL_INDEX_OFF)) as *const i16).read_unaligned() as i32 as u32;
        let row = (relocated(MODEL_TABLE).wrapping_add(model.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let kind = (row.wrapping_add(0xc4) as *const u32).read_unaligned();
        if kind == 0x54 {
            let flags = (row.wrapping_add(0x94) as *const u32).read_unaligned();
            if flags >> 5 & 1 == 0 {
                if slot == 1 {
                    slot = 0;
                } else if slot == 3 {
                    slot = 2;
                }
            }
        }
        let mut scratch = [0u32; 5];
        let ans: u32 = callee_cdecl!(
            1,
            u32,
            (&mut scratch as *mut u32) as u32,
            kind,
            slot,
            this,
            extra
        );
        let k = f32::from_bits((relocated(SCALE_ADDR) as *const u32).read_unaligned());
        let v0 = f32::from_bits((ans as *const u32).read_unaligned());
        let v2 = f32::from_bits((ans.wrapping_add(4) as *const u32).read_unaligned());
        let v3 = f32::from_bits((ans.wrapping_add(8) as *const u32).read_unaligned());
        let w = (ans.wrapping_add(0xc) as *const u32).read_unaligned();
        (out as *mut u32)
            .write_unaligned((core::hint::black_box(v0) * core::hint::black_box(k)).to_bits());
        (out.wrapping_add(4) as *mut u32)
            .write_unaligned((core::hint::black_box(v2) * core::hint::black_box(k)).to_bits());
        (out.wrapping_add(8) as *mut u32)
            .write_unaligned((core::hint::black_box(v3) * core::hint::black_box(k)).to_bits());
        (out.wrapping_add(0xc) as *mut u32).write_unaligned(w);
        w
    }
});
