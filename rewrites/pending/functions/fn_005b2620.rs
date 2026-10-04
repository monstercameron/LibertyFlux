// original: 0x005B2620 dual_state_update
/// Polls the shared status source and rotates the two latched state bytes
/// through it. When both bytes read back clear, notifies the reset helper,
/// marks this object, and clears the latches.
export!(thiscall, rw_005B2620(obj: *mut u8) -> u32 {
    unsafe {
        let eax: u32;
        let dl: u8;
        if *global::<u8>(0x011609D6) != 0 {
            let fresh = callee_cdecl!(1, u32,);
            let prev = *global::<u8>(0x01BB5639);
            *global::<u8>(0x01BB5644) = fresh as u8;
            dl = fresh as u8;
            eax = (fresh & 0xFFFF_FF00) | prev as u32;
        } else {
            let fresh = callee_cdecl!(1, u32,);
            dl = *global::<u8>(0x01BB5644);
            *global::<u8>(0x01BB5639) = fresh as u8;
            eax = fresh;
        }
        if dl == 0 && eax as u8 == 0 {
            let r = callee_cdecl!(2, u32, 0);
            *obj.add(1) = 1;
            *global::<u8>(0x01BB5644) = 0;
            *global::<u8>(0x01BB5639) = 0;
            r
        } else {
            eax
        }
    }
});
