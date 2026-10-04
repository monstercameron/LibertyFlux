//! A minimal PNG encoder: 8-bit RGBA, no compression.
//!
//! The file is the PNG signature followed by three chunks:
//!
//! - `IHDR`: width, height, bit depth 8, colour type 6 (RGBA), compression
//!   method 0, filter method 0, no interlace.
//! - `IDAT`: one zlib stream holding every scanline, each prefixed by filter
//!   type 0 (none). The zlib stream uses deflate *stored* blocks (block type
//!   0, at most 65,535 bytes each), so it is valid zlib that any decoder
//!   reads, at the cost of no size reduction. Compression is not the point:
//!   the files are for looking at, locally.
//! - `IEND`.
//!
//! Every chunk carries the CRC-32 (ISO 3309 polynomial, reflected,
//! `0xEDB88320`) of its type and data; the zlib stream ends with the
//! Adler-32 of the uncompressed data. Both are implemented here, bit by bit
//! for CRC-32 with a table built at compile time.

/// The eight-byte PNG signature.
pub const SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

/// IHDR colour type for 8-bit RGBA.
pub const COLOR_TYPE_RGBA: u8 = 6;

/// Largest payload of one deflate stored block.
pub const MAX_STORED_BLOCK: usize = 65_535;

/// zlib header for deflate with a 32 KiB window and no preset dictionary,
/// "fastest" level bits; `0x7801` is a multiple of 31 as the format needs.
pub const ZLIB_HEADER: [u8; 2] = [0x78, 0x01];

/// Largest width or height PNG allows (2^31 - 1).
pub const MAX_DIMENSION: u32 = 0x7FFF_FFFF;

/// Reflected CRC-32 polynomial.
const CRC_POLY: u32 = 0xEDB8_8320;

/// Adler-32 modulus: the largest prime below 2^16.
const ADLER_MOD: u32 = 65_521;

/// CRC-32 lookup table, one entry per byte value.
const CRC_TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let mut n = 0;
    while n < 256 {
        #[allow(clippy::cast_possible_truncation)] // n < 256
        let mut c = n as u32;
        let mut k = 0;
        while k < 8 {
            c = if c & 1 != 0 {
                CRC_POLY ^ (c >> 1)
            } else {
                c >> 1
            };
            k += 1;
        }
        table[n] = c;
        n += 1;
    }
    table
};

/// Feeds bytes into a running CRC-32 (start from `0xFFFF_FFFF`, finish by
/// inverting).
fn crc32_update(mut crc: u32, bytes: &[u8]) -> u32 {
    for &b in bytes {
        crc = CRC_TABLE[((crc ^ u32::from(b)) & 0xFF) as usize] ^ (crc >> 8);
    }
    crc
}

/// The CRC-32 of `bytes`, as PNG and zip use it.
#[must_use]
pub fn crc32(bytes: &[u8]) -> u32 {
    !crc32_update(0xFFFF_FFFF, bytes)
}

/// The Adler-32 checksum of `bytes`, as zlib uses it.
#[must_use]
pub fn adler32(bytes: &[u8]) -> u32 {
    // 5552 is the most bytes that can be summed before the 32-bit sums can
    // overflow, so the modulus is taken once per chunk of that size.
    const NMAX: usize = 5552;
    let (mut a, mut b) = (1u32, 0u32);
    for chunk in bytes.chunks(NMAX) {
        for &byte in chunk {
            a += u32::from(byte);
            b += a;
        }
        a %= ADLER_MOD;
        b %= ADLER_MOD;
    }
    (b << 16) | a
}

/// Wraps `data` in a zlib stream of deflate stored blocks.
#[must_use]
pub fn zlib_stored(data: &[u8]) -> Vec<u8> {
    let blocks = data.len().div_ceil(MAX_STORED_BLOCK).max(1);
    let mut out = Vec::with_capacity(data.len() + blocks * 5 + 6);
    out.extend_from_slice(&ZLIB_HEADER);
    let mut chunks = data.chunks(MAX_STORED_BLOCK).peekable();
    if chunks.peek().is_none() {
        // An empty input still needs one final (empty) block.
        out.extend_from_slice(&[1, 0, 0, 0xFF, 0xFF]);
    }
    while let Some(chunk) = chunks.next() {
        let last = chunks.peek().is_none();
        // Block header byte: BFINAL in bit 0, BTYPE 00 (stored); the block
        // then starts on a byte boundary with LEN and its complement NLEN.
        out.push(u8::from(last));
        let len = u16::try_from(chunk.len()).unwrap_or(u16::MAX);
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&(!len).to_le_bytes());
        out.extend_from_slice(chunk);
    }
    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

/// Appends one chunk: length, type, data, CRC of type and data.
fn push_chunk(out: &mut Vec<u8>, kind: [u8; 4], data: &[u8]) {
    let len = u32::try_from(data.len()).unwrap_or(u32::MAX);
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(&kind);
    out.extend_from_slice(data);
    let crc = !crc32_update(crc32_update(0xFFFF_FFFF, &kind), data);
    out.extend_from_slice(&crc.to_be_bytes());
}

