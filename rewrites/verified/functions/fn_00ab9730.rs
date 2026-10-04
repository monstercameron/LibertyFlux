// original: 0x00ab9730 validated_byte_lookup
/// Replace the low byte of `hi` with `lo`, keeping the upper 24 bits.
///
/// This trivial combine is deliberately `#[inline(never)]`: rustc 1.96.1
/// in release for i686 miscompiles the inlined form
/// `(hi & 0xFFFF_FF00) | (lo as u32)` into just `lo` whenever `lo` was
/// loaded from an address derived from `hi` (the mask, shift, byte-array,
/// store and volatile spellings all fold the same way; the out-of-line
/// call is the only form observed to keep the upper bits). The checker
/// proves the combined value on every trial, so any toolchain drift that
/// reintroduces the fold fails loudly. Do not inline this function.
#[inline(never)]
fn with_low_byte(hi: u32, lo: u8) -> u32 {
    (hi & 0xFFFF_FF00) | (lo as u32)
}

/// Look up one byte through the object selected by `p`'s tag word.
///
/// The signed tag at `p+0x2E` selects an object from the global table;
/// the object is validated through its vtable slot 0x30. A validation
/// result of -1 yields 0xFFFFFF00 (only the low byte is significant);
/// otherwise the byte at `[obj+0x12C]+index+0x2C` is returned in the low
/// byte with the index's upper bytes preserved.
lf_checker_rt::export!(cdecl, rw_00ab9730(p: u32, index: u32) -> u32 {
    // SAFETY: all dereferences stay inside checker-backed memory: the tag
    // word in heap, the table slot in the image, the object/vtable/row in
    // fabricated heap on every trial.
    let tag = unsafe { ((p.wrapping_add(0x2E)) as *const i16).read_unaligned() } as isize;
    let obj = unsafe { lf_checker_rt::global::<u32>(0x0129_5CD8).offset(tag).read_unaligned() };
    let vtab = unsafe { (obj as *const u32).read_unaligned() };
    let slot = unsafe { ((vtab.wrapping_add(0x30)) as *const u32).read_unaligned() };
    let validate: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
    if validate(obj) == 0xFFFF_FFFF {
        return 0xFFFF_FF00;
    }
    let base = unsafe { ((obj.wrapping_add(0x12C)) as *const u32).read_unaligned() };
    let byte = unsafe { ((base.wrapping_add(index).wrapping_add(0x2C)) as *const u8).read() };
    with_low_byte(index, byte)
});

