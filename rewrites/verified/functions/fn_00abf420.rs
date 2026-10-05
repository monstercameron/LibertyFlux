// original: 0x00ABF420 stream_dispatch_param_notify (proposed)

/// Notify with the parameter slot selected by (`kind`, `bank`).
///
/// The original picks one of six global float slots from the kind word and
/// the bank byte (cdecl, two words): bank 1 maps kinds 2/0/4 to its three
/// slots, any other bank maps kinds 2/(0 or 3)/4 to its three slots. On a
/// hit it calls the notify callee with the slot's float and returns the
/// callee's answer; otherwise it returns `kind` unchanged.
lf_checker_rt::export!(cdecl, rw_00ABF420(kind: u32, bank: u32) -> u32 {
    unsafe {
        const NOTIFY: u32 = 1;
        const B1_K2: u32 = 0x0103EF7C;
        const B1_K0: u32 = 0x0103EF64;
        const B1_K4: u32 = 0x0103EF94;
        const B0_K2: u32 = 0x0103EF34;
        const B0_K03: u32 = 0x0103EF1C;
        const B0_K4: u32 = 0x0103EF4C;
        let slot = if bank as u8 == 1 {
            if kind == 2 {
                Some(B1_K2)
            } else if kind == 0 {
                Some(B1_K0)
            } else if kind == 4 {
                Some(B1_K4)
            } else {
                None
            }
        } else if kind == 2 {
            Some(B0_K2)
        } else if kind == 0 || kind == 3 {
            Some(B0_K03)
        } else if kind == 4 {
            Some(B0_K4)
        } else {
            None
        };
        match slot {
            Some(s) => {
                let v = (lf_checker_rt::relocated(s) as *const u32).read_unaligned();
                lf_checker_rt::callee_cdecl!(NOTIFY, u32, v)
            }
            None => kind,
        }
    }
});
