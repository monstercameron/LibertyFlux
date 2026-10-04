// original: 0x009f9600 counters_reset3
use lf_k2_rt::{export, global};

/// Counter block reset, three dwords (cdecl/0 -> void). See `rw_s18f4`.
export!(cdecl, rw_s18f10() -> u32 {
    unsafe {
        *global::<u32>(0x12B6270) = 0;
        *global::<u32>(0x12B6274) = 0;
        *global::<u32>(0x12B6278) = 0;
        0
    }
});
