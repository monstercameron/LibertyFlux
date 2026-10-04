// original: 0x00b370e0 detail_level_for_category
/// Map a category id to a detail level using the timer-derived float `t`:
/// each known id compares `t` against two thresholds from rodata.
lf_checker_rt::export!(cdecl, rw_b370e0(a: u32) -> u32 {
    unsafe {
        let t = (lf_checker_rt::callee_cdecl!(2, u32,) as i32 as f32) * lf_checker_rt::global::<f32>(0xfe8684).read();
        match a {
            4 => {
                if lf_checker_rt::global::<f32>(0xfe87e8).read() > t {
                    0x0a
                } else if lf_checker_rt::global::<f32>(0xfe8858).read() > t {
                    0x0c
                } else {
                    7
                }
            }
            0x0d => {
                if lf_checker_rt::global::<f32>(0xfe87e8).read() > t {
                    0x0c
                } else if lf_checker_rt::global::<f32>(0xfe8858).read() > t {
                    0x0d
                } else {
                    7
                }
            }
            6 => {
                if lf_checker_rt::global::<f32>(0xfe87e4).read() > t {
                    7
                } else if lf_checker_rt::global::<f32>(0xfe8874).read() > t {
                    0x0d
                } else {
                    0x0c
                }
            }
            0x0e => {
                if lf_checker_rt::global::<f32>(0xfe87d0).read() > t {
                    a
                } else if lf_checker_rt::global::<f32>(0xfe8874).read() > t {
                    0x0c
                } else {
                    7
                }
            }
            _ => 0xffff_ffff,
        }
    }
});
