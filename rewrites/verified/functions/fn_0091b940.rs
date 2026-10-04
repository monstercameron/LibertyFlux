// original: 0x0091B940 copy_filtered_markers
/// Fill a scratch buffer through the loader helper, then copy it over the
/// argument with marker regions stripped.
///
/// Calls the loader with `(buffer, arg, -1)`, then walks the buffer's wide
/// characters: plain characters are stored to the argument in place, while a
/// `0x7E` or `0x807E` marker opens a skip that consumes characters until the
/// closing `0x7E`/`0x807E`. The destination is always NUL-terminated
/// (including the empty-buffer case) and the function returns the trailing
/// stack-cookie check call's answer.
export!(cdecl, rw_0091b940(a0: u32) -> u32 {
    unsafe {
        let mut buf = [0u16; 32];
        let _: u32 = callee_cdecl!(1, u32, buf.as_mut_ptr() as u32, a0, 0xFFFFFFFF);
        let d = a0 as *mut u16;
        let mut cx = buf[0];
        if cx != 0 {
            let mut idx: usize = 0;
            let mut edx: usize = 0;
            loop {
                if cx != 0x7E && cx != 0x807E {
                    *d.add(edx) = cx;
                    edx += 1;
                } else {
                    idx += 1;
                    cx = buf[idx];
                    if cx != 0x7E {
                        loop {
                            if cx == 0x807E {
                                break;
                            }
                            idx += 1;
                            cx = buf[idx];
                            if cx == 0x7E {
                                break;
                            }
                        }
                    }
                }
                idx += 1;
                cx = buf[idx];
                if cx == 0 {
                    *d.add(edx) = 0;
                    break;
                }
            }
        } else {
            *d = 0;
        }
        callee_cdecl!(2, u32,)
    }
});
