// original: 0x00BE81C0 status_latch_init (proposed)
/// Latch a status byte to 1 the first time it is seen as 0.
///
/// `obj` points to the object; the flag byte at `+0x29` is set to 1 when
/// it reads 0 and left alone otherwise. The trailing stack word is
/// accepted (the original pops it) but never read. No result.
///
/// Original: 0x00BE81C0 (thiscall, one ignored stack word).
lf_checker_rt::export!(thiscall, rw_00BE81C0(obj: u32, _unused: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x29;
        let flag = (obj + FLAG) as *mut u8;
        if flag.read() == 0 {
            flag.write(1);
        }
        0
    }
});

