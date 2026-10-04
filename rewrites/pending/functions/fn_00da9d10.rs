// original: 0x00da9d10 flee_altitude_band
/// Classify a height sample into band 1 (clearly above the anchor) or band 2.
///
/// The sample height is read from the second object, the reference height from
/// the anchor reached through the first object. Band 1 needs the difference to
/// be strictly greater than half a unit; anything at or below the boundary,
/// including an unordered (NaN) difference, takes band 2.
export!(cdecl, rw_00da9d10(a: u32, c: u32) -> u32 { unsafe {
    /// Offset of the anchor pointer inside the first object.
    const ANCHOR: u32 = 0x20;
    /// Offset of the reference height inside the anchor object.
    const REF_H: u32 = 0x38;
    /// Offset of the sample height inside the second object.
    const H: u32 = 8;
    /// Band boundary.
    const LIMIT: f32 = 0.5;
    let b = ((a.wrapping_add(ANCHOR)) as *const u32).read_unaligned();
    let d = f32::from_bits(((c.wrapping_add(H)) as *const u32).read_unaligned()) - f32::from_bits(((b.wrapping_add(REF_H)) as *const u32).read_unaligned());
    // The original compares with comiss+setbe, which reports an unordered
    // result as below-or-equal, so only a strict greater-than takes band 1.
    if d > LIMIT { 1 } else { 2 }
    } });
