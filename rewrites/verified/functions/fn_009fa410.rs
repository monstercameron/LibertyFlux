// original: 0x009fa410 playstat_stat_publish_slot
/// Publish a stat, then push one trailing slot through the gated sink.
///
/// Runs the shared publish chain, emits one fixed-kind datum pair, and
/// hands the stat's trailing slot to the gated sink. Any failed check
/// aborts with 0, else returns 1.
export!(cdecl, rw_009fa410(obj: u32, stat: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x20;
        if callee_cdecl!(1, u32, obj, stat) & 0xFF == 0 {
            return 0;
        }
        let base = obj as *const u32;
        let datum = (stat as *const u32).add(0x34 / 4).read();
        if callee_thiscall!(2, u32, obj, KIND) & 0xFF == 0 {
            return 0;
        }
        let total = base.add(3).read().wrapping_add(base.add(1).read());
        callee_cdecl!(3, u32, base.read(), datum, KIND, total);
        callee_thiscall!(4, u32, obj, KIND);
        u32::from(callee_thiscall!(5, u32, obj, stat.wrapping_add(0x38), KIND) & 0xFF != 0)
    }
});
