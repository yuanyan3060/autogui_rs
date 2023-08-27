use std::ffi::c_void;

use boa_engine::{
    class::{Class, ClassBuilder},
    object::{ObjectData, PROTOTYPE},
    Context, JsArgs, JsNativeError, JsObject, JsResult, JsValue, NativeFunction,
};
use boa_gc::Finalize;
use image::{buffer::ConvertBuffer, GrayImage};
use opencv::core::Mat_AUTO_STEP;
use opencv::prelude::*;

use crate::js_math::JsPoint;

#[derive(Debug, Finalize)]
pub struct JsImage(pub image::RgbaImage);
unsafe impl boa_gc::Trace for JsImage {
    boa_gc::empty_trace!();
}

impl JsImage {
    pub fn open(_this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
        let path = args.get_or_undefined(0).to_string(context)?.to_std_string_escaped();
        let img = image::open(&path).map_err(|e| JsNativeError::typ().with_message(e.to_string()))?;
        let img = JsImage(img.into_rgba8());
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

    pub fn save(this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
        if let Some(object) = this.as_object() {
            if let Some(img) = object.downcast_mut::<Self>() {
                let path = args.get_or_undefined(0).to_string(context)?.to_std_string_escaped();
                img.0.save(path).map_err(|e| JsNativeError::typ().with_message(e.to_string()))?;
                return Ok(JsValue::undefined());
            }
        }
        Err(JsNativeError::typ().with_message("'this' is not a Image object").into())
    }

    pub fn show(this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
        if let Some(object) = this.as_object() {
            if let Some(img) = object.downcast_mut::<Self>() {
                let name = args.get_or_undefined(0).to_string(context)?.to_std_string_escaped();
                imageproc::window::display_image(&name, &img.0, img.0.width(), img.0.height());
                return Ok(JsValue::undefined());
            }
        }
        Err(JsNativeError::typ().with_message("'this' is not a Image object").into())
    }

    pub fn height(this: &JsValue, _args: &[JsValue], _context: &mut Context<'_>) -> JsResult<JsValue> {
        if let Some(object) = this.as_object() {
            if let Some(img) = object.downcast_mut::<Self>() {
                return Ok(JsValue::Integer(img.0.height() as i32));
            }
        }
        Err(JsNativeError::typ().with_message("'this' is not a Image object").into())
    }

    pub fn width(this: &JsValue, _args: &[JsValue], _context: &mut Context<'_>) -> JsResult<JsValue> {
        if let Some(object) = this.as_object() {
            if let Some(img) = object.downcast_mut::<Self>() {
                return Ok(JsValue::Integer(img.0.width() as i32));
            }
        }
        Err(JsNativeError::typ().with_message("'this' is not a Image object").into())
    }

    pub fn match_template(this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
        let Some(object) = this.as_object() else {
            return Err(JsNativeError::typ().with_message("'this' is not a Image object").into());
        };
        let Some(img) = object.downcast_mut::<Self>() else {
            return Err(JsNativeError::typ().with_message("'this' is not a Image object").into());
        };
        let Some(object) = args.get(0).and_then(|x| x.as_object()) else {
            return Err(JsNativeError::typ().with_message("'this' is not a Image object").into());
        };
        let Some(template) = object.downcast_mut::<Self>() else {
            return Err(JsNativeError::typ().with_message("'this' is not a Image object").into());
        };
        let threshold = args.get(1).and_then(|x| x.to_number(context).ok()).unwrap_or(0.85);
        let mut img: GrayImage = img.0.convert();
        let mut template: GrayImage = template.0.convert();
        let img = unsafe {
            Mat::new_rows_cols_with_data(
                img.height() as i32,
                img.width() as i32,
                opencv::core::CV_8U,
                img.as_mut_ptr() as *mut c_void,
                Mat_AUTO_STEP,
            )
            .map_err(|e| JsNativeError::typ().with_message(e.to_string()))?
        };
        let template = unsafe {
            Mat::new_rows_cols_with_data(
                template.height() as i32,
                template.width() as i32,
                opencv::core::CV_8U,
                template.as_mut_ptr() as *mut c_void,
                Mat_AUTO_STEP,
            )
            .map_err(|e| JsNativeError::typ().with_message(e.to_string()))?
        };
        let mut result = opencv::core::Mat::default();
        opencv::imgproc::match_template(
            &img,
            &template,
            &mut result,
            opencv::imgproc::TM_CCORR_NORMED,
            &opencv::core::Mat::default(),
        )
        .map_err(|e| JsNativeError::typ().with_message(e.to_string()))?;
        let mut min_val: f64 = 0.0;
        let mut max_val: f64 = 0.0;
        let mut min_loc = opencv::core::Point::new(0, 0);
        let mut max_loc = opencv::core::Point::new(0, 0);
        let mask = Mat::default();
        opencv::core::min_max_loc(
            &result,
            Some(&mut min_val),
            Some(&mut max_val),
            Some(&mut min_loc),
            Some(&mut max_loc),
            &mask,
        )
        .map_err(|e| JsNativeError::typ().with_message(e.to_string()))?;
        if max_val < threshold {
            Ok(JsValue::Null)
        } else {
            let prototype = context
                .global_object()
                .get(JsPoint::NAME, context)?
                .as_object()
                .unwrap()
                .get(PROTOTYPE, context)?
                .as_object()
                .unwrap()
                .clone();
            let point = JsPoint { x: max_loc.x, y: max_loc.y };
            let point: JsValue = JsObject::from_proto_and_data(prototype, ObjectData::native_object(point)).into();
            Ok(point)
        }
    }
}

impl Class for JsImage {
    const NAME: &'static str = "Image";

    fn constructor(_this: &JsValue, _args: &[JsValue], _context: &mut Context<'_>) -> JsResult<Self> {
        Err(JsNativeError::typ().with_message("Image obj can not constructor from js").into())
    }

    fn init(class: &mut ClassBuilder<'_, '_>) -> JsResult<()> {
        class.static_method("open", 1, NativeFunction::from_fn_ptr(Self::open));
        class.method("height", 0, NativeFunction::from_fn_ptr(Self::height));
        class.method("width", 0, NativeFunction::from_fn_ptr(Self::width));
        class.method("save", 1, NativeFunction::from_fn_ptr(Self::save));
        class.method("show", 1, NativeFunction::from_fn_ptr(Self::show));
        class.method("match_template", 1, NativeFunction::from_fn_ptr(Self::match_template));
        Ok(())
    }
}
