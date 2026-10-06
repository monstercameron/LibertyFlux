// original: 0x0094CCC0 probe_slot_by_tag (proposed)

/// Probe one 12-byte slot selected by `index`, dispatching on its tag word.
///
/// Reads `key` at `this + index * 12` and `tag` 4 bytes past it. Tag 0
/// clears the slot through callee 2 (thiscall, `this` in ECX, `index` on
/// the stack) and returns 1. Tag 1 resolves `key` through callee 1 with
/// context `CTX_B`: a null answer clears the slot and returns 1, any other
/// returns 0. Tag 2 resolves `key` through callee 1 with context `CTX_A`:
/// a null answer clears the slot and returns 1, any other returns 0. Any
/// other tag returns 0 with no calls. All tag compares are exact equality
/// and both answer tests are full-word zero tests. Only the low byte of
/// the return is defined.
///
/// Original: 0x0094CCC0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0094CCC0(this: u32, index: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 12;
        const CTX_A: u32 = 0x12E22A4;
        const CTX_B: u32 = 0x18B6F1C;
        const RESOLVE: u32 = 1;
        const CLEAR: u32 = 2;
        let slot = this.wrapping_add(index.wrapping_mul(STRIDE));
        let key = (slot as *const u32).read_unaligned();
        let tag = (slot.wrapping_add(4) as *const u32).read_unaligned();
        if tag == 0 {
            lf_checker_rt::callee_thiscall!(CLEAR, u32, this, index);
            1
        } else if tag == 1 {
            let ctx = (lf_checker_rt::global::<u32>(CTX_B) as *const u32).read();
            let r = lf_checker_rt::callee_thiscall!(RESOLVE, u32, ctx, key);
            if r != 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(CLEAR, u32, this, index);
                1
            }
        } else if tag == 2 {
            let ctx = (lf_checker_rt::global::<u32>(CTX_A) as *const u32).read();
            let r = lf_checker_rt::callee_thiscall!(RESOLVE, u32, ctx, key);
            if r == 0 {
                lf_checker_rt::callee_thiscall!(CLEAR, u32, this, index);
                1
            } else {
                0
            }
        } else {
            0
        }
    }
});
