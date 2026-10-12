// original: 0x00C7A420 CTaskComplexStationaryScenario::CTaskComplexStationaryScenario

/// Construct a stationary-scenario task. Forwards (a0, a1, a2) to the scenario base constructor, stores the low bytes of a3 at +0x24, a4 at +0x59 and a5 at +0x65, installs the relocated vtable, clears the state fields, and when the word at +0x1c is nonzero copies the four-word vector it selects (nested +0x30 when the word at +0x20 is nonzero, else +0x10) to +0x40 and sets +0x25. Finally stores the word at +0x24 of the object returned for a0 at +0x5c. Returns `this`. Original: thiscall, six stack words.
lf_checker_rt::export!(thiscall, rw_c7a420(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr16(address: u32, value: u16) { unsafe { (address as *mut u16).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)]
        unsafe fn wr8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, a0, a1, a2);
                const VTABLE_VA: u32 = 0x00ED592C;
                const VEC: u32 = 0x40;
                wr8(this.wrapping_add(0x24), a3 as u8);
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr8(this.wrapping_add(0x25), 0);
                for off in [0x30u32, 0x34, 0x38, 0x40, 0x44, 0x48] { wr32(this.wrapping_add(off), 0); }
                wr8(this.wrapping_add(0x59), a4 as u8);
                wr8(this.wrapping_add(0x65), a5 as u8);
                wr32(this.wrapping_add(0x50), 0);
                wr32(this.wrapping_add(0x54), 0);
                wr8(this.wrapping_add(0x58), 0);
                wr32(this.wrapping_add(0x5c), 0xbf80_0000);
                wr32(this.wrapping_add(0x60), 0);
                wr8(this.wrapping_add(0x64), 0);
                wr8(this.wrapping_add(0x66), 0);
                wr32(this.wrapping_add(0x68), 0);
                wr8(this.wrapping_add(0x6c), 0);
                let anchor = rd32(this.wrapping_add(0x1c));
                if anchor != 0 {
                    let nested = rd32(anchor.wrapping_add(0x20));
                    let vec = if nested != 0 { nested.wrapping_add(0x30) } else { anchor.wrapping_add(0x10) };
                    for lane in 0..4u32 { wr32(this.wrapping_add(VEC + lane * 4), rd32(vec.wrapping_add(lane * 4))); }
                    wr8(this.wrapping_add(0x25), 1);
                }
                let info = lf_checker_rt::callee_cdecl!(2, u32, a0);
                wr32(this.wrapping_add(0x5c), rd32(info.wrapping_add(0x24)));
                this
        }
});
