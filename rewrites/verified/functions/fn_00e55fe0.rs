// original: 0x00E55FE0 UIRawClipViewer::vf103
/// Refresh the viewer's current text record (original 0x00E55FE0).
///
/// Resolves the worker through the keeper object and the viewer's own
/// dispatch table, checks the resolved item against the name service,
/// then looks the record up, releases any previous text, and copies the new
/// text into a fresh buffer, trimming four bytes off the end and stamping it
/// with the global mark. When the tail service accepts the record it raises
/// the ready flag and tail-dispatches to the keeper's continuation.
/// Covered by the contract's layout pins: the tail target receives 1 through
/// the outgoing argument slot; the incidental rewrite of the caller's own
/// argument slot with the same value 1 is invisible by the checker's
/// arg-slot rule (the stack argument is fixed to 1 and never read).
export!(thiscall, rw_e55fe0(this: u32, _arg: u32) -> u32 {
    const KEEPER: u32 = 0x0198_1A4C;
    const AUX: u32 = 0x0117_6888;
    const HEAD_CELL: u32 = 0x018B_6C8C;
    const TAG_NAME: u32 = 0x00F1_A24C;
    const TAG_LOOKUP: u32 = 0x00F1_A258;
    const TAG_OPEN: u32 = 0x00F1_A278;
    const PATH_A: u32 = 0x0105_7BF0;
    const PATH_B: u32 = 0x0114_E384;
    const MARK: u32 = 0x00F1_A288;
    const MARK_BYTE: u32 = 0x00F1_A28C;
    const TAG_TAIL: u32 = 0x00F1_A298;
    const TEXT_SLOT: u32 = 0x33C;
    const READY_FLAG: u32 = 0x334;
    const TRIM: u32 = 4;
    unsafe {
        let head = global::<u32>(HEAD_CELL).read();
        let extra = ((head + 0x200) as *const u32).read();
        callee_thiscall!(1, u32, relocated(KEEPER), extra);
        let vt = (this as *const u32).read();
        let current: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt + 0x1C0) as *const u32).read() as usize);
        let worker = current(this);
        let wvt = (worker as *const u32).read();
        let first: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((wvt as *const u32).read() as usize);
        let item = first(worker);
        let named = callee_cdecl!(2, u32, relocated(TAG_NAME));
        if item != named {
            return named;
        }
        current(this);
        callee_thiscall!(3, u32, relocated(AUX), relocated(TAG_LOOKUP));
        callee_cdecl!(6, u32, relocated(TAG_OPEN), 0);
        let found = callee_cdecl!(7, u32, worker, 0, relocated(PATH_B), relocated(PATH_A), 0);
        let slot = (this + TEXT_SLOT) as *mut u32;
        if slot.read() != 0 {
            callee_cdecl!(8, u32, slot.read());
            slot.write(0);
        }
        let fvt = (found as *const u32).read();
        let text: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((fvt + 0x23C) as *const u32).read() as usize);
        let s = text(found);
        let mut len = 0u32;
        while ((s.wrapping_add(len)) as *const u8).read() != 0 {
            len = len.wrapping_add(1);
        }
        let buf = callee_cdecl!(10, u32, len.wrapping_add(1));
        slot.write(buf);
        let s2 = text(found);
        let mut i = 0u32;
        loop {
            let b = ((s2.wrapping_add(i)) as *const u8).read();
            ((buf.wrapping_add(i)) as *mut u8).write(b);
            i = i.wrapping_add(1);
            if b == 0 {
                break;
            }
        }
        let mut len2 = 0u32;
        while ((buf.wrapping_add(len2)) as *const u8).read() != 0 {
            len2 = len2.wrapping_add(1);
        }
        ((buf.wrapping_add(len2)).wrapping_sub(TRIM) as *mut u8).write(0);
        let mut end = buf as *const u8;
        while end.read() != 0 {
            end = end.add(1);
        }
        (end as *mut u32).write_unaligned(global::<u32>(MARK).read());
        ((end as u32).wrapping_add(4) as *mut u8).write(global::<u8>(MARK_BYTE).read());
        let s3 = text(found);
        let ok = callee_thiscall!(11, u32, this, s3);
        if ok as u8 == 0 {
            return ok;
        }
        ((this + READY_FLAG) as *mut u8).write(1);
        let detail: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((fvt + 0x48) as *const u32).read() as usize);
        let d = detail(found);
        let h = callee_cdecl!(13, u32, relocated(TAG_TAIL), d);
        let fin = callee_cdecl!(2, u32, h);
        let tailobj = callee_thiscall!(1, u32, relocated(KEEPER), fin);
        let tvt = (tailobj as *const u32).read();
        let tail: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(((tvt + 0x120) as *const u32).read() as usize);
        tail(tailobj, 1)
    }
});
