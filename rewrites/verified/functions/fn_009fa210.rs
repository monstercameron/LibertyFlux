// original: 0x009fa210 playstat_stat_publish_chain
/// Publish a stat through three gated stages, then dispatch on its index.
///
/// Each stage checks a datum kind, emits the object's summed counter with
/// the stat's datum word, and acknowledges it; any failed check aborts with
/// 0. The trailing index selects between an immediate success and one final
/// gated call whose low byte becomes the result.
export!(cdecl, rw_009fa210(obj: u32, stat: u32) -> u32 {
    unsafe {
        const STAGES: [(u32, usize); 3] = [(3, 4), (6, 8), (0x20, 0x0c)];
        const MAX_INDEX: u32 = 0x2e;
        // One entry per index 0..=0x2e: 1 takes the final gated call.
        const DISPATCH: [u8; 47] = [
            0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1,
            1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0,
        ];
        let base = obj as *const u32;
        for (kind, off) in STAGES {
            if callee_thiscall!(1, u32, obj, kind) & 0xFF == 0 {
                return 0;
            }
            let total = base.add(1).read().wrapping_add(base.add(3).read());
            let datum = (stat as *const u8).add(off).cast::<u32>().read();
            callee_cdecl!(2, u32, base.read(), datum, kind, total);
            callee_thiscall!(3, u32, obj, kind);
        }
        let index = (stat as *const u32).add(2).read();
        if index > MAX_INDEX {
            return 0;
        }
        if DISPATCH[index as usize] == 0 {
            return 1;
        }
        callee_thiscall!(4, u32, obj, stat.wrapping_add(0x10), 0x20) & 0xFF
    }
});
