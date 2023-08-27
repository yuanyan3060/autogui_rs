use boa_engine::{
    class::{Class, ClassBuilder},
    object::FunctionObjectBuilder,
    property::PropertyDescriptorBuilder,
    Context, JsArgs, JsNativeError, JsResult, JsValue, NativeFunction,
};
use boa_gc::Finalize;

#[derive(Debug, Finalize)]
pub struct JsPoint {
    pub x: i32,
    pub y: i32,
}
unsafe impl boa_gc::Trace for JsPoint {
    boa_gc::empty_trace!();
}

impl JsPoint {
    pub fn get_x(this: &JsValue, _args: &[JsValue], _context: &mut Context<'_>) -> JsResult<JsValue> {
        if let Some(object) = this.as_object() {
            if let Some(p) = object.downcast_mut::<Self>() {
                return Ok(JsValue::Integer(p.x));
            }
        }
        Err(JsNativeError::typ().with_message("'this' is not a Point object").into())
    }

    pub fn set_x(this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
        if let Some(object) = this.as_object() {
            if let Some(mut p) = object.downcast_mut::<Self>() {
                let x = args.get_or_undefined(0).to_i32(context)?;
                p.x = x;
                return Ok(JsValue::null());
            }
        }
        Err(JsNativeError::typ().with_message("'this' is not a Point object").into())
    }

    pub fn get_y(this: &JsValue, _args: &[JsValue], _context: &mut Context<'_>) -> JsResult<JsValue> {
        if let Some(object) = this.as_object() {
            if let Some(p) = object.downcast_mut::<Self>() {
                return Ok(JsValue::Integer(p.y));
            }
        }
        Err(JsNativeError::typ().with_message("'this' is not a Point object").into())
    }

    pub fn set_y(this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
        if let Some(object) = this.as_object() {
            if let Some(mut p) = object.downcast_mut::<Self>() {
                let y = args.get_or_undefined(0).to_i32(context)?;
                p.y = y;
                return Ok(JsValue::null());
            }
        }
        Err(JsNativeError::typ().with_message("'this' is not a Point object").into())
    }
}
impl Class for JsPoint {
    const NAME: &'static str = "Point";

    fn constructor(_this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<Self> {
        let x = args.get_or_undefined(0).to_i32(context)?;
        let y = args.get_or_undefined(1).to_i32(context)?;

        Ok(JsPoint { x, y })
    }

    fn init(class: &mut ClassBuilder<'_, '_>) -> JsResult<()> {
        let get_x = FunctionObjectBuilder::new(class.context(), NativeFunction::from_fn_ptr(Self::get_x))
            .name("get_x")
            .length(0)
            .constructor(false)
            .build();
        let set_x = FunctionObjectBuilder::new(class.context(), NativeFunction::from_fn_ptr(Self::set_x))
            .name("set_x")
            .length(0)
            .constructor(false)
            .build();
        let get_y = FunctionObjectBuilder::new(class.context(), NativeFunction::from_fn_ptr(Self::get_y))
            .name("get_y")
            .length(0)
            .constructor(false)
            .build();
        let set_y = FunctionObjectBuilder::new(class.context(), NativeFunction::from_fn_ptr(Self::set_y))
            .name("set_y")
            .length(0)
            .constructor(false)
            .build();
        let p_x = PropertyDescriptorBuilder::new()
            .configurable(false)
            .enumerable(false)
            .get(get_x)
            .set(set_x)
            .build();
        let p_y = PropertyDescriptorBuilder::new()
            .configurable(false)
            .enumerable(false)
            .get(get_y)
            .set(set_y)
            .build();
        class.property_descriptor("x", p_x);
        class.property_descriptor("y", p_y);
        Ok(())
    }
}
