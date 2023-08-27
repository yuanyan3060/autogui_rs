use autogui_core::Controller;
use boa_engine::{
    class::{Class, ClassBuilder},
    object::{ObjectData, PROTOTYPE},
    Context, JsArgs, JsNativeError, JsObject, JsResult, JsValue, NativeFunction,
};
use boa_gc::Finalize;

use crate::js_image::JsImage;

#[derive(Debug, Finalize)]
pub struct JsAdb(pub autogui_core::ADB);
unsafe impl boa_gc::Trace for JsAdb {
    boa_gc::empty_trace!();
}

impl JsAdb {
    pub fn click(this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
        if let Some(object) = this.as_object() {
            if let Some(mut adb) = object.downcast_mut::<Self>() {
                let x = args.get_or_undefined(0).to_u32(context)?;
                let y = args.get_or_undefined(1).to_u32(context)?;
                adb.0.click(x, y).map_err(|e| JsNativeError::typ().with_message(e.to_string()))?;
                return Ok(JsValue::undefined());
            }
        }
        Err(JsNativeError::typ().with_message("'this' is not a Adb object").into())
    }

    pub fn screenshot(this: &JsValue, _args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
        if let Some(object) = this.as_object() {
            if let Some(mut adb) = object.downcast_mut::<Self>() {
                let img = adb.0.screenshot().map_err(|e| JsNativeError::typ().with_message(e.to_string()))?;
                let img = JsImage(img);
                let prototype = context
                    .global_object()
                    .get(JsImage::NAME, context)?
                    .as_object()
                    .unwrap()
                    .get(PROTOTYPE, context)?
                    .as_object()
                    .unwrap()
                    .clone();
                let img: JsValue = JsObject::from_proto_and_data(prototype, ObjectData::native_object(img)).into();
                return Ok(img);
            }
        }
        Err(JsNativeError::typ().with_message("'this' is not a Adb object").into())
    }
}

impl Class for JsAdb {
    const NAME: &'static str = "Adb";

    fn constructor(_this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<Self> {
        let addr = args.get_or_undefined(0).to_string(context)?.to_std_string_escaped();
        let target = args.get_or_undefined(1).to_string(context)?.to_std_string_escaped();
        let bin_path = args.get_or_undefined(2).to_string(context)?.to_std_string_escaped();

        let adb = autogui_core::AdbBuilder::new()
            .with_addr(&addr)
            .with_target(&target)
            .with_bin_path(&bin_path)
            .build()
            .map_err(|e| JsNativeError::typ().with_message(e.to_string()))?;
        Ok(JsAdb(adb))
    }

    fn init(class: &mut ClassBuilder<'_, '_>) -> JsResult<()> {
        class.method("click", 2, NativeFunction::from_fn_ptr(Self::click));
        class.method("screenshot", 0, NativeFunction::from_fn_ptr(Self::screenshot));
        Ok(())
    }
}
