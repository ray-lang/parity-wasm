use elements::{MemoryType, TableType, GlobalType, Type};
use elements::{Opcode, BlockType, ValueType, TableElementType};
use validation::Error;

pub struct ModuleContext {
	pub memories: Vec<MemoryType>,
	pub tables: Vec<TableType>,
	pub globals: Vec<GlobalType>,
	pub types: Vec<Type>,
	pub func_type_indexes: Vec<u32>,
}

impl ModuleContext {
	pub fn memories(&self) -> &[MemoryType] {
		&self.memories
	}

	pub fn tables(&self) -> &[TableType] {
		&self.tables
	}

	pub fn globals(&self) -> &[GlobalType] {
		&self.globals
	}

	pub fn types(&self) -> &[Type] {
		&self.types
	}

	pub fn func_type_indexes(&self) -> &[u32] {
		&self.func_type_indexes
	}

	pub fn require_memory(&self, idx: u32) -> Result<(), Error> {
		if self.memories().get(idx as usize).is_none() {
			return Err(Error(format!("Memory at index {} doesn't exists", idx)));
		}
		Ok(())
	}

	pub fn require_table(&self, idx: u32, expected_type: TableElementType) -> Result<(), Error> {
		let table = match self.tables().get(idx as usize) {
			Some(table) => table,
			None => {
				return Err(Error(format!("Table at index {} doesn't exists", idx)));
			}
		};

		if table.elem_type() != expected_type {
			return Err(Error(format!(
				"Table {} has element type {:?} while {:?} expected",
				idx,
				table.elem_type(),
				expected_type
			)));
		}

		Ok(())
	}

	pub fn require_function(&self, idx: u32) -> Result<(Vec<ValueType>, BlockType), Error> {
		let ty_idx = match self.func_type_indexes().get(idx as usize) {
			Some(ty_idx) => *ty_idx,
			None => {
				return Err(Error(
					format!("Function at index {} doesn't exists", idx),
				));
			}
		};
		self.require_function_type(ty_idx)
	}

	pub fn require_function_type(&self, idx: u32) -> Result<(Vec<ValueType>, BlockType), Error> {
		let ty = match self.types().get(idx as usize) {
			Some(&Type::Function(ref func_ty)) => func_ty,
			None => {
				return Err(Error(
					format!("Type at index {} doesn't exists", idx),
				));
			}
		};

		let params = ty.params().to_vec();
		let return_ty = ty.return_type()
			.map(BlockType::Value)
			.unwrap_or(BlockType::NoResult);
		Ok((params, return_ty))
	}
}
