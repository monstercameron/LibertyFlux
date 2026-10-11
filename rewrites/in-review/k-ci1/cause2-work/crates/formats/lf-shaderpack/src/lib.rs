//! Read-only parser for the game's compiled shader container (`.fxc`).
//!
//! # Format overview
//!
//! Each `.fxc` file is a small container that bundles precompiled Direct3D 9
//! shader programs with the metadata the engine needs to bind them: a
//! parameter table, a technique/pass table, and per-pass render states.
//! All integers are little-endian. There is no compression or encryption.
//!
//! Layout, in order:
//!
//! ```text
//! offset  field
//! ------  ---------------------------------------------------------
//! 0       magic `rgxa` (u32, 0x61786772)
//! 4       vertex program count (u8)
//! 5       vertex programs (fragments), see below
//! ...     pixel program count raw (u8); real count is this minus one
//! ...     five reserved bytes (observed all zero)
//! ...     pixel programs (fragments)
//! ...     shared parameter count (u8), then that many parameters
//! ...     material parameter count (u8), then that many parameters
//! ...     technique count (u8), then that many techniques
//! ...     end of file (no footer; nothing follows the techniques)
//! ```
//!
//! A program fragment holds the names the program reads, then the bytecode:
//!
//! ```text
//! u8     bound-name count
//! repeat bound-name count times:
//!   u8   parameter type code (same codes as the parameter table)
//!   u8   index (observed always 0; meaning unknown)
//!   u16  constant register number (matches the compiler listing: the
//!        matrix named `gWorld` sits at register 0, for example)
//!   u8 + bytes  name as a length-prefixed string (length counts the
//!        trailing NUL)
//! u16    bytecode size in bytes
//! u16    bytecode size again (the two always agree in shipped files)
//! bytes  Direct3D 9 shader bytecode (starts with a version token such
//!        as `vs_3_0`, ends with the end token 0x0000FFFF)
//! ```
//!
//! A parameter entry describes one named constant, texture, or switch:
//!
//! ```text
//! u8     type code (see [`ParamKind`])
//! u8     array length: 0 means a single value, otherwise that many
//!        elements (a 48-entry bone matrix array and a 64-entry global
//!        block both appear in shipped files)
//! string name, string semantic (both length-prefixed, see above)
//! u8     annotation count, then that many annotations:
//!   string annotation name
//!   u8     value kind: 0 = u32, 1 = f32, 2 = string
//!   value  one dword, or a length-prefixed string for kind 2
//! u8     default-value length in dwords, then that many dwords
//!        (float defaults for vectors, (id, value) pairs for samplers)
//! ```
//!
//! A technique is one named rendering setup with one or more passes:
//!
//! ```text
//! string technique name
//! u8     pass count (almost always 1)
//! repeat pass count times:
//!   u8   vertex program index (0-based into the vertex list)
//!   u8   pixel program index (1-based into the pixel list; 0 would mean
//!        no pixel program, which no shipped file uses)
//!   u8   render-state count, then that many pairs of:
//!     u32  state id (engine-specific numbering, not raw Direct3D ids)
//!     u32  state value
//! ```
//!
//! Strings are length-prefixed: one length byte that counts the bytes
//! that follow including the trailing NUL, then the bytes. Names are
//! plain ASCII in every shipped file.
//!
//! # What is still unknown
//!
//! * The pixel-count-minus-one rule and the five reserved bytes after it
//!   have no confirmed explanation; every shipped file parses this way.
//! * The second byte of a bound name (always 0) is unexplained.
//! * Render-state ids and sampler pair ids are engine enumerations whose
//!   exact meanings need a running game to confirm; they are exposed as
//!   raw `(id, value)` pairs.
//! * The semantic string of a parameter is descriptive text (often the
//!   name without its prefix); the engine matches programs to parameters
//!   by name.
//!
//! # Feeding a translation step
//!
//! A shader translator (for example the MojoShader-based pipeline built
//! by lane `t-shaders`) takes each [`Program::bytecode`] slice together
//! with its [`Stage`]. [`Program::validate`] checks the bytecode before
//! handing it over: version token, token walk to the end marker, and a
//! constant-table probe. Techniques say which vertex/pixel pairs are used
//! together, and the parameter tables give every name, register, default,
//! and annotation the translated shader must expose to the engine.
//!
//! # Sidecar formats
//!
//! Next to the containers sit two plain-text sidecars, parsed by the
//! [`dcl`] and [`sps`] modules: `.dcl` vertex declaration bitmasks (one
//! per shader, stems match the container names exactly) and `.sps`
//! material preset overrides.

