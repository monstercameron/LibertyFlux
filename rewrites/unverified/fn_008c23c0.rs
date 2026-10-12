// original: 0x008C23C0 audio_bank_sweep_request

/// Request an audio bank sweep at the fixed sweep rate.
///
/// Calls the sweep setup with the bank object from `BANK` and the fixed
/// rate `RATE` (-100.0f as bits), then calls the sweep service with the
/// service object from `SERVICE` and the request word from `REQUEST`,
/// returning the service result. Cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_008C23C0() -> u32 {
    unsafe {
        const BANK: u32 = 0x0116_253C;
        const REQUEST: u32 = 0x0116_18FC;
        const SERVICE: u32 = 0x0115_F848;
        const RATE: u32 = 0xC2C8_0000;
        let bank = (lf_checker_rt::relocated(BANK) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(1, u32, bank, RATE);
        let request = (lf_checker_rt::relocated(REQUEST) as *const u32).read_unaligned();
        let service = (lf_checker_rt::relocated(SERVICE) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(2, u32, service, request)
    }
});
