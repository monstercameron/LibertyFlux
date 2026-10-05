// original: 0x00ccfa00 melee_event_fire
/// Fire the melee event for the object, returning 1 on success, else 0.
///
/// Resolves the event source (`[arg+0xa80]+0x3c`) and runs a chain of
/// gated helpers: the pair check (cdecl, arg and source) must answer
/// non-zero, then the source check (cdecl) must answer non-zero or the
/// fallback check (cdecl) must, then the id lookup (thiscall on the fixed
/// manager address, source and the answer's word at `+4`) must not answer
/// -1, and finally the event sink runs (thiscall on `[arg+0x78]`, the id,
/// 0x52, 12.0, -1). Any failure returns 0 in AL. The single stack argument
/// is the object; ECX is unused. Cdecl-shaped thiscall.
export!(thiscall, rw_00ccfa00(_this: u32, obj: u32) -> u32 {
    unsafe {
        const SRC_PTR_OFF: u32 = 0xa80;
        const SRC_OFF: u32 = 0x3c;
        const SINK_OBJ_OFF: u32 = 0x78;
        const MGR_C: u32 = 0x0171c968;
        const KIND: u32 = 0x52;
        const TWELVE: u32 = 0x41400000;
        let w = (obj.wrapping_add(SRC_PTR_OFF) as *const u32).read_unaligned();
        let src = (w.wrapping_add(SRC_OFF) as *const u32).read_unaligned();
        let ok: u32 = callee_cdecl!(1, u32, obj, src);
        if ok & 0xff == 0 {
            return 0;
        }
        let h: u32 = callee_cdecl!(2, u32, src);
        let h = if h != 0 {
            h
        } else {
            let f: u32 = callee_cdecl!(3, u32, src);
            if f == 0 {
                return 0;
            }
            f
        };
        let w4 = (h.wrapping_add(4) as *const u32).read_unaligned();
        let id: u32 = callee_thiscall!(4, u32, relocated(MGR_C), src, w4);
        if id == 0xffff_ffff {
            return 0;
        }
        let sink = (obj.wrapping_add(SINK_OBJ_OFF) as *const u32).read_unaligned();
        let _: u32 = callee_thiscall!(5, u32, sink, id, KIND, TWELVE, 0xffff_ffff);
        1
    }
});
