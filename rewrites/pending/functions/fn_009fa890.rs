// original: 0x009fa890 playstat_position_report_emit
/// Refresh the cached position report when the sample moved enough.
///
/// Seeds the report flag, samples the current position, and compares the
/// squared distance against the cached threshold: a fresh sample within
/// tolerance keeps the cache, anything else rebuilds and stores the
/// report. Float operations follow the original's order exactly so NaN
/// payloads propagate identically. Returns nothing meaningful.
export!(cdecl, rw_009fa890() -> u32 {
    unsafe {
        // File VA of the report tag object (relocated; see 5D0).
        const TAG: u32 = 0x00e99878;
        let mut frame = [0u32; 24];
        let base = frame.as_mut_ptr();
        let temp = base.add(5);
        let aux = base.add(18);
        let seed = global::<u32>(0x012b9c70);
        if seed.read() & 1 == 0 {
            seed.write(seed.read() | 1);
        }
        callee_cdecl!(1, u32, base as u32);
        // The sample is words 0-2 of the buffer (esp still holds the pushed
        // slot at the load sites, shifting every lane down one word).
        let x = f32::from_bits(base.read())
            - f32::from_bits(global::<u32>(0x012b9c60).read());
        let y = f32::from_bits(base.add(1).read())
            - f32::from_bits(global::<u32>(0x012b9c64).read());
        let z = f32::from_bits(base.add(2).read())
            - f32::from_bits(global::<u32>(0x012b9c68).read());
        // Matches `comiss sum, thresh_sq; jb skip`: skip when the sum is
        // below the threshold or unordered, i.e. run exactly when
        // `sum >= thresh_sq` holds.
        let run = if global::<u8>(0x012b9c74).read() != 0 {
            true
        } else {
            let sum = (y * y + x * x) + z * z;
            let limit = f32::from_bits(global::<u32>(0x0103b5a8).read());
            sum >= limit * limit
        };
        if run {
            callee_thiscall!(2, u32, temp as u32, 1, 0x0c);
            temp.write(relocated(TAG));
            callee_cdecl!(3, u32, base as u32, aux as u32);
            callee_cdecl!(4, u32, temp as u32, 0x38);
            let tick = callee_cdecl!(5, u32,);
            global::<u32>(0x012b900c).write(tick | 1);
            global::<u32>(0x012b9c60).write(base.read());
            global::<u32>(0x012b9c64).write(base.add(1).read());
            global::<u32>(0x012b9c68).write(base.add(2).read());
            global::<u32>(0x012b9c6c).write(base.add(3).read());
            global::<u8>(0x012b9c74).write(0);
            callee_thiscall!(6, u32, temp as u32);
        }
        // Stack-cookie check: argument excluded from comparison (see 740).
        callee_thiscall!(7, u32, 0);
        0
    }
});
