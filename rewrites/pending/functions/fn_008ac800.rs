// original: 0x008ac800 rage::audDelayEffect::~audDelayEffect__deleting
/// rage::audDelayEffect deleting destructor.
export!(thiscall, rw_008ac800(this: *mut u32, flag: u32) -> u32 {
    unsafe {
        let buf = *this.add(0x74 / 4);
        *this = relocated(0x00e7c698);
        callee_cdecl!(1, u32, buf);
        callee_thiscall!(3, u32, this as u32);
        if flag & 1 != 0 {
            callee_cdecl!(2, u32, this as u32);
        }
        this as u32
    }
});

