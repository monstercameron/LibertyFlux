// original: 0x00ae2370 ui_grid120_plot
/// Plot a point into the 120-wide cell grid and grow the bounding box.
///
/// Same shape as the 48-wide plotter with bound `0x77`, row stride 120, the
/// signed bounding box at `+0xE4C8` and the per-row signed minima/maxima at
/// `+0xE108` / `+0xE2E8`. Returns the row maximum written.
export!(thiscall, rw_00ae2370(obj: u32, x: u32, y: u32, bits: u32) -> u32 {
    unsafe {
        let bx = (obj as *const u32).read();
        let by = ((obj + 4) as *const u32).read();
        let gx = (x.wrapping_sub(bx) as i32).clamp(0, 0x77) as u32;
        let gy = (y.wrapping_sub(by) as i32).clamp(0, 0x77) as u32;
        let cellp = obj.wrapping_add((gy * 120 + gx) * 4 + 8) as *mut u32;
        cellp.write(cellp.read() | bits);
        let ax = gx.wrapping_add(bx);
        let ay = gy.wrapping_add(by);
        let minxp = (obj + 0xE4C8) as *mut u32;
        minxp.write((minxp.read() as i32).min(ax as i32) as u32);
        let maxxp = (obj + 0xE4CC) as *mut u32;
        maxxp.write((maxxp.read() as i32).max(ax as i32) as u32);
        let minyp = (obj + 0xE4D0) as *mut u32;
        minyp.write((minyp.read() as i32).min(ay as i32) as u32);
        let maxyp = (obj + 0xE4D4) as *mut u32;
        maxyp.write((maxyp.read() as i32).max(ay as i32) as u32);
        let rminp = obj.wrapping_add(gy * 4 + 0xE108) as *mut u32;
        rminp.write((rminp.read() as i32).min(ax as i32) as u32);
        let rmaxp = obj.wrapping_add(gy * 4 + 0xE2E8) as *mut u32;
        let r = (rmaxp.read() as i32).max(ax as i32) as u32;
        rmaxp.write(r);
        r
    }
});
