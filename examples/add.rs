extern crate llvm;
use llvm::*;

fn main() {
    let ctx = Context::new();
    let module = Module::new("add", &ctx);
    let func = module.add_function("add", Type::get::<fn(f64, f64) -> f64>(&ctx));
    let entry = func.append("entry");
    let builder = Builder::new(&ctx);
    builder.position_at_end(entry);
    let a = &func[0];
    let b = &func[1];
    let value = builder.build_add(a, b);
    builder.build_ret(value);
    module.verify().unwrap();
    let ee = JitEngine::new(&module, JitOptions { opt_level: 3 }).unwrap();
    ee.with_function(func, |add: extern "C" fn((f64, f64)) -> f64| {
        println!("{} + {} = {}", 1., 2., add((1., 2.)));
    });
}
