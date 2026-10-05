// original: 0x00873870 crmt_blend_weight_apply

/// Apply per-slot blend weights: run the 0x872c30 preparation, ask the object for its slot count through vtable slot +0x24, and for each slot widen the two f32 values at +0x20/+0x24 of the slot to f64 and pass them with the slot index to the 0x873620 applier (with the data-table address 0xfc75a0). Returns the slot count. (True size 116 bytes; the batch lists 110, which omits the epilogue.)
///
/// Original: 0x00873870 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00873870(this: u32, arg: u32) -> u32 {
    const PREP: u32 = 1;
    const COUNT_SLOT: u32 = 0x24;
    const APPLY: u32 = 3;
    const APPLY_TABLE: u32 = 0x00fc75a0;
    unsafe {
        let _: u32 = lf_checker_rt::callee_thiscall!(PREP, u32, this, arg);
        let vt = (this as *const u32).read_unaligned();
        let tgt = ((vt as *const u8).add(COUNT_SLOT as usize) as *const u32)
            .read_unaligned();
        let count_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let mut n = count_of(this);
        if n == 0 {
            return 0;
        }
        let mut idx = 0u32;
        loop {
            let f1 = ((this + 0x20 + idx * 8) as *const u32).read_unaligned();
            let f2 = ((this + 0x20 + idx * 8 + 4) as *const u32).read_unaligned();
            let d1 = (f32::from_bits(f1) as f64).to_bits();
            let d2 = (f32::from_bits(f2) as f64).to_bits();
            let slot = idx;
            let _: u32 = lf_checker_rt::callee_cdecl!(APPLY, u32, arg,
                lf_checker_rt::relocated(APPLY_TABLE), slot,
                d1 as u32, (d1 >> 32) as u32, d2 as u32, (d2 >> 32) as u32);
            idx += 1;
            n = count_of(this);
            if idx >= n {
                break;
            }
        }
        n
    }
});
