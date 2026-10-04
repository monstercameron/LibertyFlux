// original: 0x0094f7d0 init_pool_dims
/// Initialise a pool descriptor with aligned dimensions.
///
/// Rounds both dimensions up to a multiple of 4 (wrapping), then writes the
/// three fixed capacity constants. Returns the descriptor pointer.
export!(thiscall, rw_0094f7d0(dims: *mut u32, w: u32, h: u32) -> u32 {
    unsafe {
        *dims = w.wrapping_add(3) & !3;
        *dims.add(1) = h.wrapping_add(3) & !3;
        *dims.add(2) = 0x1000;
        *dims.add(3) = 0x0BB8;
        *dims.add(4) = 0x1E;
        dims as u32
    }
});
