
// k-cast3 probe (instrumentation of a test copy only; never in a tracked file).
// Classes of an operand: N NaN, I infinity, P at or past +2^31 (or +2^63), M past the negative end, R in range.
fn k3_class(b: u32, wide: bool) -> u8 {
    let e = (b >> 23) & 0xff;
    if e == 0xff {
        return if b & 0x007f_ffff == 0 { b'I' } else { b'N' };
    }
    let neg = (b >> 31) == 1;
    let mag = b & 0x7fff_ffff;
    let limit: u32 = if wide { 0x5f00_0000 } else { 0x4f00_0000 };
    if neg {
        if mag > limit { b'M' } else { b'R' }
    } else if mag >= limit {
        b'P'
    } else {
        b'R'
    }
}

fn k3_log(id: u32, class: u8) {
    if let Ok(path) = std::env::var("K3_PROBE_LOG") {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(f, "{} {}", id, class as char);
        }
    }
}

pub fn k3probe(id: u32, wide: bool, v: f32) -> f32 {
    let c = k3_class(v.to_bits(), wide);
    k3_log(id, c);
    if c != b'R' && std::env::var("K3_MUT").map(|s| s == "1").unwrap_or(false) {
        return 0.0;
    }
    v
}

pub fn k3probe64(id: u32, wide: bool, v: f64) -> f64 {
    let b = v.to_bits();
    let e = (b >> 52) & 0x7ff;
    let c: u8 = if e == 0x7ff {
        if b & 0x000f_ffff_ffff_ffff == 0 { b'I' } else { b'N' }
    } else {
        let neg = (b >> 63) == 1;
        let mag = b & 0x7fff_ffff_ffff_ffff;
        let limit: u64 = if wide { 0x43e0_0000_0000_0000 } else { 0x41e0_0000_0000_0000 };
        if neg {
            if mag > limit { b'M' } else { b'R' }
        } else if mag >= limit {
            b'P'
        } else {
            b'R'
        }
    };
    k3_log(id, c);
    if c != b'R' && std::env::var("K3_MUT").map(|s| s == "1").unwrap_or(false) {
        return 0.0;
    }
    v
}