/// Why an image cannot be encoded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PngError {
    /// Width or height is zero or above [`MAX_DIMENSION`].
    BadDimensions {
        /// Requested width.
        width: u32,
        /// Requested height.
        height: u32,
    },
    /// `pixels` is not `width * height * 4` bytes.
    WrongLength {
        /// Bytes expected.
        expected: u64,
        /// Bytes given.
        got: usize,
    },
    /// The image is too large for one IDAT chunk (4 GiB).
    TooLarge,
}

impl std::fmt::Display for PngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PngError::BadDimensions { width, height } => {
                write!(f, "cannot encode a {width}x{height} PNG")
            }
            PngError::WrongLength { expected, got } => {
                write!(f, "pixel buffer is {got} bytes, expected {expected}")
            }
            PngError::TooLarge => f.write_str("image too large for one PNG data chunk"),
        }
    }
}

impl std::error::Error for PngError {}

/// Encodes row-major RGBA8 pixels (top row first) as a PNG file.
///
/// # Errors
///
/// Returns [`PngError`] for zero or oversized dimensions, a pixel buffer of
/// the wrong length, or an image whose data exceeds one chunk.
pub fn encode_rgba8(width: u32, height: u32, pixels: &[u8]) -> Result<Vec<u8>, PngError> {
    if width == 0 || height == 0 || width > MAX_DIMENSION || height > MAX_DIMENSION {
        return Err(PngError::BadDimensions { width, height });
    }
    let row = u64::from(width) * 4;
    let expected = row * u64::from(height);
    if pixels.len() as u64 != expected {
        return Err(PngError::WrongLength {
            expected,
            got: pixels.len(),
        });
    }
    let row = usize::try_from(row).map_err(|_| PngError::TooLarge)?;
    let mut raw = Vec::with_capacity(pixels.len() + height as usize);
    for line in pixels.chunks_exact(row) {
        raw.push(0); // filter type 0: none
        raw.extend_from_slice(line);
    }
    let idat = zlib_stored(&raw);
    if u32::try_from(idat.len()).is_err() {
        return Err(PngError::TooLarge);
    }
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, COLOR_TYPE_RGBA, 0, 0, 0]);
    let mut out = Vec::with_capacity(idat.len() + 64);
    out.extend_from_slice(&SIGNATURE);
    push_chunk(&mut out, *b"IHDR", &ihdr);
    push_chunk(&mut out, *b"IDAT", &idat);
    push_chunk(&mut out, *b"IEND", &[]);
    Ok(out)
}

// Tests compare exactly representable floats and index small fixtures.
#[cfg(test)]
#[allow(clippy::float_cmp, clippy::cast_possible_truncation)]
mod tests {
    use super::*;
    use std::io::Read;

    /// One parsed chunk: type, data, and whether its CRC checked out.
    struct Chunk {
        kind: [u8; 4],
        data: Vec<u8>,
        crc_ok: bool,
    }

    /// Splits a PNG into chunks, checking the signature and every CRC.
    fn parse_png(bytes: &[u8]) -> Vec<Chunk> {
        assert_eq!(&bytes[..8], &SIGNATURE, "signature");
        let mut at = 8;
        let mut chunks = Vec::new();
        while at < bytes.len() {
            let len = u32::from_be_bytes(bytes[at..at + 4].try_into().unwrap()) as usize;
            let kind: [u8; 4] = bytes[at + 4..at + 8].try_into().unwrap();
            let data = bytes[at + 8..at + 8 + len].to_vec();
            let stored = u32::from_be_bytes(bytes[at + 8 + len..at + 12 + len].try_into().unwrap());
            let crc_ok = stored == crc32(&bytes[at + 4..at + 8 + len]);
            chunks.push(Chunk { kind, data, crc_ok });
            at += 12 + len;
        }
        assert_eq!(at, bytes.len(), "no trailing bytes");
        chunks
    }

    /// Our own reader of a stored-block zlib stream, checking every field.
    fn inflate_stored(z: &[u8]) -> Vec<u8> {
        let cmf = z[0];
        let flg = z[1];
        assert_eq!(cmf & 0x0F, 8, "deflate");
        assert_eq!((u16::from(cmf) << 8 | u16::from(flg)) % 31, 0, "FCHECK");
        assert_eq!(flg & 0x20, 0, "no preset dictionary");
        let mut at = 2;
        let mut out = Vec::new();
        loop {
            let header = z[at];
            assert_eq!(header >> 1, 0, "stored block type");
            let len = u16::from_le_bytes([z[at + 1], z[at + 2]]);
            let nlen = u16::from_le_bytes([z[at + 3], z[at + 4]]);
            assert_eq!(len, !nlen, "NLEN is the complement of LEN");
            out.extend_from_slice(&z[at + 5..at + 5 + len as usize]);
            at += 5 + len as usize;
            if header & 1 == 1 {
                break;
            }
        }
        let adler = u32::from_be_bytes(z[at..at + 4].try_into().unwrap());
        assert_eq!(adler, adler32(&out), "Adler-32 trailer");
        assert_eq!(at + 4, z.len(), "stream ends after the trailer");
        out
    }

