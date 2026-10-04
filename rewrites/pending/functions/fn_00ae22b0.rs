// original: 0x00ae22b0 ui_grid48_plot
/// Plot a point into the 48-wide cell grid and grow the bounding box.
///
/// Clamps the point, relative to the grid origin at `+0/+4`, into `[0, 47]`
/// on both axes, ORs the bit mask into the cell word, then expands the signed
/// bounding box at `+0x2588` and the per-row signed minima/maxima at `+0x2408`
/// / `+0x24C8` to include the absolute point. Returns the row maximum written.
export!(thiscall, rw_00ae22b0(obj: u32, x: u32, y: u32, bits: u32) -> u32 {
    unsafe {
        let bx = (obj as *const u32).read();
        let by = ((obj + 4) as *const u32).read();
        let gx = (x.wrapping_sub(bx) as i32).clamp(0, 0x2F) as u32;
        let gy = (y.wrapping_sub(by) as i32).clamp(0, 0x2F) as u32;
        let cellp = obj.wrapping_add((gy * 48 + gx) * 4 + 8) as *mut u32;
        cellp.write(cellp.read() | bits);
        let ax = gx.wrapping_add(bx);
        let ay = gy.wrapping_add(by);
        let minxp = (obj + 0x2588) as *mut u32;
        minxp.write((minxp.read() as i32).min(ax as i32) as u32);
        let maxxp = (obj + 0x258C) as *mut u32;
        maxxp.write((maxxp.read() as i32).max(ax as i32) as u32);
        let minyp = (obj + 0x2590) as *mut u32;
        minyp.write((minyp.read() as i32).min(ay as i32) as u32);
        let maxyp = (obj + 0x2594) as *mut u32;
        maxyp.write((maxyp.read() as i32).max(ay as i32) as u32);
        let rminp = obj.wrapping_add(gy * 4 + 0x2408) as *mut u32;
        rminp.write((rminp.read() as i32).min(ax as i32) as u32);
        let rmaxp = obj.wrapping_add(gy * 4 + 0x24C8) as *mut u32;
        let r = (rmaxp.read() as i32).max(ax as i32) as u32;
        rmaxp.write(r);
        r
    }
});
