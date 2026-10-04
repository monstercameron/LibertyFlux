// original: 0x009a4250 audio_dispatch_event
/// Original 0x009a4250 (unnamed): dispatch an audio event by id.
///
/// Drops the event (returning the incoming code with all low bits set) when
/// the id is zero, a backend probe fails, or the owner word at +0x74 is
/// neither 0xff nor 0xfe after a live probe. Otherwise matches `key`
/// against the global key: on match the event code comes from the codec
/// helper, on mismatch from the tag byte of the resolved record. A code of
/// 0xff drops the event; otherwise it is submitted with the id. The failure
/// value keeps incoming register bits on one path, so no return is compared.
export!(thiscall, rw_009a4250(this_: u32, id: u32, key: u32) -> u32 {
    if id == 0 {
        return 0;
    }
    if callee_cdecl!(1, u32,) == 0 {
        return 0;
    }
    if callee_cdecl!(2, u32, 0) != 0 {
        let c = unsafe { ((this_ + 0x74) as *const u32).read() };
        if c != 0xff && c != 0xfe {
            return 0;
        }
    }
    let want = unsafe { (relocated(0x01284670) as *const u32).read() };
    let code = if key == want {
        let a = callee_cdecl!(1, u32,);
        callee_cdecl!(3, u32, 0, a.wrapping_sub(1))
    } else {
        let q = callee_cdecl!(4, u32, key);
        if q == 0 {
            return 0;
        }
        unsafe { ((q + 0x1918) as *const u8).read() as u32 }
    };
    if code == 0xff {
        return 0;
    }
    callee_cdecl!(5, u32, code, id, 3)
});
