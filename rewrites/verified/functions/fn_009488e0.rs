// original: 0x009488e0 select_filtered_extremum
/// Select a filtered extremum over four probed samples, or a direct lane.
///
/// Takes five floats (f0, a, b, c, d), a flag word and an index word and
/// returns a float. When the flag's low byte is non-zero the index picks
/// a direct lane: above 11 the result is 0.0, indices 1, 3 and 10 return
/// b+d if 0.0 > d else b-d (so NaN d subtracts), and every other index
/// returns b. (That dispatch is the original's code-embedded jump table,
/// replicated here as a plain match: the rewrite cannot read the table
/// bytes because the checker's code pages are revoked while it runs.)
/// When the flag's low byte is zero, four samples are probed: with
/// b > -100.0 or b NaN through a scripted six-float helper as
/// (f0, a+c), (f0, a-c), (f0+c, a), (f0-c, a) each with trailing
/// (b, 0, 0, 4), else through a scripted three-float helper with the
/// same first pairs and trailing 4. The running maximum m of the four
/// answers is taken (a strictly greater ordered answer replaces, so a
/// NaN first answer poisons while later NaNs are ignored), then the
/// result is the smallest answer strictly above m-1.5, or m when no
/// answer qualifies. Each return path issues the module's stack-cookie
/// check call, which carries no compared data.
export!(cdecl, rw_009488e0(
    f0: f32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    flag: u32,
    idx: u32,
) -> f32 {
    unsafe {
        const F_ID: u32 = 5;
        const G_ID: u32 = 6;
        const COOKIE_ID: u32 = 9;
        const NEG100_GLOB: u32 = 0xFE8DF8;
        const WINDOW_GLOB: u32 = 0xFE8960;

        if (flag as u8) != 0 {
            let r = if idx > 11 {
                0.0
            } else if idx == 1 || idx == 3 || idx == 10 {
                // Original is comiss(0.0, d)+jbe: add only when 0.0 > d.
                if 0.0 > d {
                    b + d
                } else {
                    b - d
                }
            } else {
                b
            };
            let _: u32 = callee_cdecl!(COOKIE_ID, u32,);
            return r;
        }
        let neg100 = *(relocated(NEG100_GLOB) as *const f32);
        let s = a + c;
        // Original is comiss(-100.0, b)+jb: taken when b is strictly
        // greater or unordered (NaN).
        let g_path = b > neg100 || b.is_nan();
        let mut rs = [0.0f32; 4];
        if g_path {
            rs[0] = callee_cdecl!(
                G_ID, f32, f0.to_bits(), s.to_bits(), b.to_bits(), 0, 0, 4
            );
            rs[1] = callee_cdecl!(
                G_ID, f32, f0.to_bits(), (a - c).to_bits(), b.to_bits(), 0, 0, 4
            );
            rs[2] = callee_cdecl!(
                G_ID, f32, (f0 + c).to_bits(), a.to_bits(), b.to_bits(), 0, 0, 4
            );
            rs[3] = callee_cdecl!(
                G_ID, f32, (f0 - c).to_bits(), a.to_bits(), b.to_bits(), 0, 0, 4
            );
        } else {
            rs[0] = callee_cdecl!(F_ID, f32, f0.to_bits(), s.to_bits(), 4);
            rs[1] = callee_cdecl!(F_ID, f32, f0.to_bits(), (a - c).to_bits(), 4);
            rs[2] = callee_cdecl!(F_ID, f32, (f0 + c).to_bits(), a.to_bits(), 4);
            rs[3] = callee_cdecl!(F_ID, f32, (f0 - c).to_bits(), a.to_bits(), 4);
        }
        // Running maximum: only a strictly greater ordered answer
        // replaces the current one.
        let mut m = rs[0];
        if rs[1] > m {
            m = rs[1];
        }
        if rs[2] > m {
            m = rs[2];
        }
        if rs[3] > m {
            m = rs[3];
        }
        let window = *(relocated(WINDOW_GLOB) as *const f32);
        let lo = m - window;
        let mut best = m;
        let mut i = 0;
        while i < 4 {
            let r = rs[i];
            if r > lo && best > r {
                best = r;
            }
            i += 1;
        }
        let _: u32 = callee_cdecl!(COOKIE_ID, u32,);
        best
    }
});
