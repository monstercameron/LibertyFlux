// original: 0x009f9470 level_hysteresis_arm
use lf_k2_rt::{export, callee_addr, global};

/// One-bits float (1.0). Shared lane-crate const, inlined by the
/// v2 port (the lane's out file referenced it without defining it).
const ONE_BITS: u32 = 0x3F800000;

/// Arms when the level reaches +0.6, releases below -0.6 (cdecl/1 -> void).
///
/// Compares the nested float at obj+0x20/+0x28 against a threshold that
/// depends on the armed flag byte, fires the kick/notify steps on the rising
/// edge and maintains the flag.
export!(cdecl, rw_s18f9(obj: *const u8) -> u32 {
    unsafe {
        // Thresholds from the binary's constant table (+/-0.6).
        const HI: u32 = 0x3F19999A;
        const LO: u32 = 0xBF19999A;
        let armed = *global::<u8>(0x12B6262);
        let inner = *((obj.add(0x20)) as *const u32) as *const u8;
        let x = f32::from_bits(*((inner.add(0x28)) as *const u32));
        if armed != 0 {
            // ja-returns: x > -0.6 returns, unordered (NaN) proceeds.
            if x > f32::from_bits(LO) {
                return 0;
            }
            if *((obj.add(0x1304)) as *const u32) == 0 {
                let kick: extern "cdecl" fn() -> u32 =
                    core::mem::transmute(callee_addr(1) as usize);
                kick();
            }
            let notify: extern "cdecl" fn(u32, u32) -> u32 =
                core::mem::transmute(callee_addr(2) as usize);
            notify(0x108, ONE_BITS);
            *global::<u8>(0x12B6262) = 0;
            return 0;
        }
        // jb-returns: x < +0.6 returns AND unordered (NaN) returns, because
        // comiss sets CF on unordered. `!(x >= HI)` matches both.
        if !(x >= f32::from_bits(HI)) {
            return 0;
        }
        *global::<u8>(0x12B6262) = 1;
        0
    }
});
