// original: 0x00890cd0 audio_chain_state_flag
/// Walk the chain while records report state 2; report whether it ends in 1.
///
/// Each record's state is bits 10-11 of the dword at +0x70. State 2 follows
/// the link byte at +5 through the first row table (a 0xff link ends with 0);
/// any other state ends the walk with 1 when the state is 1 and 0 otherwise.
export!(thiscall, rw_00890cd0(this: u32) -> u32 {
    unsafe {
        let mut cur = this;
        loop {
            let state = ((((cur + 0x70) as *const u32).read()) >> 10) & 3;
            if state != 2 {
                return if state == 1 { 1 } else { 0 };
            }
            let link = ((cur + 5) as *const u8).read();
            if link == 0xff {
                return 0;
            }
            let variant = ((cur + 0x40) as *const u8).read() as u32;
            let stride = *global::<u32>(0x0115D964);
            let base = *global::<u32>(0x0115D988);
            let row = ((base
                .wrapping_add(variant.wrapping_mul(0x6f40))
                .wrapping_add(0x6f10)) as *const u32)
                .read();
            cur = stride.wrapping_mul(link as u32).wrapping_add(row);
        }
    }
});
