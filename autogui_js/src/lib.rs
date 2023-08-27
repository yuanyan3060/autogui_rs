use boa_engine::builtins::promise::PromiseState;
use boa_engine::module::{ModuleLoader, SimpleModuleLoader};
use boa_engine::object::FunctionObjectBuilder;
use boa_engine::property::Attribute;
use boa_engine::{Context, JsError, JsValue, Module, NativeFunction, Source};
use boa_runtime::Console;
use builtin::sleep;
use js_adb::JsAdb;
use js_image::JsImage;
use js_math::JsPoint;
use std::path::Path;
mod builtin;
mod js_adb;
mod js_image;
mod js_math;
pub fn add_runtime(context: &mut Context<'_>) {
    let console = Console::init(context);
    context
        .register_global_property(Console::NAME, console, Attribute::all())
        .expect("the console object shouldn't exist");
    context.register_global_class::<JsAdb>().expect("the Adb builtin shouldn't exist");
    context.register_global_class::<JsImage>().expect("the Image builtin shouldn't exist");
    context.register_global_class::<JsPoint>().expect("the Point builtin shouldn't exist");
    context
        .register_global_callable("sleep", 1, NativeFunction::from_fn_ptr(sleep))
        .expect("the sleep builtin shouldn't exist");
}

pub fn run_js_code(js_file_path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let src = Source::from_filepath(Path::new(js_file_path.as_ref()))?;
    let loader = &SimpleModuleLoader::new(js_file_path.as_ref().parent().unwrap()).unwrap();
    let dyn_loader: &dyn ModuleLoader = loader;
    let mut context = &mut Context::builder().module_loader(dyn_loader).build()?;
    add_runtime(&mut context);
    let module = Module::parse(src, None, context)?;
    loader.insert(js_file_path.as_ref().into(), module.clone());
    let promise_result = module
        .load(context)
        .then(
            Some(
                FunctionObjectBuilder::new(
                    context,
                    NativeFunction::from_copy_closure_with_captures(
                        |_, _, module, context| {
                            module.link(context)?;
                            Ok(JsValue::undefined())
                        },
                        module.clone(),
                    ),
                )
                .build(),
            ),
            None,
            context,
        )?
        .then(
            Some(
                FunctionObjectBuilder::new(
                    context,
                    NativeFunction::from_copy_closure_with_captures(|_, _, module, context| Ok(module.evaluate(context).into()), module.clone()),
                )
                .build(),
            ),
            None,
            context,
        )?;
    context.run_jobs();
    match promise_result.state()? {
        PromiseState::Pending => return Err("module didn't execute!".into()),
        PromiseState::Fulfilled(v) => {
            assert_eq!(v, JsValue::undefined())
        }
        PromiseState::Rejected(err) => return Err(JsError::from_opaque(err).try_native(context)?.into()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let js_file_path = "./scripts/index.js";
        run_js_code(js_file_path)
    }
}
