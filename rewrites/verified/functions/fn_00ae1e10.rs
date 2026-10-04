// original: 0x00ae1e10 ui_cell_commit
/// Build a cell object from four arguments and commit its hashed field.
///
/// Allocates a scratch block, constructs the cell through the builder (which
/// receives the block and the four arguments unchanged), then calls the cell's
/// virtual slot twice: the first answer picks a residue class `e`, the second
/// is added to it, divided by 16 and shifted into bits `0x1FFC000` of the
/// cell's word at `+4` (xor-masked). A null block faults on both sides.
/// Returns the masked value written.
export!(cdecl, rw_00ae1e10(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        let blk = callee_cdecl!(1, u32, 0x60, 0);
        let obj = if blk == 0 {
            0
        } else {
            callee_thiscall!(2, u32, blk, a0, a1, a2, a3)
        };
        let vt = (obj as *const u32).read();
        let slot = ((vt + 8) as *const u32).read();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let r1 = f(obj) as i32;
        let e = (16 - (r1 % 16)) % 16;
        let r2 = f(obj) as i32;
        let t = r2.wrapping_add(e);
        let v = ((t / 16) as u32).wrapping_mul(16384);
        let fldp = (obj + 4) as *mut u32;
        let m = (fldp.read() ^ v) & 0x1FFC000;
        fldp.write(fldp.read() ^ m);
        m
    }
});