#![forbid(unsafe_code)]

pub mod bytecode;
pub mod dcl;
pub mod sps;

use bytecode::{BytecodeError, BytecodeInfo};

/// Container magic: the bytes `rgxa` read as little-endian u32.
pub const MAGIC: u32 = 0x6178_6772;

/// Number of reserved bytes between the pixel count and the pixel programs.
pub const PS_PADDING_LEN: usize = 5;

/// What went wrong while parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    /// Byte offset where parsing failed (or end of input for truncation).
    pub offset: usize,
    /// The failure reason.
    pub kind: ErrorKind,
}

/// Reasons a parse can fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// Input ended in the middle of a field.
    UnexpectedEof {
        /// How many bytes the field needed.
        need: usize,
    },
    /// First four bytes were not the container magic.
    BadMagic {
        /// The value actually found.
        found: u32,
    },
    /// An annotation value kind other than 0 (u32), 1 (f32), 2 (string).
    UnknownAnnotationType {
        /// The value kind byte found.
        code: u8,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::UnexpectedEof { need } => {
                write!(
                    f,
                    "unexpected end of input at offset {} (need {} bytes)",
                    self.offset, need
                )
            }
            ErrorKind::BadMagic { found } => {
                write!(f, "bad magic 0x{found:08x} at offset {}", self.offset)
            }
            ErrorKind::UnknownAnnotationType { code } => {
                write!(
                    f,
                    "unknown annotation value kind {code} at offset {}",
                    self.offset
                )
            }
        }
    }
}

impl std::error::Error for Error {}

/// Byte cursor over a borrowed slice. All reads are explicit little-endian.
struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Cursor { bytes, pos: 0 }
    }

    fn left(&self) -> usize {
        self.bytes.len().saturating_sub(self.pos)
    }

    fn u8(&mut self) -> Result<u8, Error> {
        if self.left() < 1 {
            return Err(Error {
                offset: self.pos,
                kind: ErrorKind::UnexpectedEof { need: 1 },
            });
        }
        let v = self.bytes[self.pos];
        self.pos += 1;
        Ok(v)
    }

    fn u16(&mut self) -> Result<u16, Error> {
        if self.left() < 2 {
            return Err(Error {
                offset: self.pos,
                kind: ErrorKind::UnexpectedEof { need: 2 },
            });
        }
        let v = u16::from_le_bytes([self.bytes[self.pos], self.bytes[self.pos + 1]]);
        self.pos += 2;
        Ok(v)
    }

    fn u32(&mut self) -> Result<u32, Error> {
        if self.left() < 4 {
            return Err(Error {
                offset: self.pos,
                kind: ErrorKind::UnexpectedEof { need: 4 },
            });
        }
        let v = u32::from_le_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(v)
    }

    fn f32(&mut self) -> Result<f32, Error> {
        let raw = self.bytes(4)?;
        Ok(f32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]))
    }

    fn bytes(&mut self, n: usize) -> Result<&'a [u8], Error> {
        if self.left() < n {
            return Err(Error {
                offset: self.pos,
                kind: ErrorKind::UnexpectedEof { need: n },
            });
        }
        let v = &self.bytes[self.pos..self.pos + n];
        self.pos += n;
        Ok(v)
    }

    /// Length-prefixed string: one length byte counting the trailing NUL,
    /// then that many bytes. Stops at the first NUL; undecodable bytes
    /// become the replacement character rather than failing the parse.
    fn string(&mut self) -> Result<String, Error> {
        let at = self.pos;
        let len = self.u8()? as usize;
        if self.left() < len {
            return Err(Error {
                offset: at,
                kind: ErrorKind::UnexpectedEof { need: len },
            });
        }
        let raw = &self.bytes[self.pos..self.pos + len];
        self.pos += len;
        let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
        Ok(String::from_utf8_lossy(&raw[..end]).into_owned())
    }
}

