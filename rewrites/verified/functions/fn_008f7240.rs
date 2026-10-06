// original: 0x008F7240 Input_PollDevices

/// Poll the device list: when the gate answers nonzero, walk the slots from
/// `SLOT_FIRST` while the index is below both the reported count (unsigned)
/// and `0x2F0`, and for each live item report the probe byte through its
/// `+0x78` slot, then, when the probe confirms, its parameter dword through
/// `+0x64`, its four flag bytes through `+0x6C`, and release it through
/// `+0x38`. All indirect targets are read through the item's table exactly
/// like the original. Returns the last callee answer. Convention: cdecl, no
/// stack words.
lf_checker_rt::export!(cdecl, rw_008f7240() -> u32 {
    unsafe {
        const SLOT_FIRST: u32 = 0x0118D470;
        const SLOT_STRIDE: u32 = 0xBC;
        const GATE: u32 = 1;
        const COUNT: u32 = 2;
        const GET: u32 = 3;
        const REPORT: u32 = 4;
        const PARAM: u32 = 5;
        const FLAG: u32 = 6;
        const RELEASE: u32 = 7;
        const PROBE: u32 = 8;
        const LIST: u32 = 0x019F25E8;
        const SPAN: u32 = 0x2F0;
        const VT_REPORT: u32 = 0x78;
        const VT_PARAM: u32 = 0x64;
        const VT_FLAG: u32 = 0x6C;
        const VT_RELEASE: u32 = 0x38;
        let list = lf_checker_rt::relocated(LIST);
        let mut r: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, list);
        if r == 0 {
            return r;
        }
        let mut idx: u32 = 0;
        let mut slot = lf_checker_rt::relocated(SLOT_FIRST);
        let mut dist: u32 = 0;
        loop {
            r = lf_checker_rt::callee_thiscall!(COUNT, u32, list);
            if idx >= r {
                break;
            }
            r = lf_checker_rt::callee_thiscall!(GET, u32, list, idx);
            let item = r;
            if item != 0 {
                let table = (item as *const u32).read_unaligned();
                r = lf_checker_rt::callee_thiscall!(PROBE, u32, slot);
                let probe_byte = r & 0xFF;
                let report: extern "stdcall" fn(u32, u32) -> u32 =
                    core::mem::transmute(
                        (table.wrapping_add(VT_REPORT) as *const u32).read_unaligned()
                            as usize,
                    );
                r = report(item, probe_byte);
                r = lf_checker_rt::callee_thiscall!(PROBE, u32, slot);
                if r & 0xFF != 0 {
                    let target = (slot as *const u32).read_unaligned();
                    let param: extern "stdcall" fn(u32, u32) -> u32 =
                        core::mem::transmute(
                            (table.wrapping_add(VT_PARAM) as *const u32).read_unaligned()
                                as usize,
                        );
                    let dword = (target.wrapping_add(4) as *const u32).read_unaligned();
                    r = param(item, dword);
                    let flag: extern "stdcall" fn(u32, u32, u32) -> u32 =
                        core::mem::transmute(
                            (table.wrapping_add(VT_FLAG) as *const u32).read_unaligned()
                                as usize,
                        );
                    let mut k: u32 = 0;
                    while k < 4 {
                        let b = (target.wrapping_add(0x0C + k) as *const u8).read();
                        r = flag(item, k, b as u32);
                        k += 1;
                    }
                    let release: extern "stdcall" fn(u32) -> u32 =
                        core::mem::transmute(
                            (table.wrapping_add(VT_RELEASE) as *const u32)
                                .read_unaligned() as usize,
                        );
                    r = release(item);
                }
            }
            dist = dist.wrapping_add(SLOT_STRIDE);
            idx += 1;
            slot = slot.wrapping_add(SLOT_STRIDE);
            if dist >= SPAN {
                break;
            }
        }
        r
    }
});