    #[test]
    fn checksums_match_published_check_values() {
        // The standard CRC-32 check value, and the worked Adler-32 example
        // that reference pages use.
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
        assert_eq!(crc32(b""), 0);
        assert_eq!(adler32(b"Wikipedia"), 0x11E6_0398);
        assert_eq!(adler32(b""), 1);
        // The CRC of an IEND chunk (type only) is the well-known AE426082.
        assert_eq!(crc32(b"IEND"), 0xAE42_6082);
        // Long input exercises the deferred modulus.
        let big = vec![0xFFu8; 100_000];
        let (mut a, mut b) = (1u64, 0u64);
        for &x in &big {
            a = (a + u64::from(x)) % 65_521;
            b = (b + a) % 65_521;
        }
        assert_eq!(u64::from(adler32(&big)), (b << 16) | a);
    }

    #[test]
    fn header_chunks_and_crcs() {
        let pixels: Vec<u8> = (0..3 * 2 * 4)
            .map(|i| u8::try_from(i * 10).unwrap())
            .collect();
        let png = encode_rgba8(3, 2, &pixels).unwrap();
        let chunks = parse_png(&png);
        let kinds: Vec<&[u8; 4]> = chunks.iter().map(|c| &c.kind).collect();
        assert_eq!(kinds, vec![b"IHDR", b"IDAT", b"IEND"]);
        assert!(chunks.iter().all(|c| c.crc_ok));
        let ihdr = &chunks[0].data;
        assert_eq!(ihdr.len(), 13);
        assert_eq!(u32::from_be_bytes(ihdr[0..4].try_into().unwrap()), 3);
        assert_eq!(u32::from_be_bytes(ihdr[4..8].try_into().unwrap()), 2);
        assert_eq!(&ihdr[8..13], &[8, COLOR_TYPE_RGBA, 0, 0, 0]);
        assert!(chunks[2].data.is_empty());
        // Scanlines: a zero filter byte then the row's pixels.
        let raw = inflate_stored(&chunks[1].data);
        assert_eq!(raw.len(), 2 * (1 + 12));
        assert_eq!(raw[0], 0);
        assert_eq!(&raw[1..13], &pixels[..12]);
        assert_eq!(raw[13], 0);
        assert_eq!(&raw[14..], &pixels[12..]);
    }

    #[test]
    fn independent_decoder_reads_the_stream() {
        // 200 x 100 RGBA is 80,100 bytes of scanlines: two stored blocks.
        let (w, h) = (200u32, 100u32);
        let pixels: Vec<u8> = (0..w * h * 4).map(|i| (i % 251) as u8).collect();
        let png = encode_rgba8(w, h, &pixels).unwrap();
        let idat = &parse_png(&png)[1].data;
        let mut inflated = Vec::new();
        flate2::read::ZlibDecoder::new(&idat[..])
            .read_to_end(&mut inflated)
            .unwrap();
        assert_eq!(inflated, inflate_stored(idat));
        assert_eq!(inflated.len(), (h * (1 + w * 4)) as usize);
        for (y, line) in inflated.chunks_exact((1 + w * 4) as usize).enumerate() {
            assert_eq!(line[0], 0);
            let start = y * (w * 4) as usize;
            assert_eq!(&line[1..], &pixels[start..start + (w * 4) as usize]);
        }
    }

    #[test]
    fn zlib_edge_cases() {
        let empty = zlib_stored(&[]);
        assert_eq!(inflate_stored(&empty), Vec::<u8>::new());
        let exact = vec![7u8; MAX_STORED_BLOCK];
        assert_eq!(inflate_stored(&zlib_stored(&exact)), exact);
        let over = vec![9u8; MAX_STORED_BLOCK + 1];
        let z = zlib_stored(&over);
        assert_eq!(inflate_stored(&z), over);
        let mut check = Vec::new();
        flate2::read::ZlibDecoder::new(&z[..])
            .read_to_end(&mut check)
            .unwrap();
        assert_eq!(check, over);
    }

    #[test]
    fn bad_inputs_are_rejected() {
        assert_eq!(
            encode_rgba8(0, 1, &[]),
            Err(PngError::BadDimensions {
                width: 0,
                height: 1
            })
        );
        assert_eq!(
            encode_rgba8(2, 2, &[0; 15]),
            Err(PngError::WrongLength {
                expected: 16,
                got: 15
            })
        );
        assert!(encode_rgba8(MAX_DIMENSION + 1, 1, &[]).is_err());
    }
}
