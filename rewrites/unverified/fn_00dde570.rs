// original: 0x00DDE570 UITextField key classifier
/// Return 1 when `key` lies in 0x8E..=0xCD or equals 0xD3, else 0.
/// The range comparisons are SIGNED (jl/jle); negative values never match.
/// Only al is written, so the upper 24 bits of the key are preserved in
/// the result. Takes no object (ecx is ignored). Original: one stack word.
lf_checker_rt::export!(thiscall, rw_00DDE570(_this: u32, key: u32) -> u32 {
    let k = key as i32;
    if (0x8Ei32 <= k && k <= 0xCDi32) || k == 0xD3i32 { (key & 0xFFFF_FF00) | 1 }
    else { key & 0xFFFF_FF00 }
});
