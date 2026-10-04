// original: 0x006f4fe0 pack_six_fields
/// Packs six caller-supplied values into the object through the field writer.
///
/// Forwards the six stack arguments, two of them narrowed to 16 bits, to six
/// field-writer calls with fixed field ids and widths.
export!(thiscall, rw_006f4fe0(
    this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32,
) -> () {
    unsafe {
        callee_cdecl!(1, u32, this, a0.wrapping_add(9), 10, 0);
        callee_cdecl!(1, u32, this, a5, 4, 10);
        callee_cdecl!(1, u32, this, a1, 4, 14);
        callee_cdecl!(1, u32, this, (a2 & 0xFFFF) as u32, 16, 18);
        callee_cdecl!(1, u32, this, (a3 & 0xFFFF) as u32, 16, 34);
        callee_cdecl!(1, u32, this, a4, 22, 50);
    }
});
