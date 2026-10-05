// original: 0x00A4E0B0 CVehicle::vf76

/// Virtual method 76: gates on a virtual check, then marks and reports.
///
/// Calls the gate through virtual slot `GATE_SLOT` (0x12C) with `this` in
/// `ecx` and no stack words, loading the target through the object's vtable
/// exactly like the original. A zero low byte returns 0 at once. Otherwise:
/// when the argument byte is non-zero, snapshots the global at `RATE` into
/// `this + SNAP` (0x12A4); sets bit 4 of the flag byte at `this + FLAG`
/// (0x0F1D); clears bit 3 of the dword at `this + FLAGS2` (0x24); reports
/// through the second callee (cdecl, `(this, 0)`); and returns 1.
///
/// Original: 0x00A4E0B0 (thiscall, one byte-ish stack word), two callees.
lf_checker_rt::export!(thiscall, rw_00A4E0B0(this: u32, arg: u32) -> u32 {
    unsafe {
        const GATE_SLOT: u32 = 0x12C;
        const SNAP: u32 = 0x12A4;
        const FLAG: u32 = 0x0F1D;
        const FLAGS2: u32 = 0x24;
        const RATE: u32 = 0x011735B4;
        const FLAG_BIT: u8 = 0x10;
        const FLAGS2_KEEP: u32 = 0xFFFF_FFF7;
        const GATE_CALLEE: u32 = 1;
        const REPORT_CALLEE: u32 = 2;
        let vtable = ((this as *const u32).read_unaligned() + GATE_SLOT)
            as *const u32;
        let gate: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(vtable.read_unaligned() as usize);
        if gate(this) & 0xFF == 0 {
            return 0;
        }
        if (arg & 0xFF) != 0 {
            let g = lf_checker_rt::global::<u32>(RATE).read_unaligned();
            ((this + SNAP) as *mut u32).write_unaligned(g);
        }
        let f = (this + FLAG) as *mut u8;
        f.write(f.read() | FLAG_BIT);
        let f2 = (this + FLAGS2) as *mut u32;
        f2.write(f2.read_unaligned() & FLAGS2_KEEP);
        lf_checker_rt::callee_cdecl!(REPORT_CALLEE, u32, this, 0);
        1
    }
});
