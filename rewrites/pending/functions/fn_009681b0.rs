// original: 0x009681B0 check_launch_gate
/// Run the chain of launch-gate checks; return 1 only if all pass.
///
/// Fetches the context object (callee 1); gives up when it is null, when
/// bit 2 of its flag byte at `+0x26C` is clear, or when its sub-object
/// at `+0xB30` is null. The sub-object's signed word at `+0x2E` goes to
/// the range check (callee 2, cdecl) beside the constant 0x18; a zero
/// answer fails. Then the veto check (callee 3) runs on the context: a
/// nonzero answer fails. Finally the two enable checks (callee 4, twice)
/// run on the object at context `+0x224` offset by `+0x2E0` with codes
/// 0x2DE and 0x2E2; a nonzero answer from either fails. Returns 1 in
/// `al` when every check passes, else 0.
///
/// Original: 0x009681B0 (cdecl, no stack words).

export!(cdecl, rw_009681B0() -> u32 {
    unsafe {
        let ctx = callee_cdecl!(1, u32,);
        if ctx == 0 {
            return 0;
        }
        if (ctx.wrapping_add(0x26C) as *const u8).read() & 4 == 0 {
            return 0;
        }
        let sub = (ctx.wrapping_add(0xB30) as *const u32).read_unaligned();
        if sub == 0 {
            return 0;
        }
        let code = (sub.wrapping_add(0x2E) as *const u16).read_unaligned() as i16 as i32 as u32;
        let range_ok: u32 = callee_cdecl!(2, u32, code, 0x18);
        if range_ok as u8 == 0 {
            return 0;
        }
        let edi = (ctx.wrapping_add(0x224) as *const u32).read_unaligned();
        let veto: u32 = callee_thiscall!(3, u32, ctx);
        if veto as u8 != 0 {
            return 0;
        }
        let first: u32 = callee_thiscall!(4, u32, edi.wrapping_add(0x2E0), 0x2DE, 0);
        if first as u8 != 0 {
            return 0;
        }
        let second: u32 = callee_thiscall!(4, u32, edi.wrapping_add(0x2E0), 0x2E2, 0);
        if second as u8 != 0 {
            return 0;
        }
        1
    }
});
