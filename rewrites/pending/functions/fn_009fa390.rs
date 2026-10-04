// original: 0x009fa390 playstat_dual_stage_publish
/// Publish a stat through two fixed-kind datum stages.
///
/// Runs the shared publish chain, then emits two datum pairs of the same
/// kind back to back. Any failed check aborts with 0, else returns 1.
export!(cdecl, rw_009fa390(obj: u32, stat: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x20;
        if callee_cdecl!(1, u32, obj, stat) & 0xFF == 0 {
            return 0;
        }
        let base = obj as *const u32;
        let fields = stat as *const u32;
        let total = base.add(1).read().wrapping_add(base.add(3).read());
        if callee_thiscall!(2, u32, obj, KIND) & 0xFF == 0 {
            return 0;
        }
        callee_cdecl!(3, u32, base.read(), fields.add(0x34 / 4).read(), KIND, total);
        callee_thiscall!(4, u32, obj, KIND);
        let second = fields.add(0x38 / 4).read();
        if callee_thiscall!(2, u32, obj, KIND) & 0xFF == 0 {
            return 0;
        }
        callee_cdecl!(3, u32, base.read(), second, KIND, total);
        callee_thiscall!(4, u32, obj, KIND);
        1
    }
});
