// original: 0x008a93b0 rage::audEffect::vf1
/// `rage::audEffect::vf1`: attach the info block and reset the gains.
///
/// Records the info pointer at `this+4` (a null pointer returns 0 at once),
/// initialises the small scalar fields, then reads the tag dword at
/// `info+0xa`. Unless the tag is -1 the voice handle at `this+8` comes from
/// the lookup helper (thiscall/2 on the global audio manager, stubbed);
/// otherwise the handle is null. Finally all fifteen gain slots at
/// `this+0x34` are reset to 1.0 and 1 is returned (in AL; the full EAX value
/// the original leaves behind is reproduced exactly).
export!(thiscall, rw_008a93b0(this: *mut u8, info: *const u8, param: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x0115DAD4;
        const ONE: u32 = 0x3F800000;
        *(this.add(0x04) as *mut u32) = info as u32;
        if info.is_null() {
            return 0;
        }
        *(this.add(0x30) as *mut u32) = 1;
        *(this.add(0x2c) as *mut u32) = 0;
        *(this.add(0x24) as *mut u32) = param;
        let tag = (info.add(0x0a) as *const u32).read_unaligned();
        let eax = if tag != 0xFFFF_FFFF {
            let handle =
                callee_thiscall!(1, u32, relocated(MANAGER), tag, param.wrapping_add(1));
            *(this.add(0x08) as *mut u32) = handle;
            (handle & 0xFFFF_FF00) | 1
        } else {
            *(this.add(0x08) as *mut u32) = 0;
            ((info as u32) & 0xFFFF_FF00) | 1
        };
        let gains = this.add(0x34) as *mut u32;
        for i in 0..15usize {
            *gains.add(i) = ONE;
        }
        eax
    }
});
