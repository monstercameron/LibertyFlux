// original: 0x009b7040 input_obj_index_record
/// Resolve the indexed record of an input object, or null for a bad index.
///
/// Reads the signed index at +0x52c: a negative index yields null, otherwise
/// the result is the object plus index times the 0x84-byte record stride.
export!(thiscall, rw_009b7040(this_: u32) -> u32 {
    unsafe {
        const INDEX_OFF: u32 = 0x52C;
        const STRIDE: u32 = 0x84;
        let idx = *((this_.wrapping_add(INDEX_OFF)) as *const i32);
        if idx < 0 {
            0
        } else {
            this_.wrapping_add((idx as u32).wrapping_mul(STRIDE))
        }
    }
});