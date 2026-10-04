// original: 0x00ab9760 validated_byte_lookup_s9
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

/// Look up one byte at a strided offset through the validated object.
///
/// Same validation as `rw_00ab9730`, but strided by two keys.
///
/// On success returns the byte at `[obj+0x12C]+index*8+index2+0x37` in
/// the low byte with `index2`'s upper bytes preserved (the scale read
/// happens before the register restore, the offset read after, which is
/// why the two keys live in different argument slots), and 0xFFFFFF00
/// when validation yields -1.
lf_checker_rt::export!(cdecl, rw_00ab9760(p: u32, index: u32, index2: u32) -> u32 {
    // SAFETY: as in rw_00ab9730; the strided row read stays in heap.
    let tag = unsafe { ((p.wrapping_add(0x2E)) as *const i16).read_unaligned() } as isize;
    let obj = unsafe { lf_checker_rt::global::<u32>(0x0129_5CD8).offset(tag).read_unaligned() };
    let vtab = unsafe { (obj as *const u32).read_unaligned() };
    let slot = unsafe { ((vtab.wrapping_add(0x30)) as *const u32).read_unaligned() };
    let validate: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
    if validate(obj) == 0xFFFF_FFFF {
        return 0xFFFF_FF00;
    }
    let base = unsafe { ((obj.wrapping_add(0x12C)) as *const u32).read_unaligned() };
    let at = base.wrapping_add(index.wrapping_mul(8)).wrapping_add(index2).wrapping_add(0x37);
    let byte = unsafe { (at as *const u8).read() };
    with_low_byte(index2, byte)
});