/// Which pipeline stage a program runs on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// Vertex program (`vs_*` bytecode).
    Vertex,
    /// Pixel program (`ps_*` bytecode).
    Pixel,
}

/// One name a program reads: which parameter, at which constant register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundVar {
    /// Parameter type code; same numbering as [`Param::type_code`].
    pub type_code: u8,
    /// Second byte of the entry. Always 0 in shipped files; unknown meaning.
    pub index: u8,
    /// Constant register number (the `cN` of the compiler listing).
    pub register: u16,
    /// Parameter name, matched against the parameter tables by name.
    pub name: String,
}

/// One compiled Direct3D 9 program and the names it binds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program<'a> {
    /// Vertex or pixel stage.
    pub stage: Stage,
    /// Names this program reads, with registers.
    pub vars: Vec<BoundVar>,
    /// Bytecode size as stored in the first size field.
    pub declared_size: u16,
    /// Bytecode size as stored in the second size field (always equal).
    pub declared_size_copy: u16,
    /// File offset where the bytecode starts.
    pub bytecode_offset: usize,
    /// The raw Direct3D 9 bytecode, ready for a translator.
    pub bytecode: &'a [u8],
}

impl Program<'_> {
    /// First dword of the bytecode (the version token), if present.
    #[must_use]
    pub fn version_token(&self) -> Option<u32> {
        self.bytecode
            .get(..4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// Walk the token stream to the end marker and classify the program.
    /// This is the check to run before handing [`Program::bytecode`] to a
    /// shader translator.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn validate(&self) -> Result<BytecodeInfo, BytecodeError> {
        bytecode::validate(self.bytecode)
    }
}

/// A parameter annotation: tool-facing metadata such as UI ranges.
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    /// Annotation name (`UIName`, `UIMin`, `Space`, ...).
    pub name: String,
    /// Annotation value.
    pub value: AnnotationValue,
}

/// An annotation value. Kind 0 is an integer, kind 1 a float, kind 2 text.
#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationValue {
    /// Kind 0: raw dword (bucket ids, integer bounds).
    Int(u32),
    /// Kind 1: float (UI ranges and steps; verified sensible, e.g. -10..10).
    Float(f32),
    /// Kind 2: text (widget names, help strings).
    Text(String),
}

/// One entry of a parameter table.
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    /// Type code; see [`Param::kind`] for the inferred meaning.
    pub type_code: u8,
    /// Array length: 0 means a single value, otherwise that many elements.
    /// The default length always equals length times element dwords.
    pub slot: u8,
    /// Parameter name; programs bind to this.
    pub name: String,
    /// Descriptive second name, often the name without its prefix.
    pub semantic: String,
    /// Tool-facing metadata (UI ranges, widget hints).
    pub annotations: Vec<Annotation>,
    /// Default value as raw dwords: floats for vectors/scalars, (id, value)
    /// pairs for samplers (see [`Param::sampler_states`]).
    pub defaults: Vec<u32>,
}

/// Inferred meaning of a parameter type code, from names, default lengths,
/// and the compiler's own type spellings in the reference listing.
/// Anything outside 1..=9 has never been observed and maps to `Other`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamKind {
    /// Code 1: one dword integer (`drawBucket`, `orderNumber`).
    Int,
    /// Code 2: one dword scalar (floats such as resolutions and biases).
    Scalar,
    /// Code 3: two dwords.
    Vec2,
    /// Code 4: three dwords.
    Vec3,
    /// Code 5: four dwords (float4 vectors; the 64-entry global block).
    Vec4,
    /// Code 6: texture plus sampler state as (id, value) dword pairs.
    Texture,
    /// Code 7: boolean switch (`switchOn`, `gUseDirectional`).
    Bool,
    /// Code 8: matrix array (the 48-entry bone matrix array).
    MatrixArray,
    /// Code 9: single matrix (world, view, projection); no default stored.
    Matrix,
    /// Any code outside 1..=9. Never observed in shipped files.
    Other(u8),
}

impl Param {
    /// Interpret [`Param::type_code`] (see [`ParamKind`]).
    #[must_use]
    pub fn kind(&self) -> ParamKind {
        match self.type_code {
            1 => ParamKind::Int,
            2 => ParamKind::Scalar,
            3 => ParamKind::Vec2,
            4 => ParamKind::Vec3,
            5 => ParamKind::Vec4,
            6 => ParamKind::Texture,
            7 => ParamKind::Bool,
            8 => ParamKind::MatrixArray,
            9 => ParamKind::Matrix,
            other => ParamKind::Other(other),
        }
    }

