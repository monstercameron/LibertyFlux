// original: 0x00d8c660 audObjectAudioEntity::audObjectAudioEntity
/// Object audio entity constructor: builds the base `audEntity`, then
/// stamps the object type tag and clears the trailing state word.
export!(thiscall, rw_00d8c660(this: *mut u32) -> u32 {
    unsafe {
        let me = this as u32;
        callee_thiscall!(1, u32, me);
        *this = relocated(0x00EE_D2D4);
        *this.add(2) = 0;
        me
    }
});
