// original: 0x008b8250 meter_axis_value
/// Meter-driven axis value lookup.
///
/// Samples the meter for handle-table slot `index`. A sample of -0x5c passes
/// through unchanged; a negative sample yields 0; otherwise the sample is
/// validated against the live group's entry count and the signed 16-bit code
/// at offset 0x12 is returned, selecting the entry by the sample when `sel`
/// is -1, by `sel` when it is a valid slot, and defaulting to the first entry
/// for any other negative or over-range `sel`.
export!(cdecl, rw_008b8250(index: u32, sel: u32) -> u32 {
    unsafe {
        const HANDLES: u32 = 0x01160C0C;
        const LIVE_GROUP: u32 = 0x01160C40;
        const COUNT_TABLE: u32 = 0x019D33A4;
        const BASE_TABLE: u32 = 0x019D33A0;
        const ENTRY_LEN: u32 = 0x16;
        const CODE_OFF: u32 = 0x12;
        const ABSENT: u32 = 0xFFFF_FFA4;
        let addr = relocated(HANDLES).wrapping_add(index.wrapping_mul(4));
        let handle = (addr as *const u32).read();
        let sample: u32 = callee_cdecl!(2, u32, handle);
        if sample == ABSENT {
            return sample;
        }
        if (sample as i32) < 0 {
            return 0;
        }
        let group = (relocated(LIVE_GROUP) as *const u32).read();
        let row = group.wrapping_mul(3);
        let count_addr = relocated(COUNT_TABLE).wrapping_add(row.wrapping_mul(8));
        let count = (count_addr as *const u16).read() as u32;
        if (sample as i32) > (count as i32) {
            return 0;
        }
        if (count as u16) == 0 {
            return 0;
        }
        let base_addr = relocated(BASE_TABLE).wrapping_add(row.wrapping_mul(8));
        let base = (base_addr as *const u32).read();
        if sel == 0xFFFF_FFFF {
            let code_addr = base
                .wrapping_add(sample.wrapping_mul(ENTRY_LEN))
                .wrapping_add(CODE_OFF);
            return (code_addr as *const i16).read() as i32 as u32;
        }
        if (sel as i32) < 0 || (sel as i32) >= (count as i32) {
            let code_addr = base.wrapping_add(CODE_OFF);
            return (code_addr as *const i16).read() as i32 as u32;
        }
        let code_addr = base
            .wrapping_add(sel.wrapping_mul(ENTRY_LEN))
            .wrapping_add(CODE_OFF);
        (code_addr as *const i16).read() as i32 as u32
    }
});
