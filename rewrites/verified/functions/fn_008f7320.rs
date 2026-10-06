// original: 0x008F7320 InputDevice_ctor

/// Construct the device object at `this`: zero the header words, construct
/// the sub-objects at `+0x44`, `+0x50`, `+0x38` (flagging `+0x40`) and
/// `+0x60`, zero the state words at `+0x10..+0x1C` and `+0x470`, set the mode
/// word at `+0x20` to -1, run the final setup, and return `this`.
/// Convention: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_008f7320(this: u32) -> u32 {
    unsafe {
        const SUB_A: u32 = 1;
        const SUB_B: u32 = 2;
        const SUB_C: u32 = 3;
        const SETUP: u32 = 4;
        for off in [0_u32, 4, 8, 0xC, 0x38, 0x3C] {
            (this.wrapping_add(off) as *mut u32).write_unaligned(0);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            SUB_A,
            u32,
            this.wrapping_add(0x44)
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            SUB_A,
            u32,
            this.wrapping_add(0x50)
        );
        (this.wrapping_add(0x40) as *mut u8).write(1);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            SUB_B,
            u32,
            this.wrapping_add(0x38)
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            SUB_C,
            u32,
            this.wrapping_add(0x60)
        );
        (this.wrapping_add(0x470) as *mut u16).write_unaligned(0);
        for off in [0x10_u32, 0x14, 0x18, 0x1C] {
            (this.wrapping_add(off) as *mut u32).write_unaligned(0);
        }
        (this.wrapping_add(0x20) as *mut u32).write_unaligned(0xFFFF_FFFF);
        let _: u32 = lf_checker_rt::callee_thiscall!(SETUP, u32, this);
        this
    }
});
