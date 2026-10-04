// original: 0x00b253b0 ensure_collision_init
// s08_b253b0: ensure collision state initialised. thiscall/0: when the
// state byte at +0x1F4 is negative without bit 3, runs the state-setup
// helper (thiscall/0), then sets the initialised bit 0x80. Returns nothing.
export!(thiscall, rw_b253b0(this: *mut u8) -> () {
    unsafe {
        let st = *this.add(0x1F4);
        if (st as i8) < 0 && st & 8 == 0 {
            let setup: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(callee_addr(1) as usize);
            setup(this as u32);
        }
        *this.add(0x1F4) |= 0x80;
    }
});
