// original: 0x00a90ca0 stream_init_entry_full (proposed)

/// Initialise a streaming entry: lookups, conditional virtual closes, stamps.
///
/// The word at `this+0x2e` (signed) indexes a global pointer table; the
/// pointed object's word at `+0x52` (signed) and 0 go to the first lookup
/// (callee 1, cdecl/2). Then a measure runs (callee 2, thiscall/3 with
/// 0, 0, 1) and a release (callee 3, cdecl/1 with 0). When the dword at
/// `this+0x38` is null the entry's slot-0xa8 virtual runs (callee 4,
/// thiscall/0 on the entry); when it is set and its word at `+8` equals
/// 0xffff the slot-0xac virtual runs instead (callee 5, thiscall/0).
/// Finally `this+0x78` is set to 1, `this+0x108` to 0xff and `this+0x104`
/// to 0.
///
/// Returns the slot-0xac answer when it runs, else the `+0x38` dword.
/// Thiscall: object in ecx, no stack words.
lf_checker_rt::export!(thiscall, rw_00a90ca0(this: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 0x2e;
        const LINK_OFF: u32 = 0x38;
        const KIND_TABLE: u32 = 0x01295cd8;
        const INNER_OFF: u32 = 0x52;
        const READY_OFF: u32 = 8;
        const READY_VAL: u16 = 0xffff;
        const STAMP1_OFF: u32 = 0x78;
        const STAMP2_OFF: u32 = 0x108;
        const CLEAR_OFF: u32 = 0x104;
        const SLOT_CLOSE: u32 = 0xa8;
        const SLOT_READY: u32 = 0xac;
        let kind = ((this + KIND_OFF) as *const i16).read_unaligned() as i32 as u32;
        let obj = ((lf_checker_rt::relocated(KIND_TABLE) + kind.wrapping_mul(4))
            as *const u32)
            .read_unaligned();
        let inner =
            ((obj + INNER_OFF) as *const i16).read_unaligned() as i32 as u32;
        lf_checker_rt::callee_cdecl!(1, u32, inner, 0);
        lf_checker_rt::callee_thiscall!(2, u32, this, 0, 0, 1);
        lf_checker_rt::callee_cdecl!(3, u32, 0);
        if ((this + LINK_OFF) as *const u32).read_unaligned() == 0 {
            let vt = (this as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(((vt + SLOT_CLOSE) as *const u32).read_unaligned()
                    as usize);
            f(this);
        }
        let link = ((this + LINK_OFF) as *const u32).read_unaligned();
        let mut ret = link;
        if link != 0
            && ((link + READY_OFF) as *const u16).read_unaligned() == READY_VAL
        {
            let vt = (this as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(((vt + SLOT_READY) as *const u32).read_unaligned()
                    as usize);
            ret = f(this);
        }
        ((this + STAMP1_OFF) as *mut u8).write(1);
        ((this + STAMP2_OFF) as *mut u8).write(0xff);
        ((this + CLEAR_OFF) as *mut u32).write_unaligned(0);
        ret
    }
});
