// original: 0x00d8e090 audDoorAudioEntity::audDoorAudioEntity
/// Door audio entity constructor: builds the base `audEntity`, then stamps
/// the door type tag and clears two state words.
export!(thiscall, rw_00d8e090(this: *mut u32) -> u32 {
    unsafe {
        let me = this as u32;
        callee_thiscall!(1, u32, me);
        *this = relocated(0x00EE_DB64);
        *this.add(2) = 0;
        *this.add(3) = 0;
        me
    }
});
