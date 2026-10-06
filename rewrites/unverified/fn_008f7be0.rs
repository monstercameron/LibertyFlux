// original: 0x008f7be0 input_gated_flag_byte (proposed)

/// Return the device flag byte when its gate is open, else zero.
///
/// `obj` is the input device object. When the gate byte at `+0x471` is
/// nonzero the function returns the value byte at `+0x470`; otherwise it
/// returns 0. Only AL is meaningful (the taken branch leaves the caller
/// EAX's upper bytes in place), so the contract compares `al`.
///
/// Thiscall: object in ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f7be0(obj: u32) -> u32 {
    unsafe {
        const VALUE_OFF: u32 = 0x470;
        const GATE_OFF: u32 = 0x471;
        if ((obj + GATE_OFF) as *const u8).read() != 0 {
            ((obj + VALUE_OFF) as *const u8).read() as u32
        } else {
            0
        }
    }
});
