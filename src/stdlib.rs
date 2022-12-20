//! Load the Nickel standard library in strings at compile-time.

use crate::identifier::Ident;
use crate::term::make as mk_term;
use crate::term::RichTerm;

<<<<<<< HEAD
/// This is an array containing all the Nickel standard library modules.
pub fn modules() -> [StdlibModule; 9] {
    [
        StdlibModule::Builtin,
        StdlibModule::Contract,
        StdlibModule::Array,
        StdlibModule::Record,
        StdlibModule::String,
        StdlibModule::Num,
        StdlibModule::Function,
        StdlibModule::Internals,
        StdlibModule::Compat,
    ]
}


/// Represents a particular Nickel standard library module.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum StdlibModule {
    Builtin,
    Contract,
    Array,
    Record,
    String,
    Num,
    Function,
    Internals,
    Compat,
}

impl StdlibModule {
    pub fn file_name(&self) -> &'static str {
        match self {
            StdlibModule::Builtin => "<stdlib/builtin.ncl>",
            StdlibModule::Contract => "<stdlib/contract.ncl>",
            StdlibModule::Array => "<stdlib/array.ncl>",
            StdlibModule::Record => "<stdlib/record.ncl>",
            StdlibModule::String => "<stdlib/string.ncl>",
            StdlibModule::Num => "<stdlib/num.ncl>",
            StdlibModule::Function => "<stdlib/function.ncl>",
            StdlibModule::Internals => "<stdlib/internals.ncl>",
            StdlibModule::Compat => "<stdlib/compat.ncl>",
        }
    }

    pub const fn content(&self) -> &'static str {
        match self {
            StdlibModule::Builtin => include_str!("../stdlib/builtin.ncl"),
            StdlibModule::Contract => include_str!("../stdlib/contract.ncl"),
            StdlibModule::Array => include_str!("../stdlib/array.ncl"),
            StdlibModule::Record => include_str!("../stdlib/record.ncl"),
            StdlibModule::String => include_str!("../stdlib/string.ncl"),
            StdlibModule::Num => include_str!("../stdlib/num.ncl"),
            StdlibModule::Function => include_str!("../stdlib/function.ncl"),
            StdlibModule::Internals => include_str!("../stdlib/internals.ncl"),
            StdlibModule::Compat => include_str!("../stdlib/compat.ncl"),
        }
    }
}

pub struct UnknownStdlibModule;

impl TryFrom<Ident> for StdlibModule {
    type Error = UnknownStdlibModule;

    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        let module = match value.label() {
            "builtin" => StdlibModule::Builtin,
            "contract" => StdlibModule::Contract,
            "array" => StdlibModule::Array,
            "record" => StdlibModule::Record,
            "string" => StdlibModule::String,
            "num" => StdlibModule::Num,
            "function" => StdlibModule::Function,
            "internals" => StdlibModule::Internals,
            "compat" => StdlibModule::Compat,
            _ => return Err(UnknownStdlibModule),
        };
        Ok(module)
    }
}

impl From<StdlibModule> for Ident {
    fn from(module: StdlibModule) -> Self {
        let name = match module {
            StdlibModule::Builtin => "builtin",
            StdlibModule::Contract => "contract",
            StdlibModule::Array => "array",
            StdlibModule::Record => "record",
            StdlibModule::String => "string",
            StdlibModule::Num => "num",
            StdlibModule::Function => "function",
            StdlibModule::Internals => "internals",
            StdlibModule::Compat=> "Compat",
        };
        Ident::from(name)
    }
}

macro_rules! generate_accessor {
    ($value:ident) => {
        pub fn $value() -> RichTerm {
            mk_term::var(format!("${}", stringify!($value)))
        }
    };
}

/// Accessors to the builtin contracts.
pub mod contract {
    use super::*;

    // `dyn` is a reserved keyword in rust
    pub fn dynamic() -> RichTerm {
        mk_term::var("$dyn")
    }

    generate_accessor!(num);
    generate_accessor!(bool);
    generate_accessor!(string);
    generate_accessor!(array);
    generate_accessor!(func);
    generate_accessor!(forall_var);
    generate_accessor!(fail);
    generate_accessor!(enums);
    generate_accessor!(enum_fail);
    generate_accessor!(record);
    generate_accessor!(dyn_record);
    generate_accessor!(record_extend);
    generate_accessor!(forall_tail);
    generate_accessor!(dyn_tail);
    generate_accessor!(empty_tail);
}

pub mod internals {
    use super::*;

    generate_accessor!(rec_default);
    generate_accessor!(rec_force);
}
