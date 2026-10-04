// original: 0x00c654d0 cutscene_scale_record_by_time
/// Scales two float triplets of record `index` by the shared time factor.
///
/// Looks the record id up in the table at [holder+0xcc]; negative ids do
/// nothing. Otherwise resolves the record twice through helper callee 1
/// and scales the triplets at offsets 0 and 0x20.
export!(thiscall, rw_c654d0(this: u32, index: u32, holder: u32) -> u32 {
    /// File VA of the shared float multiplier.
    const TIME_SCALE_VA: u32 = 0x00FE_8D94;
    let table = unsafe { (holder as *const u32).byte_add(0xcc).read() };
    let rec = unsafe {
        (table.wrapping_add(index.wrapping_mul(4)) as *const i32).read()
    };
    if rec <= -1 {
        return index;
    }
    let factor = unsafe { global::<f32>(TIME_SCALE_VA).read() };
    let first = callee_thiscall!(1, u32, this, rec as u32);
    for i in 0..3 {
        unsafe {
            let slot = first.wrapping_add(i * 4) as *mut f32;
            slot.write(slot.read() * factor);
        }
    }
    let second = callee_thiscall!(1, u32, this, rec as u32);
    for i in 0..3 {
        unsafe {
            let slot = second.wrapping_add(0x20).wrapping_add(i * 4) as *mut f32;
            slot.write(slot.read() * factor);
        }
    }
    second
});
