// original: 0x00AC6150 shader_fx_maybe_bind (proposed)

/// Bind the effect's resource unless the probe callee accepts it.
///
/// The original calls the probe callee on `this` and returns when its answer
/// is nonzero (thiscall, two stack words: `target`, `slot`). Otherwise it
/// calls the bind callee with object `[target + 8]`, passing the word at
/// `this + 0x18` and `slot`. No value is returned.
lf_checker_rt::export!(thiscall, rw_00AC6150(this: u32, target: u32, slot: u32) -> u32 {
    unsafe {
        const PROBE: u32 = 1;
        const BIND: u32 = 2;
        const HANDLE: u32 = 0x18;
        const OBJ_OFF: u32 = 8;
        let ok = lf_checker_rt::callee_thiscall!(PROBE, u32, this);
        if ok != 0 {
            return 0;
        }
        let obj = (target.wrapping_add(OBJ_OFF) as *const u32).read_unaligned();
        let handle = (this.wrapping_add(HANDLE) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(BIND, u32, obj, handle, slot);
        0
    }
});
