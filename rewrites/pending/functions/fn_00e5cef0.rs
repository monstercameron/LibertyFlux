// original: 0x00e5cef0 timer_state_reset
/// Reset the mainloop timer state block, then report success.
///
/// Calls the timer helper (`thiscall/0`, stubbed) with ECX pointing at the
/// state block, then writes the power-on pattern over `0x01908DA8..0x01908E00`:
/// zeroed 64-bit slots and zeroed words mixed with `0xFFFFFFFF` marker dwords.
/// Three 2-byte gaps (`0xDAA`, `0xDE2`, `0xDEA`) are left untouched, and two
/// 64-bit slots are written twice, exactly like the original. Returns 0.
export!(cdecl, rw_00e5cef0() -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, relocated(0x1908DC0));
        *global::<u64>(0x1908DC0) = 0;
        *global::<u64>(0x1908DC8) = 0;
        *global::<u32>(0x1908DA8) = 0xFFFFFFFF;
        *global::<u64>(0x1908DB0) = 0;
        *global::<u64>(0x1908DB8) = 0;
        *global::<u32>(0x1908DD0) = 0;
        *global::<u64>(0x1908DC0) = 0;
        *global::<u64>(0x1908DC8) = 0;
        *global::<u32>(0x1908DD4) = 0xFFFFFFFF;
        *global::<u16>(0x1908DD8) = 0;
        *global::<u32>(0x1908DDC) = 0xFFFFFFFF;
        *global::<u16>(0x1908DE0) = 0;
        *global::<u32>(0x1908DE4) = 0xFFFFFFFF;
        *global::<u16>(0x1908DE8) = 0;
        *global::<u32>(0x1908DEC) = 0;
        *global::<u64>(0x1908DF0) = 0;
        *global::<u32>(0x1908DF8) = 0xFFFFFFFF;
        *global::<u32>(0x1908DFC) = 0xFFFFFFFF;
        0
    }
});
