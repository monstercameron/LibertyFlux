//! Synthetic fixtures for `lf-inspect bench synthetic`: well-formed files
//! of each benchmarked format, generated in memory from a fixed seed. No
//! byte comes from the game; the shapes follow the format crates' own
//! documentation. Sizes are chosen so a full run stays within CI time
//! limits even in a debug build.

/// Magic of a PC RSC5 resource ("RSC" + 0x05, little-endian).
const RSC5_MAGIC: u32 = 0x0543_5352;
/// Largest data length of one stored deflate block.
const STORED_BLOCK_MAX: usize = 0xFFFF;
/// Modulus of the Adler-32 checksum.
const ADLER_MOD: u32 = 65_521;
/// RPF table offset.
const RPF_TOC_OFFSET: usize = 0x800;
/// IMG block size.
const IMG_BLOCK: usize = 0x800;

/// Small deterministic generator (xorshift64*); content only, not security.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        let [_, _, _, _, a, b, c, d] = self.0.wrapping_mul(0x2545_F491_4F6C_DD1D).to_le_bytes();
        u32::from_le_bytes([a, b, c, d])
    }

    /// The low byte of the next word.
    fn byte(&mut self) -> u8 {
        self.next().to_le_bytes()[0]
    }

    /// A value in `0..n` as a u16 (`n` at most 65536).
    fn below16(&mut self, n: u32) -> u16 {
        u16::try_from(self.next() % n).unwrap_or(0)
    }
}

/// Little-endian appends.
trait Put {
    fn u16le(&mut self, v: u16);
    fn u32le(&mut self, v: u32);
}

impl Put for Vec<u8> {
    fn u16le(&mut self, v: u16) {
        self.extend_from_slice(&v.to_le_bytes());
    }
    fn u32le(&mut self, v: u32) {
        self.extend_from_slice(&v.to_le_bytes());
    }
}

fn to_u32(n: usize) -> u32 {
    u32::try_from(n).expect("synthetic fixture sizes fit in u32")
}

fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for &x in data {
        a = (a + u32::from(x)) % ADLER_MOD;
        b = (b + a) % ADLER_MOD;
    }
    (b << 16) | a
}

/// A zlib stream (`78 DA` header) of stored deflate blocks.
fn zlib_stored(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x78, 0xDA];
    let mut chunks = data.chunks(STORED_BLOCK_MAX).peekable();
    if chunks.peek().is_none() {
        out.extend_from_slice(&[0x01, 0x00, 0x00, 0xFF, 0xFF]);
    }
    while let Some(chunk) = chunks.next() {
        out.push(u8::from(chunks.peek().is_none()));
        let n = u16::try_from(chunk.len()).expect("stored block fits in u16");
        out.u16le(n);
        out.u16le(!n);
        out.extend_from_slice(chunk);
    }
    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

/// An RSC5 resource with a system segment of `pages` x 4 KiB (exponent 4)
/// and no graphics segment.
#[must_use]
pub fn resource(pages: u16) -> Vec<u8> {
    let mut rng = Rng(0x5EED_0001);
    let sys_len = usize::from(pages) * 4096;
    let sys: Vec<u8> = (0..sys_len).map(|_| rng.byte()).collect();
    let flags = u32::from(pages) | (4 << 11) | (1 << 31);
    let mut file = Vec::new();
    file.u32le(RSC5_MAGIC);
    file.u32le(1);
    file.u32le(flags);
    file.extend_from_slice(&zlib_stored(&sys));
    file
}

/// An unencrypted RPF2 archive: a root directory holding `files` entries of
/// 64 bytes each.
#[must_use]
pub fn rpf2(files: usize) -> Vec<u8> {
    let count = files + 1;
    let mut names = b"/\0".to_vec();
    let mut name_offsets = Vec::with_capacity(files);
    for i in 0..files {
        name_offsets.push(to_u32(names.len()));
        names.extend_from_slice(format!("file{i}.bin\0").as_bytes());
    }
    while !names.len().is_multiple_of(16) {
        names.push(0);
    }
    let toc_size = count * 16 + names.len();
    let payload_at = (RPF_TOC_OFFSET + toc_size).next_multiple_of(RPF_TOC_OFFSET);
    let mut toc = Vec::with_capacity(toc_size);
    toc.u32le(0);
    toc.u32le(0);
    toc.u32le(0x8000_0001);
    toc.u32le(to_u32(files));
    for (i, name) in name_offsets.iter().enumerate() {
        toc.u32le(*name);
        toc.u32le(64);
        toc.u32le(to_u32(payload_at + i * 64));
        toc.u32le(64);
    }
    toc.extend_from_slice(&names);
    let mut file = b"RPF2".to_vec();
    file.u32le(to_u32(toc_size));
    file.u32le(to_u32(count));
    file.resize(RPF_TOC_OFFSET, 0);
    file.extend_from_slice(&toc);
    file.resize(payload_at + files * 64, 0x5A);
    file
}

