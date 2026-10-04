// original: 0x005e7060 CELL_CAM_SET_ZOOM
/// Script native handler `CELL_CAM_SET_ZOOM` (hash 0x087C5347).
///
/// Looks up the cell-camera object through the camera manager, then stores
/// the requested zoom (kept as raw float bits) into the object and into the
/// global zoom mirror. Either lookup can miss, in which case only the global
/// mirror is written. The zoom is only ever spilled and stored, never passed
/// to a callee: the first lookup takes no stack arguments at all.
export!(cdecl, rw_005e7060(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let zoom = *args;
        let mgr = relocated(0x0161547C);
        let found = callee_thiscall!(1, u32, mgr);
        if found == 0xFFFFFFFF {
            *global::<u32>(0x0106C31C) = zoom;
            return found;
        }
        let cam = callee_thiscall!(2, u32, mgr, found);
        if cam == 0 {
            *global::<u32>(0x0106C31C) = zoom;
            return 0;
        }
        *((cam + 0x94C) as *mut u32) = zoom;
        *global::<u32>(0x0106C31C) = zoom;
        cam
    }
});
