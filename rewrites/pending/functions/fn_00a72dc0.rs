// original: 0x00a72dc0 table_gate_then_probe (proposed)
/// True (1) when the indexed table entry, the detail probe and the flag
/// compare all agree, else 0.
///
/// `cdecl`, one stack word (the entity). Reads the table index from its
/// global: -1, a null slot, or a slot whose word at `+0x4c8` is non-zero
/// exits early, and so does a non-zero kill-switch global after the probe.
/// The early exits only clear the low byte, so their upper bytes keep the
/// working value (`index`, the slot pointer, or the probe answer); the
/// rewrite reproduces those deterministic upper bytes exactly. Otherwise
/// resolves the detail (callee 1, thiscall on `entity+0x2b0`) and
/// remembers `kind = [entity+0x2b0]`; a live detail probes the key at
/// `detail+0x18` (callee 2) and takes the word at `+4` as `mode` (0 when
/// the detail is null). Then: `mode` above 1 returns 1, else the result is
/// `kind == 8`, both as full exact words.
lf_checker_rt::export!(cdecl, rw_00a72dc0(ent: u32) -> u32 {
    unsafe {
        const INDEX_G: u32 = 0x01036f14;
        const TABLE: u32 = 0x011a8808;
        const KILL: u32 = 0x01160e78;
        const SLOT_USED: u32 = 0x4c8;
        const KIND_OFF: u32 = 0x2b0;
        const KEY_OFF: u32 = 0x18;
        const MODE_OFF: u32 = 0x4;
        const KIND_WANT: u32 = 8;
        const LOW_CLEAR: u32 = 0xffff_ff00;
        let idx = (lf_checker_rt::global::<u32>(INDEX_G)).read_unaligned();
        if idx == 0xffff_ffff {
            return idx & LOW_CLEAR;
        }
        let slot = ((lf_checker_rt::relocated(TABLE) + idx.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        if slot == 0 {
            return 0;
        }
        if ((slot + SLOT_USED) as *const u32).read_unaligned() != 0 {
            return slot & LOW_CLEAR;
        }
        let det = lf_checker_rt::callee_thiscall!(1, u32, ent.wrapping_add(KIND_OFF));
        let kind = ((ent + KIND_OFF) as *const u32).read_unaligned();
        let mut mode = 0u32;
        let mut last = det;
        if det != 0 {
            let key = ((det + KEY_OFF) as *const u32).read_unaligned();
            let r = lf_checker_rt::callee_cdecl!(2, u32, key);
            mode = ((r + MODE_OFF) as *const u32).read_unaligned();
            last = r;
        }
        if (lf_checker_rt::global::<u32>(KILL)).read_unaligned() != 0 {
            return last & LOW_CLEAR;
        }
        if mode != 0 && mode != 1 {
            return 1;
        }
        (kind == KIND_WANT) as u32
    }
});
