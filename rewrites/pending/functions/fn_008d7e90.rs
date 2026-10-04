// original: 0x008D7E90 probe_batch_builder

const BATCH_RECORDS: usize = 64;
const RECORD_LEN: usize = 0x60;

/// Probe-batch builder: fill 64 records, run two helpers, test the flag.
pub fn build_and_test(a: u32, f: f32, b: u32) -> u32 {
    unsafe {
        let tick = callee_cdecl!(0, u32,);
        let g_hi = global::<f32>(0x01B4B328).read();
        let g_mid = global::<f32>(0x01B4B324).read();
        let g_lo = global::<f32>(0x01B4B320).read();
        let mut rec = [0u8; BATCH_RECORDS * RECORD_LEN];
        let base = rec.as_mut_ptr();
        for r in 0..BATCH_RECORDS {
            let o = r * RECORD_LEN;
            let w32 = |off: usize, v: u32| {
                (base.add(o + off) as *mut u32).write(v);
            };
            w32(0x10, g_lo.to_bits());
            w32(0x14, g_mid.to_bits());
            w32(0x18, g_hi.to_bits());
            w32(0x20, g_lo.to_bits());
            w32(0x24, g_mid.to_bits());
            w32(0x28, g_hi.to_bits());
            w32(0x30, g_lo.to_bits());
            w32(0x34, g_mid.to_bits());
            w32(0x38, g_hi.to_bits());
            w32(0x4c, 0xffff);
        }
        // Two words: the flag plus the zero word the snapshot reads after it.
        let mut flag = [0x40u32, 0u32];
        let rp = rec.as_mut_ptr() as u32;
        let fp = flag.as_mut_ptr() as u32;
        callee_cdecl!(1, u32, a, f.to_bits(), b, tick, rp, fp);
        callee_cdecl!(2, u32, rp, fp);
        if (flag[0] as i32) > 0 {
            1
        } else {
            0
        }
    }
}
