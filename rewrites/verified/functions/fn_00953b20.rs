// original: 0x00953b20 init_hub_and_clear
/// Run the three hub initializers, then clear the hub's latched flag.
///
/// The hub probe runs twice: a null first answer skips the clear, otherwise
/// the second answer's flag byte at +0x398 loses bit 0. Returns the probe
/// answer (0 when the first probe came back null).
export!(cdecl, rw_00953b20() -> u32 {
    unsafe {
        let _: u32 = callee_cdecl!(0, u32, 0, 0);
        let _: u32 = callee_cdecl!(1, u32, 0, 0);
        let _: u32 = callee_cdecl!(2, u32, 0);
        let hub = relocated(0x103E498);
        let first: u32 = callee_thiscall!(3, u32, hub);
        if first == 0 {
            return 0;
        }
        let second: u32 = callee_thiscall!(3, u32, hub);
        let flag = (second.wrapping_add(0x398)) as *mut u8;
        *flag &= 0xFE;
        second
    }
});
