// original: 0x00b1d100 PC10 (symbols)

/// Selects the active device profile through a four-stage probe chain.
///
/// Cdecl with no arguments. Opens the device hub (the vtable slot at
/// +0x18 of the hub pointer, stdcall of the hub and a scratch word that
/// receives the device pointer); on a negative answer it reports through
/// the error sink (cdecl of a message constant and two zeros) and
/// returns the sink's answer. Otherwise it walks up to four probe
/// stages: each formats a five-word query (cdecl, answer kept), then
/// submits it with the device pointer through the device's own slot at
/// +0x28 (stdcall of seven words: the two fresh pushes plus the five
/// still-stacked query words). A negative stage-1 answer selects
/// profile 1; a nonnegative stage-2 answer selects 2; a nonnegative
/// stage-3 answer selects 5; otherwise stage 4 runs and selects 4 on a
/// nonnegative answer, 0 on a negative one. The profile's table word
/// goes to the active slot and the profile number to the index slot,
/// then the device is released through its slot at +8 (stdcall of the
/// device) and the release answer is returned.
lf_checker_rt::export!(cdecl, rw_00b1d100() -> u32 {
    unsafe {
        const HUB_PTR: u32 = 0x017ed8d8;
        const PROFILE_TABLE: u32 = 0x01045520;
        const ACTIVE_SLOT: u32 = 0x01633800;
        const INDEX_SLOT: u32 = 0x01045538;
        const ERROR_MSG: u32 = 0x00eab8a8;
        let hub = (lf_checker_rt::global::<u32>(HUB_PTR) as *const u32).read_unaligned();
        let mut out = 0u32;
        let opened: u32 = lf_checker_rt::callee_stdcall!(
            1,
            u32,
            hub,
            (&mut out) as *mut u32 as u32
        );
        if (opened as i32) < 0 {
            return lf_checker_rt::callee_cdecl!(
                8,
                u32,
                lf_checker_rt::relocated(ERROR_MSG),
                0,
                0
            );
        }
        let device = out;
        macro_rules! stage {
            ($x:expr, $fcc:expr) => {{
                let answer: u32 =
                    lf_checker_rt::callee_cdecl!(2, u32, 1, 0x16, $x, 3, $fcc);
                lf_checker_rt::callee_stdcall!(3, u32, device, answer, 1, 0x16, $x, 3, $fcc)
            }};
        }
        let profile = if (stage!(0x100001u32, 0x74u32) as i32) < 0 {
            1u32
        } else if (stage!(2u32, 0x5a574152u32) as i32) >= 0 {
            2u32
        } else if (stage!(2u32, 0x34324644u32) as i32) >= 0 {
            5u32
        } else {
            let last = stage!(2u32, 0x5a544e49u32);
            if (last as i32) < 0 {
                0u32
            } else {
                4u32
            }
        };
        let word = (lf_checker_rt::relocated(
            PROFILE_TABLE.wrapping_add(profile.wrapping_mul(4)),
        ) as *const u32)
            .read_unaligned();
        (lf_checker_rt::global::<u32>(ACTIVE_SLOT) as *mut u32).write_unaligned(word);
        (lf_checker_rt::global::<u32>(INDEX_SLOT) as *mut u32).write_unaligned(profile);
        lf_checker_rt::callee_stdcall!(7, u32, device)
    }
});
