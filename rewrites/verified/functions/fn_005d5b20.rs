// original: 0x005d5b20 html_node_compute_extents
/// Walk two offset-`+8` chains from this node and derive four floats into
/// the caller buffers.
///
/// Each walk keeps following links while the node's tag float equals the
/// shared `-1.0` constant (an unordered NaN comparison counts as unequal)
/// or the tag word at `+0x14` is `0x12`. From the first landing node it
/// writes `b0 + w2c` and `w20 - (b0 + wB4)`; from the second `b8 + w30`
/// and `w24 - (b8 + wAc)`, where `b0`/`b8` are the bias words at `+0xb0`
/// / `+0xb8`. Returns the fourth buffer pointer.
export!(thiscall, rw_005d5b20(
    this_ptr: u32,
    out0: u32,
    out1: u32,
    out2: u32,
    out3: u32,
) -> u32 {
    let minus_one = unsafe { global::<f32>(0x00FE8D94).read() };
    // First chain, tag float at +0x20.
    let mut d = this_ptr;
    if d != 0 {
        loop {
            let fv = unsafe { ((d + 0x20) as *const f32).read() };
            let tag = unsafe { ((d + 0x14) as *const u32).read() };
            if !(fv == minus_one || tag == 0x12) {
                break;
            }
            d = unsafe { ((d + 8) as *const u32).read() };
            if d == 0 {
                break;
            }
        }
    }
    let b0 = unsafe { ((d + 0xB0) as *const f32).read() };
    let w2c = unsafe { ((d + 0x2C) as *const f32).read() };
    unsafe { (out0 as *mut f32).write(w2c + b0) };
    let w_b4 = unsafe { ((d + 0xB4) as *const f32).read() };
    let w20 = unsafe { ((d + 0x20) as *const f32).read() };
    unsafe { (out2 as *mut f32).write(w20 - (w_b4 + b0)) };
    // Second chain, tag float at +0x24.
    let mut c = this_ptr;
    if c != 0 {
        loop {
            let fv = unsafe { ((c + 0x24) as *const f32).read() };
            let tag = unsafe { ((c + 0x14) as *const u32).read() };
            if !(fv == minus_one || tag == 0x12) {
                break;
            }
            c = unsafe { ((c + 8) as *const u32).read() };
            if c == 0 {
                break;
            }
        }
    }
    let b8 = unsafe { ((c + 0xB8) as *const f32).read() };
    let w30 = unsafe { ((c + 0x30) as *const f32).read() };
    unsafe { (out1 as *mut f32).write(w30 + b8) };
    let w_ac = unsafe { ((c + 0xAC) as *const f32).read() };
    let w24 = unsafe { ((c + 0x24) as *const f32).read() };
    unsafe { (out3 as *mut f32).write(w24 - (w_ac + b8)) };
    out3
});
