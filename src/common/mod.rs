//! Common data structures shared by the Compiler and VM.

pub mod bytecode;
pub mod disassembler;
pub mod file;
pub mod opcode;
pub mod source;
pub mod span;
pub mod value;

pub use bytecode::{ByteCode, DebugInfo};
pub use disassembler::Disassembler;
pub use opcode::Opcode;
pub use source::Source;
pub use span::{Span, Spanned};
pub use value::{
    Captured, Class, Closure, Constructor, External, ExternalData, Function, Instance,
    InstanceMethod, MetaMap, NativeFun, Upvalue, Value, ValueMap,
};

pub use file::{KaonFile, KaonRead, KaonWrite};
