use buffer::MemoryBuffer;
use cbox::{CBox, DisposeRef};
use context::Context;
use ffi::object::{self, LLVMBinaryRef};
use std::marker::PhantomData;
use std::mem::MaybeUninit;

/// An external object file that has been parsed by LLVM.
pub struct Binary(PhantomData<[u8]>);
native_ref!(&Binary = LLVMBinaryRef);

#[derive(Debug, Copy, Clone)]
pub enum BinaryType {
    /// Archive file
    Archive,
    /// Mach-O Universal Binary file
    MachOUniversalBinary,
    /// COFF Import file
    COFFImportFile,
    /// LLVM IR
    IR,
    /// Windows resource (.res) file
    WinRes,
    /// COFF Object file
    COFF,
    /// ELF 32-bit, little endian
    ELF32L,
    /// ELF 32-bit, big endian
    ELF32B,
    /// ELF 64-bit, little endian
    ELF64L,
    /// ELF 64-bit, big endian
    ELF64B,
    /// MachO 32-bit, little endian
    MachO32L,
    /// MachO 32-bit, big endian
    MachO32B,
    /// MachO 64-bit, little endian
    MachO64L,

    /// MachO 64-bit, big endian
    MachO64B,
    /// Web assembly
    Wasm,
}
impl DisposeRef for Binary {
    type RefTo = object::LLVMOpaqueBinary;
    unsafe fn dispose(ptr: LLVMBinaryRef) {
        object::LLVMDisposeBinary(ptr);
    }
}

impl From<object::LLVMBinaryType> for BinaryType {
    fn from(ty: object::LLVMBinaryType) -> Self {
        match ty {
            object::LLVMBinaryType::LLVMBinaryTypeArchive => BinaryType::Archive,
            object::LLVMBinaryType::LLVMBinaryTypeMachOUniversalBinary => {
                BinaryType::MachOUniversalBinary
            }
            object::LLVMBinaryType::LLVMBinaryTypeCOFFImportFile => BinaryType::COFFImportFile,
            object::LLVMBinaryType::LLVMBinaryTypeIR => BinaryType::IR,
            object::LLVMBinaryType::LLVMBinaryTypeWinRes => BinaryType::WinRes,
            object::LLVMBinaryType::LLVMBinaryTypeCOFF => BinaryType::COFF,
            object::LLVMBinaryType::LLVMBinaryTypeELF32L => BinaryType::ELF32L,
            object::LLVMBinaryType::LLVMBinaryTypeELF32B => BinaryType::ELF32B,
            object::LLVMBinaryType::LLVMBinaryTypeELF64L => BinaryType::ELF64L,
            object::LLVMBinaryType::LLVMBinaryTypeELF64B => BinaryType::ELF64B,
            object::LLVMBinaryType::LLVMBinaryTypeMachO32L => BinaryType::MachO32L,
            object::LLVMBinaryType::LLVMBinaryTypeMachO32B => BinaryType::MachO32B,
            object::LLVMBinaryType::LLVMBinaryTypeMachO64L => BinaryType::MachO64L,
            object::LLVMBinaryType::LLVMBinaryTypeMachO64B => BinaryType::MachO64B,
            object::LLVMBinaryType::LLVMBinaryTypeWasm => BinaryType::Wasm,
        }
    }
}

impl Binary {
    /// Parse the object file at the path given, or return an error string if an error occurs.
    pub fn read(path: &str, ctx: &mut Context) -> Result<CBox<Binary>, CBox<str>> {
        let buf = MemoryBuffer::new_from_file(path)?;
        unsafe {
            let mut err = MaybeUninit::zeroed().assume_init();
            let ptr = object::LLVMCreateBinary(buf.as_ptr(), ctx.into(), &mut err);
            if ptr.is_null() {
                Err(CBox::new(err))
            } else {
                Ok(CBox::new(ptr))
            }
        }
    }
    pub fn binary_type(&self) -> BinaryType {
        unsafe { object::LLVMBinaryGetType(self.into()) }.into()
    }
}