    /// True when [`Param::slot`] says this is an array rather than single.
    #[must_use]
    pub fn is_array(&self) -> bool {
        self.slot != 0
    }

    /// Defaults reinterpreted as floats (for scalar/vector parameters).
    #[must_use]
    pub fn defaults_f32(&self) -> Vec<f32> {
        self.defaults
            .iter()
            .map(|&w| f32::from_le_bytes(w.to_le_bytes()))
            .collect()
    }

    /// Defaults grouped as (id, value) pairs (for texture parameters).
    /// A trailing lone dword, if any, is dropped.
    #[must_use]
    pub fn sampler_states(&self) -> Vec<(u32, u32)> {
        self.defaults
            .chunks_exact(2)
            .map(|pair| (pair[0], pair[1]))
            .collect()
    }
}

/// One render-state assignment inside a pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateAssign {
    /// Engine state id (engine-specific numbering).
    pub id: u32,
    /// State value.
    pub value: u32,
}

/// One pass: a vertex/pixel program pair plus render states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pass {
    /// 0-based index into [`ShaderPack::vs_programs`].
    pub vs_index: usize,
    /// 1-based pixel index minus one, i.e. an index into
    /// [`ShaderPack::ps_programs` — `None` when the file stores 0, which no
    /// shipped file does.
    pub ps_index: Option<usize>,
    /// Render states applied for this pass.
    pub states: Vec<StateAssign>,
}

/// One named technique: a rendering setup with one or more passes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Technique {
    /// Technique name (`draw`, `drawskinned`, `deferred_draw`, ...).
    pub name: String,
    /// Passes, almost always exactly one.
    pub passes: Vec<Pass>,
}

/// A parsed shader container, borrowing the file bytes.
#[derive(Debug, Clone, PartialEq)]
pub struct ShaderPack<'a> {
    /// Vertex programs in file order; techniques index here 0-based.
    pub vs_programs: Vec<Program<'a>>,
    /// Pixel programs in file order; techniques index here 1-based.
    pub ps_programs: Vec<Program<'a>>,
    /// The five reserved bytes after the pixel count (all zero observed).
    pub ps_padding: [u8; PS_PADDING_LEN],
    /// First parameter group: engine-wide shared values (matrices, lights,
    /// fog, shadow setup). The shared/material split is inferred from the
    /// names each group holds.
    pub shared_params: Vec<Param>,
    /// Second parameter group: per-material values (diffuse sampler,
    /// material constants, switches).
    pub material_params: Vec<Param>,
    /// Techniques in file order.
    pub techniques: Vec<Technique>,
    /// Bytes after the last technique. Empty in every shipped file; kept
    /// so a future revision with a footer still parses.
    pub trailing_bytes: &'a [u8],
}

impl<'a> ShaderPack<'a> {
    /// Parse a whole container from borrowed bytes. Never panics on
    /// malformed input: every truncation or bad tag becomes an [`Error`].
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(bytes: &'a [u8]) -> Result<Self, Error> {
        let mut c = Cursor::new(bytes);
        let magic = c.u32()?;
        if magic != MAGIC {
            return Err(Error {
                offset: 0,
                kind: ErrorKind::BadMagic { found: magic },
            });
        }
        let n_vs = c.u8()? as usize;
        let mut vert_programs = Vec::with_capacity(n_vs);
        for _ in 0..n_vs {
            vert_programs.push(read_program(&mut c, Stage::Vertex)?);
        }
        let n_pix_raw = c.u8()?;
        let pad = c.bytes(PS_PADDING_LEN)?;
        let mut ps_padding = [0u8; PS_PADDING_LEN];
        ps_padding.copy_from_slice(pad);
        // Every shipped file stores one more than the pixel programs that
        // follow; files with a zero count would hold none.
        let n_pix = n_pix_raw.saturating_sub(1) as usize;
        let mut pix_programs = Vec::with_capacity(n_pix);
        for _ in 0..n_pix {
            pix_programs.push(read_program(&mut c, Stage::Pixel)?);
        }
        let mut shared_params = Vec::new();
        let n_shared = c.u8()? as usize;
        for _ in 0..n_shared {
            shared_params.push(read_param(&mut c)?);
        }
        let mut material_params = Vec::new();
        let n_material = c.u8()? as usize;
        for _ in 0..n_material {
            material_params.push(read_param(&mut c)?);
        }
        let mut techniques = Vec::new();
        let n_tech = c.u8()? as usize;
        for _ in 0..n_tech {
            techniques.push(read_technique(&mut c)?);
        }
        let trailing_bytes = &bytes[c.pos..];
        Ok(ShaderPack {
            vs_programs: vert_programs,
            ps_programs: pix_programs,
            ps_padding,
            shared_params,
            material_params,
            techniques,
            trailing_bytes,
        })
    }

