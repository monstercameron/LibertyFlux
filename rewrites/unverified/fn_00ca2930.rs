// original: 0x00CA2930 face_tex_load
/// Select a JPEG loader from the path suffix, decode a texture, apply positive
/// requested dimensions, and submit it through the renderer's global object.
/// A successful submission releases the previous face object and stores the
/// resulting texture status through the face-slot helper. A non-JPEG path uses
/// the alternate loader. Texture and face reference counts follow the original
/// decrement and release conditions.
///
/// The contract intercepts the string tests, loaders, resize operation,
/// renderer callback, release helpers, and face-slot helper. Their internal
/// behavior is outside this function's proof; the calls, arguments, returns,
/// direct memory changes, and input branches are compared.
lf_checker_rt::export!(thiscall, rw_00ca2930(
    this: u32,
    path: u32,
    width: u32,
    height: u32,
) -> u32 {
    unsafe {
        const FACE_SLOT: u32 = 0x18;
        const FACE_KIND: u32 = 0x08;
        const FACE_REFS: u32 = 0x0A;
        const TEXTURE_REFS: u32 = 0x20;
        const RENDERER_GLOBAL_VA: u32 = 0x017F_5630;
        const JPEG_SUFFIX_VA: u32 = 0x00ED_70DC;
        const JPEG_LONG_SUFFIX_VA: u32 = 0x00ED_70E4;

        let has_short_suffix = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            path,
            lf_checker_rt::relocated(JPEG_SUFFIX_VA),
        );

        let mut branch_status;
        let mut return_value;
        if has_short_suffix != 0 {
            let texture = lf_checker_rt::callee_cdecl!(4, u32, path, 0);
            if texture == 0 {
                return 0;
            }

            if (width as i32) > 0 && (height as i32) > 0 {
                let _ = lf_checker_rt::callee_thiscall!(5, u32, texture, width, height);
            }

            let renderer = *lf_checker_rt::global::<u32>(RENDERER_GLOBAL_VA);
            branch_status = lf_checker_rt::callee_thiscall!(
                6,
                u32,
                renderer,
                texture,
                0,
            );
            return_value = branch_status;

            let texture_refs = (texture.wrapping_add(TEXTURE_REFS) as *const u32)
                .read_unaligned()
                .wrapping_sub(1);
            (texture.wrapping_add(TEXTURE_REFS) as *mut u32)
                .write_unaligned(texture_refs);
            if texture_refs == 0 {
                let _ = lf_checker_rt::callee_thiscall!(7, u32, texture);
                return_value = lf_checker_rt::callee_cdecl!(8, u32, texture);
            }
        } else {
            let has_long_suffix = lf_checker_rt::callee_cdecl!(
                2,
                u32,
                path,
                lf_checker_rt::relocated(JPEG_LONG_SUFFIX_VA),
            );
            if has_long_suffix != 0 {
                let texture = lf_checker_rt::callee_cdecl!(4, u32, path, 0);
                if texture == 0 {
                    return 0;
                }

                if (width as i32) > 0 && (height as i32) > 0 {
                    let _ = lf_checker_rt::callee_thiscall!(5, u32, texture, width, height);
                }

                let renderer = *lf_checker_rt::global::<u32>(RENDERER_GLOBAL_VA);
                branch_status = lf_checker_rt::callee_thiscall!(
                    6,
                    u32,
                    renderer,
                    texture,
                    0,
                );
                return_value = branch_status;

                let texture_refs = (texture.wrapping_add(TEXTURE_REFS) as *const u32)
                    .read_unaligned()
                    .wrapping_sub(1);
                (texture.wrapping_add(TEXTURE_REFS) as *mut u32)
                    .write_unaligned(texture_refs);
                if texture_refs == 0 {
                    let _ = lf_checker_rt::callee_thiscall!(7, u32, texture);
                    return_value = lf_checker_rt::callee_cdecl!(8, u32, texture);
                }
            } else {
                branch_status = lf_checker_rt::callee_cdecl!(3, u32, path);
                return_value = branch_status;
            }
        }

        if branch_status == 0 {
            return_value
        } else {
            let face = (this.wrapping_add(FACE_SLOT) as *const u32).read_unaligned();
            if face != 0 {
                let face_refs = (face.wrapping_add(FACE_REFS) as *const u16)
                    .read_unaligned();
                if face_refs != 0 {
                    let next_refs = face_refs.wrapping_sub(1);
                    (face.wrapping_add(FACE_REFS) as *mut u16).write_unaligned(next_refs);
                    let kind = (face.wrapping_add(FACE_KIND) as *const u8).read_unaligned();
                    if next_refs == 0 && (kind == 2 || kind == 4) {
                        let _ = lf_checker_rt::callee_thiscall!(9, u32, face, 1);
                    }
                }
                (this.wrapping_add(FACE_SLOT) as *mut u32).write_unaligned(0);
            }
            lf_checker_rt::callee_thiscall!(10, u32, this, 2, branch_status)
        }
    }
});
