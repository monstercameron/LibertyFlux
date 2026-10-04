// original: 0x00b56ee0 rage::crmtFrameBufferFixed<220>::vf2
// rs05f1: release a slot in a fixed frame-buffer registry
// (`rage::crmtFrameBufferFixed<220>::vf2`).
//
// Searches the 50 entry slots at `this+4+i*0x14` for `target`; when slot `i`
// matches, clears bit `i` in the occupancy mask at `this+0x3EC`. A guard
// object on the frame brackets the search (an init call, then one of two
// fini call sites depending on the outcome); the guard address is volatile
// across compilers and excluded from call comparison by the contract.
export!(thiscall, rw_b56ee0(this: *mut u8, target: u32) -> () {
    unsafe {
        let mut guard = [0u32; 2];
        let init: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        init(
            guard.as_mut_ptr() as u32,
            this.add(0x3F4) as u32,
        );
        let mut found: Option<u32> = None;
        for i in 0..50u32 {
            if this.add(4 + (i as usize) * 0x14) as u32 == target {
                found = Some(i);
                break;
            }
        }
        if let Some(i) = found {
            let word = this.add(0x3EC + (i as usize >> 5) * 4) as *mut u32;
            *word &= !(1u32 << (i & 31));
            let fini: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(callee_addr(3) as usize);
            fini(guard.as_mut_ptr() as u32);
        } else {
            let fini: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(callee_addr(2) as usize);
            fini(guard.as_mut_ptr() as u32);
        }
    }
});
