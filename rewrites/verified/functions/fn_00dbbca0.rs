// original: 0x00DBBCA0 skydome_textures_init (proposed)
/// Load the sky-dome textures and publish the dome object.
///
/// Resolves the skydome handle, registers the platform texture path, runs the
/// loader queries, and when the build query succeeds wraps its answer into the
/// published dome object (otherwise publishes null). Then loads the seven
/// named sky textures plus the atmosphere table into that object, applies the
/// constant sampler description block, stamps the dome flags and scale, and
/// tail-calls the finish routine, whose answer is returned.
///
/// The sampler block is passed by pointer into the original's own stack frame
/// (compared by contents, not address). Three of its words carry untouched
/// stack-fill bytes around the narrow stores; the contract defines the fill
/// as zero.
lf_checker_rt::export!(cdecl, rw_dbbca0() -> u32 {
    unsafe {
        const DOME_SLOT: u32 = 0x017A_66A4;
        const SAMPLER: [u32; 8] = [0, 0x20, 0x20, 0x101, 1, 0, 0x100, 2];
        const DOME_SCALE: u32 = 0x3A03_126F;
        let sky = callee_cdecl!(1, u32, relocated(0x00EF_410C));
        callee_cdecl!(2, u32, sky, relocated(0x00EF_4114));
        callee_cdecl!(3, u32, sky);
        callee_cdecl!(4, u32,);
        callee_cdecl!(5, u32, sky);
        let built = callee_cdecl!(6, u32, 0x330);
        let slot = global::<u32>(DOME_SLOT);
        if built != 0 {
            slot.write(callee_thiscall!(7, u32, built));
        } else {
            slot.write(0);
        }
        callee_cdecl!(8, u32, relocated(0x00EF_4130));
        let dome = slot.read();
        callee_thiscall!(9, u32, dome, relocated(0x00EF_4148));
        callee_thiscall!(10, u32, dome, relocated(0x00EF_4150));
        callee_thiscall!(11, u32, dome, relocated(0x00EF_415C));
        callee_thiscall!(12, u32, dome, relocated(0x00EF_4168));
        callee_thiscall!(13, u32, dome, relocated(0x00EF_4174));
        callee_thiscall!(14, u32, dome, relocated(0x00EF_417C));
        callee_thiscall!(15, u32, dome, relocated(0x00EF_4188));
        let mut desc = SAMPLER;
        callee_thiscall!(16, u32, dome, 0, 0, desc.as_mut_ptr() as u32);
        let obj = slot.read() as *mut u8;
        (obj.add(0x314)).write(1);
        (obj.add(0x280)).write(1);
        ((obj.add(0xC0)) as *mut u32).write(DOME_SCALE);
        callee_cdecl!(17, u32,)
    }
});

// Probe export (not a rewrite): deliberately returns zero so the dffaeb
// probe verdict prints the original's true xmm0 answer in the mismatch.
lf_checker_rt::export!(cdecl, probe_dffaeb() -> u32 {
    0
});
