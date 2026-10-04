// original: 0x009fc110 CPlayStatSessionInfo::~CPlayStatSessionInfo

/// Encode a session payload with a byte key and send it through the socket.
///
/// Takes seven cdecl stack arguments: `sock` (socket handle, passed to the
/// poll and send calls), `session` (session object; its header pointer lives
/// at `HDR_OFF`, the payload is read from its start), `param` and `tag`
/// (stored into the header; `tag` is also the success return value),
/// `key_full` (low byte is the XOR key), `counter` (pointer to a sent-bytes
/// accumulator, nonzero selects the bare encoding), and `flags_full` (a
/// nonzero low byte selects polled mode and mode 4 for the open call).
///
/// Sequence: the header's words take `param` and `tag`; when the header's
/// `HDR_AVAIL` word is zero it is set to `PAYLOAD_MAX` and the header is
/// linked to the session. Callee 0 opens the transfer (cdecl, mode and
/// header); a negative answer fails. A nonzero `HDR_AVAIL` with a zero flags
/// byte returns `tag` at once with nothing sent. Otherwise `PAYLOAD_MAX`
/// minus `HDR_AVAIL` payload bytes are XOR-encoded with the key into a frame
/// message: with a zero counter first, a three-byte header (`HDR_MAGIC`,
/// key) is emitted and the payload follows it, and only the message length
/// is then sent, so the header displaces the last three payload bytes.
///
/// The send loop runs while bytes remain and the status byte is set: callee 1
/// polls the socket (cdecl, `sock` and `POLL_ARG`); a zero answer clears the
/// status, a negative one clears it and runs callee 2 (cdecl, no arguments).
/// Otherwise callee 3 sends from the message cursor through the import slot
/// (stdcall, `sock`, cursor, remaining, 0): a positive answer advances the
/// cursor and the counter, any other answer runs callee 2 and clears the
/// status unless it reports `RETRY_CODE`. The outer pass repeats only if the
/// header word reads zero afterwards with the status still set (dead under
/// the callee stubs, which never zero it). Returns `tag` on success,
/// all-ones on failure. Callee 4 is the stack-cookie check (no arguments,
/// registers preserved).
lf_checker_rt::export!(
    cdecl,
    rw_009fc110(
        sock: u32,
        session: u32,
        param: u32,
        tag: u32,
        key_full: u32,
        counter: u32,
        flags_full: u32
    ) -> u32 {
        unsafe {
            const HDR_OFF: u32 = 0x400;
            const HDR_AVAIL: u32 = 0x10;
            const HDR_LINK: u32 = 0x0C;
            const PAYLOAD_MAX: u32 = 0x400;
            const HDR_MAGIC: u16 = 0x0100;
            const POLL_ARG: u32 = 0x1B58;
            const RETRY_CODE: u32 = 0x2733;
            const SEND_SLOT: u32 = 0x00E7_34D4;

            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            #[inline(always)]
            unsafe fn wr32(a: u32, v: u32) {
                unsafe { (a as *mut u32).write_unaligned(v) }
            }

            let key = (key_full & 0xFF) as u8;
            let flags = (flags_full & 0xFF) as u8;
            let mut msg = [0u8; 0x410];
            let mut ok = true;

            let hdr = rd32(session.wrapping_add(HDR_OFF));
            wr32(hdr.wrapping_add(4), tag);
            let hdr = rd32(session.wrapping_add(HDR_OFF));
            wr32(hdr, param);
            let mode = if flags != 0 { 4 } else { 0 };
            'outer: loop {
                let h = rd32(session.wrapping_add(HDR_OFF));
                if rd32(h.wrapping_add(HDR_AVAIL)) == 0 {
                    wr32(h.wrapping_add(HDR_AVAIL), PAYLOAD_MAX);
                    let h2 = rd32(session.wrapping_add(HDR_OFF));
                    wr32(h2.wrapping_add(HDR_LINK), session);
                }
                // Push order is (mode, header), so the header is argument 0.
                let opened: u32 = lf_checker_rt::callee_cdecl!(0, u32, rd32(session.wrapping_add(HDR_OFF)), mode);
                if (opened as i32) < 0 {
                    ok = false;
                    break 'outer;
                }
                let avail = rd32(rd32(session.wrapping_add(HDR_OFF)).wrapping_add(HDR_AVAIL));
                if avail != 0 && flags == 0 {
                    break 'outer;
                }
                let total = PAYLOAD_MAX.wrapping_sub(avail);
                let base: usize = if rd32(counter) != 0 {
                    0
                } else {
                    msg[0] = (HDR_MAGIC & 0xFF) as u8;
                    msg[1] = (HDR_MAGIC >> 8) as u8;
                    msg[2] = key;
                    3
                };
                if (total as i32) > 0 {
                    for i in 0..total {
                        let b = (session.wrapping_add(i) as *const u8).read();
                        msg[base.wrapping_add(i as usize)] = b ^ key;
                    }
                }
                let send_slot = lf_checker_rt::relocated(SEND_SLOT) as *const u32;
                let send: extern "stdcall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(send_slot.read() as usize);
                let mut cursor: usize = 0;
                let mut rem = total;
                if rem != 0 {
                    loop {
                        if !ok {
                            break;
                        }
                        let ready: u32 = lf_checker_rt::callee_cdecl!(1, u32, sock, POLL_ARG);
                        if ready == 0 {
                            ok = false;
                        } else if (ready as i32) < 0 {
                            ok = false;
                            let _: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
                        } else {
                            let at = (msg.as_mut_ptr() as u32).wrapping_add(cursor as u32);
                            let n: u32 = send(sock, at, rem, 0);
                            if (n as i32) > 0 {
                                rem = rem.wrapping_sub(n);
                                cursor = cursor.wrapping_add(n as usize);
                                wr32(counter, rd32(counter).wrapping_add(n));
                            } else {
                                let err: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
                                if err != RETRY_CODE {
                                    ok = false;
                                }
                            }
                        }
                        if rem == 0 {
                            break;
                        }
                    }
                }
                let hx = rd32(session.wrapping_add(HDR_OFF));
                if rd32(hx.wrapping_add(HDR_AVAIL)) == 0 && ok {
                    continue 'outer;
                }
                break 'outer;
            }
            let _: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
            if ok { tag } else { 0xFFFF_FFFF }
        }
    }
);
