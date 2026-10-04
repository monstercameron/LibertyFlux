// original: 0x009a4640 NativeImpl_RETUNE_RADIO_TO_STATION_NAME
/// Original 0x009a4640 (unnamed): match a name against "OFF" and arm state.
///
/// Compares the NUL-terminated string at `s` with the constant "OFF" two
/// bytes at a time. On mismatch, normalises the name through a helper and
/// forwards it with `this` to the tag-store routine. On match with a live
/// owner pointer at +0x28 (and a tag word at +0x74 other than 0xff), runs
/// the arm helper, stamps the mode word, clears the global mode byte and
/// sets the ready flag on the owner. Returns the owner, or 0, or the
/// forwarded answer on the mismatch path.
export!(thiscall, rw_009a4640(this_: u32, s: u32) -> u32 {
    let c = relocated(0x00E913FC);
    let mut i = 0u32;
    let matched = loop {
        let a0 = unsafe { ((s + i) as *const u8).read() };
        let b0 = unsafe { ((c + i) as *const u8).read() };
        if a0 != b0 {
            break false;
        }
        if a0 == 0 {
            break true;
        }
        let a1 = unsafe { ((s + i + 1) as *const u8).read() };
        let b1 = unsafe { ((c + i + 1) as *const u8).read() };
        if a1 != b1 {
            break false;
        }
        if a1 == 0 {
            break true;
        }
        i = i.wrapping_add(2);
    };
    if !matched {
        let t = callee_cdecl!(2, u32, s, 0);
        return callee_thiscall!(3, u32, this_, t);
    }
    let owner = unsafe { ((this_ + 0x28) as *const u32).read() };
    if owner == 0 {
        return 0;
    }
    if unsafe { ((this_ + 0x74) as *const u32).read() } == 0xff {
        return 0;
    }
    callee_cdecl!(1, u32, 0);
    unsafe {
        ((this_ + 0x6c) as *mut u32).write(3);
        (relocated(0x012845C8) as *mut u8).write(0);
        ((owner + 0xd3b) as *mut u8).write(1);
    }
    owner
});
