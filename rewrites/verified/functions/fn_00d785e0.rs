// original: 0x00D785E0 set_flag_when_mode_and_thresholds_pass (proposed)

/// Set `*out` to 1 when the mode and two float thresholds all pass.
///
/// `obj` points to an object with a mode word at `+0x1304`; `v0` and `v1`
/// are floats, `out` a byte pointer. Stores 1 to `*out` only when the mode
/// is 2, `v0` is strictly above 2.0 and `|v1|` is strictly above
/// 0.43633232 (magnitude via the 0x7fffffff mask); the float compares are
/// ordered float comparison semantics, so NaN fails. Returns `out` when the store
/// happened, else `obj` (the original leaves `eax` holding whichever was
/// loaded last). Cdecl, four stack words.
use lf_checker_rt::export;

export!(cdecl, rw_00d785e0(obj: u32, v0b: u32, v1b: u32, out: u32) -> u32 {
    unsafe {
        const MODE_OFF: u32 = 0x1304;
        const MODE_MATCH: u32 = 2;
        const THRESH_HI: f32 = f32::from_bits(0x4000_0000); // 2.0
        const ABS_MASK: u32 = 0x7fff_ffff;
        const THRESH_MAG: f32 = f32::from_bits(0x3edf_66f3); // 0.43633232
        if ((obj + MODE_OFF) as *const u32).read_unaligned() != MODE_MATCH {
            return obj;
        }
        if !(f32::from_bits(v0b) > THRESH_HI) {
            return obj;
        }
        let mag = f32::from_bits(v1b & ABS_MASK);
        if !(mag > THRESH_MAG) {
            return obj;
        }
        (out as *mut u8).write(1);
        out
    }
});
