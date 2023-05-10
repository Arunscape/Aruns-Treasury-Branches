#wasmBytes = open('../../atb_backend/target/wasm32-unknown-unknown/debug/atb_api.wasm ', 'rb').read()

from wasmer_compiler_cranelift import Compiler
from wasmer import engine, Store, Module, Instance

wasm_bytes = open('atb_api.wasm ', 'rb').read()

engine = engine.JIT(Compiler)
store = Store(engine)
module = Module(store, wasm_bytes)

instance = Instance(module)