    /// Iterate over all programs with their stage, vertex first.
    pub fn programs(&self) -> impl Iterator<Item = (Stage, &Program<'a>)> {
        self.vs_programs
            .iter()
            .map(|p| (Stage::Vertex, p))
            .chain(self.ps_programs.iter().map(|p| (Stage::Pixel, p)))
    }

    /// Iterate over all parameters, shared first.
    pub fn all_params(&self) -> impl Iterator<Item = &Param> {
        self.shared_params.iter().chain(self.material_params.iter())
    }

    /// Look up a parameter by name (shared group first).
    #[must_use]
    pub fn param(&self, name: &str) -> Option<&Param> {
        self.all_params().find(|p| p.name == name)
    }
}

fn read_program<'a>(c: &mut Cursor<'a>, stage: Stage) -> Result<Program<'a>, Error> {
    let n_vars = c.u8()? as usize;
    let mut vars = Vec::with_capacity(n_vars);
    for _ in 0..n_vars {
        let type_code = c.u8()?;
        let index = c.u8()?;
        let register = c.u16()?;
        let name = c.string()?;
        vars.push(BoundVar {
            type_code,
            index,
            register,
            name,
        });
    }
    let declared_size = c.u16()?;
    let declared_size_copy = c.u16()?;
    let bytecode_offset = c.pos;
    let bytecode = c.bytes(usize::from(declared_size))?;
    Ok(Program {
        stage,
        vars,
        declared_size,
        declared_size_copy,
        bytecode_offset,
        bytecode,
    })
}

fn read_param(c: &mut Cursor<'_>) -> Result<Param, Error> {
    let type_code = c.u8()?;
    let slot = c.u8()?;
    let name = c.string()?;
    let semantic = c.string()?;
    let n_ann = c.u8()? as usize;
    let mut annotations = Vec::with_capacity(n_ann);
    for _ in 0..n_ann {
        let at = c.pos;
        let name = c.string()?;
        let code = c.u8()?;
        let value = match code {
            0 => AnnotationValue::Int(c.u32()?),
            1 => AnnotationValue::Float(c.f32()?),
            2 => AnnotationValue::Text(c.string()?),
            other => {
                return Err(Error {
                    offset: at,
                    kind: ErrorKind::UnknownAnnotationType { code: other },
                });
            }
        };
        annotations.push(Annotation { name, value });
    }
    let n_defaults = c.u8()? as usize;
    let mut defaults = Vec::with_capacity(n_defaults);
    for _ in 0..n_defaults {
        defaults.push(c.u32()?);
    }
    Ok(Param {
        type_code,
        slot,
        name,
        semantic,
        annotations,
        defaults,
    })
}

fn read_technique(c: &mut Cursor<'_>) -> Result<Technique, Error> {
    let name = c.string()?;
    let n_pass = c.u8()? as usize;
    let mut passes = Vec::with_capacity(n_pass);
    for _ in 0..n_pass {
        let vs_index = usize::from(c.u8()?);
        let ps_raw = c.u8()?;
        let ps_index = if ps_raw == 0 {
            None
        } else {
            Some(usize::from(ps_raw) - 1)
        };
        let n_states = c.u8()? as usize;
        let mut states = Vec::with_capacity(n_states);
        for _ in 0..n_states {
            let id = c.u32()?;
            let value = c.u32()?;
            states.push(StateAssign { id, value });
        }
        passes.push(Pass {
            vs_index,
            ps_index,
            states,
        });
    }
    Ok(Technique { name, passes })
}
