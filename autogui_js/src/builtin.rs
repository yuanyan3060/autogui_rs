use boa_engine::{Context, JsArgs, JsResult, JsValue};

pub fn sleep(_this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
    let ms = args.get_or_undefined(0).to_u32(context)?;
    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    return Ok(JsValue::undefined());
}
