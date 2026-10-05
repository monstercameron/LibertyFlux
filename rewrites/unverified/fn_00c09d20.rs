// original: 0x00c09d20 stream_table_init (proposed)

/// Initialise the streaming table: slots, pointer maps and row buffer.
///
/// `this` points to the table. Each of the `NSLOT` slots (stride `SLOT`,
/// first at `SLOTS`) has its header words cleared and goes to the slot helper
/// (callee 1) with the owner bias in `ecx`. The two pointer maps (each `NMAP`
/// entries, at `MAP0` and `MAP1`) are filled with strided pointers into the
/// table itself. A row buffer of `ROWBUF` bytes comes from the allocator
/// (callee 2), the capacity fields at `CAPW` and `CAPD` are set to `NSLOT`,
/// and the allocator's answer comes back with its low byte forced to 1.
///
/// Original: 0x00c09d20 (thiscall, no stack words; 1 is thiscall, 2 cdecl).
lf_checker_rt::export!(thiscall, rw_00c09d20(this: u32) -> u32 {
    unsafe {
        const NSLOT: u32 = 0x200;
        const SLOT: u32 = 0x18;
        const SLOTS: u32 = 0x14;
        const OWNER_BIAS: u32 = 0x3008;
        const NMAP: u32 = 0x40;
        const MAP0: u32 = 0x3820;
        const MAP1: u32 = 0x5120;
        const MAP0_SRC: u32 = 0x3020;
        const MAP0_STEP: u32 = 0x20;
        const MAP1_SRC: u32 = 0x3920;
        const MAP1_STEP: u32 = 0x60;
        const ROWBUF: u32 = 0x4000;
        const ROWS: u32 = 0x5224;
        const CAPW: u32 = 0x522a;
        const CAPD: u32 = 0x5220;
        const SLOT_HELPER: u32 = 1;
        const MALLOC: u32 = 2;
        let mut k = 0u32;
        while k < NSLOT {
            let s = this.wrapping_add(SLOTS).wrapping_add(k.wrapping_mul(SLOT));
            (s.wrapping_sub(0x0c) as *mut u32).write_unaligned(0);
            (s.wrapping_sub(4) as *mut u32).write_unaligned(0);
            (s as *mut u32).write_unaligned(0);
            (s.wrapping_add(4) as *mut u32).write_unaligned(0);
            (s.wrapping_add(8) as *mut u8).write(0);
            let _r: u32 = lf_checker_rt::callee_thiscall!(
                SLOT_HELPER, u32, this.wrapping_add(OWNER_BIAS), s.wrapping_sub(0x0c));
            k += 1;
        }
        (this as *mut u32).write_unaligned(0);
        (this.wrapping_add(4) as *mut u32).write_unaligned(0);
        let mut m = 0u32;
        while m < NMAP {
            (this.wrapping_add(MAP0).wrapping_add(m.wrapping_mul(4)) as *mut u32)
                .write_unaligned(this.wrapping_add(MAP0_SRC).wrapping_add(m.wrapping_mul(MAP0_STEP)));
            (this.wrapping_add(MAP1).wrapping_add(m.wrapping_mul(4)) as *mut u32)
                .write_unaligned(this.wrapping_add(MAP1_SRC).wrapping_add(m.wrapping_mul(MAP1_STEP)));
            m += 1;
        }
        let r: u32 = lf_checker_rt::callee_cdecl!(MALLOC, u32, ROWBUF);
        (this.wrapping_add(ROWS) as *mut u32).write_unaligned(r);
        (this.wrapping_add(CAPW) as *mut u16).write_unaligned(NSLOT as u16);
        (this.wrapping_add(CAPD) as *mut u32).write_unaligned(NSLOT);
        (r & 0xffff_ff00) | 1
    }
});
