// original: 0x00adffb0 ui_gated_meter_push
/// Push two meter readings downstream when the gate arguments select channel 1.
///
/// Does nothing unless called with `(1, 0)`. Otherwise it hands the two global
/// meter floats to the first sink as a pair, reads the indexed table float
/// selected by the global index (`table[i]` at stride `0x210`) for the second
/// sink, forwards this object's word at `+0x30` to the object sink whose `this`
/// comes from a global, and finally sends the constant `1.0` to the second
/// sink again. Returns nothing meaningful.
export!(thiscall, rw_00adffb0(this_ptr: u32, a: u32, b: u32) -> u32 {
    unsafe {
        if a != 1 || b != 0 {
            return 0;
        }
        let pair = [
            *global::<u32>(0x139C234),
            *global::<u32>(0x139C238),
        ];
        callee_cdecl!(1, u32, pair.as_ptr() as u32);
        let idx = *global::<u32>(0x1174794);
        let entry = *global::<u32>(0x15E8994u32.wrapping_add(idx.wrapping_mul(0x210)));
        callee_cdecl!(2, u32, entry);
        let sink_this = *global::<u32>(0x1BB6674);
        let w = ((this_ptr + 0x30) as *const u32).read();
        callee_thiscall!(3, u32, sink_this, w);
        callee_cdecl!(2, u32, 0x3F800000);
        0
    }
});
