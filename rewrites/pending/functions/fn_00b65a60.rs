// original: 0x00b65a60 lifecycle_state_machine
// thiscall/1. Drives the lifecycle word at +0xb8: stage 3 advances to 4;
// stages 4/2/1 run their entry step plus the shared finish step and clear;
// any other stage leaves the word untouched. Returns nothing.
export!(thiscall, rw_rs11f18(this: *mut u8, a: u32) -> () {
    unsafe {
        let stage = this.add(0xb8) as *mut u32;
        match *stage {
            3 => {
                *stage = 4;
            }
            4 => {
                let enter: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(callee_addr(1) as usize);
                enter(this as u32, a, 1);
                let finish: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(callee_addr(3) as usize);
                finish(this as u32, 0x11, 0, 0, 0);
                *stage = 0;
            }
            2 => {
                let enter: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(callee_addr(2) as usize);
                enter(this as u32, a, 0x2e, 1);
                let finish: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(callee_addr(3) as usize);
                finish(this as u32, 0x11, 0, 0, 0);
                *stage = 0;
            }
            1 => {
                let enter: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(callee_addr(2) as usize);
                enter(this as u32, a, 0x2e, 1);
                let finish: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(callee_addr(3) as usize);
                finish(this as u32, 0, 0, 0, 0);
                *stage = 0;
            }
            _ => {
                // Any other stage returns without touching the stage word:
                // the original jumps past the clearing store.
            }
        }
    }
});
