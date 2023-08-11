use mlua::prelude::*;

use crate::lune::lua::{
    luau::{LuauCompileOptions, LuauLoadOptions},
    table::TableBuilder,
};

const BYTECODE_ERROR_BYTE: u8 = 0;

pub fn create(lua: &'static Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("compile", compile_source)?
        .with_function("load", load_source)?
        .build_readonly()
}

fn compile_source<'lua>(
    lua: &'lua Lua,
    (source, options): (LuaString<'lua>, LuauCompileOptions),
) -> LuaResult<LuaString<'lua>> {
    let bytecode = options.into_compiler().compile(source);

    match bytecode.first() {
        Some(&BYTECODE_ERROR_BYTE) => Err(LuaError::RuntimeError(
            String::from_utf8_lossy(&bytecode).into_owned(),
        )),
        Some(_) => lua.create_string(bytecode),
        None => panic!("Compiling resulted in empty bytecode"),
    }
}

fn load_source<'a>(
    lua: &'static Lua,
    (source, options): (LuaString<'a>, LuauLoadOptions),
) -> LuaResult<LuaFunction<'a>> {
    lua.load(source.as_bytes())
        .set_name(options.debug_name)
        .into_function()
}