/// An unencrypted IMG v3 archive with `files` one-block entries.
#[must_use]
pub fn img(files: usize) -> Vec<u8> {
    let mut names = Vec::new();
    for i in 0..files {
        names.extend_from_slice(format!("entry{i}.dat\0").as_bytes());
    }
    let table_size = files * 16 + names.len();
    let first_block = (20 + table_size).div_ceil(IMG_BLOCK);
    let mut file = Vec::new();
    file.u32le(0xA94E_2A52);
    file.u32le(3);
    file.u32le(to_u32(files));
    file.u32le(to_u32(table_size));
    file.u16le(16);
    file.u16le(0);
    for i in 0..files {
        file.u32le(100);
        file.u32le(0);
        file.u32le(to_u32(first_block + i));
        file.u16le(1);
        file.u16le(0);
    }
    file.extend_from_slice(&names);
    file.resize((first_block + files) * IMG_BLOCK, 0);
    file
}

/// A GXT database: one `MAIN` table with `entries` 16-bit strings.
#[must_use]
pub fn gxt(entries: usize) -> Vec<u8> {
    let mut rng = Rng(0x5EED_0002);
    let mut keys = Vec::new();
    let mut strings = Vec::new();
    for _ in 0..entries {
        keys.u32le(to_u32(strings.len()));
        keys.u32le(rng.next());
        for _ in 0..(8 + rng.next() % 24) {
            strings.u16le(0x41 + rng.below16(26));
        }
        strings.u16le(0);
    }
    let mut file = Vec::new();
    file.u16le(4);
    file.u16le(16);
    file.extend_from_slice(b"TABL");
    file.u32le(12);
    file.extend_from_slice(b"MAIN\0\0\0\0");
    file.u32le(24);
    file.extend_from_slice(b"TKEY");
    file.u32le(to_u32(keys.len()));
    file.extend_from_slice(&keys);
    file.extend_from_slice(b"TDAT");
    file.u32le(to_u32(strings.len()));
    file.extend_from_slice(&strings);
    file
}

/// A binary WPL placement file with `instances` `inst` records.
#[must_use]
pub fn wpl(instances: usize) -> Vec<u8> {
    let mut rng = Rng(0x5EED_0003);
    let mut file = Vec::new();
    file.u32le(3);
    file.u32le(to_u32(instances));
    for _ in 1..16 {
        file.u32le(0);
    }
    for _ in 0..instances {
        for _ in 0..12 {
            file.u32le(rng.next());
        }
    }
    file
}

/// A `.nod` path graph: `nodes` nodes with `links_per_node` links each.
#[must_use]
pub fn nod(nodes: usize, links_per_node: usize) -> Vec<u8> {
    let mut rng = Rng(0x5EED_0004);
    let links = nodes * links_per_node;
    let mut file = Vec::new();
    file.u32le(to_u32(nodes));
    file.u32le(to_u32(nodes));
    file.u32le(0);
    file.u32le(to_u32(links));
    for i in 0..nodes {
        file.u32le(0);
        file.u32le(0);
        file.u16le(1);
        file.u16le(u16::try_from(i % 0x1_0000).unwrap_or(0));
        file.u32le(rng.next());
        file.u16le(0);
        file.u16le(u16::try_from((i * links_per_node) % 0x1_0000).unwrap_or(0));
        for _ in 0..3 {
            file.u16le(rng.below16(0x1_0000));
        }
        file.push(8);
        file.push(0);
        file.u32le(rng.next());
    }
    for _ in 0..links {
        file.u16le(1);
        file.u16le(rng.below16(1000));
        file.push(10);
        file.push(0);
        file.u16le(0);
    }
    file
}

/// A plain (unencrypted) script holding `instructions` instructions.
#[must_use]
pub fn sco(instructions: usize) -> Vec<u8> {
    let mut rng = Rng(0x5EED_0005);
    let mut code = Vec::new();
    for _ in 0..instructions {
        match rng.next() % 4 {
            0 => {
                code.push(0x29);
                code.u32le(rng.next());
            }
            1 => {
                code.push(0x2D);
                code.push(2);
                code.push(1);
                code.u32le(rng.next());
            }
            2 => code.push(0x01),
            _ => code.push(0x60),
        }
    }
    let mut file = Vec::new();
    file.u32le(0x0E52_4353);
    file.u32le(to_u32(code.len()));
    file.u32le(16);
    file.u32le(16);
    file.u32le(0);
    file.u32le(0);
    file.extend_from_slice(&code);
    file.resize(file.len() + 32 * 4, 0);
    file
}

/// A `vehicles.ide` text with `rows` `cars` rows.
#[must_use]
pub fn ide_text(rows: usize) -> Vec<u8> {
    use std::fmt::Write as _;
    let mut text = String::from("# synthetic\ncars\n");
    for i in 0..rows {
        writeln!(
            text,
            "car{i}, car{i}, car, CAR{i}, CAR{i}, VEH@STD, NULL, 100, 999, 0.2, 0.2, 0, 2, 1.0, 0, -"
        )
        .expect("write to String cannot fail");
    }
    text.push_str("end\n");
    text.into_bytes()
}

/// A save file: header plus `blocks` blocks of `payload` bytes.
#[must_use]
pub fn save(blocks: usize, payload: usize) -> Vec<u8> {
    let mut file = Vec::new();
    file.u32le(57);
    file.u32le(0);
    file.u32le(0);
    file.extend_from_slice(b"SAVE");
    file.resize(0x110, 0);
    for _ in 0..blocks {
        file.extend_from_slice(b"BLOCK");
        file.u32le(to_u32(payload + 9));
        file.resize(file.len() + payload, 0x11);
    }
    let sum = file.iter().fold(0u32, |s, &b| s.wrapping_add(u32::from(b)));
    file.u32le(sum);
    file
}
