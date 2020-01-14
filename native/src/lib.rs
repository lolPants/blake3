extern crate neon;
extern crate blake3;

use std::ops::Deref;
use neon::prelude::*;

struct BackgroundTask {
    argument: Vec<u8>
}

impl Task for BackgroundTask {
    type Output = Vec<u8>;
    type Error = ();
    type JsEvent = JsBuffer;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        let hash = blake3::hash(&self.argument);

        let hex = hash.as_bytes();
        Ok(hex.to_vec())
    }

    fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        let vec: Vec<u8> = result.unwrap();
        let js_buf: Handle<JsBuffer> = JsBuffer::new(&mut cx, vec.len() as u32)?;

        for (i, obj) in vec.iter().enumerate() {
            let js_byte = cx.number(*obj as f64);
            js_buf.set(&mut cx, i as u32, js_byte).unwrap();
        }

        Ok(js_buf)
    }
}

pub fn calculate_blake3(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let buf_handle: Handle<JsBuffer> = cx.argument::<JsBuffer>(0)?;
    let mut vec: Vec<u8> = Vec::new();
    cx.borrow(&buf_handle, |data| {
        let s: &[u8] = data.deref().as_slice::<u8>();
        vec.extend_from_slice(&s);
    });

    let f = cx.argument::<JsFunction>(1)?;
    let task = BackgroundTask { argument: vec };
    task.schedule(f);

    Ok(cx.undefined())
}

register_module!(mut m, {
    m.export_function("blake3", calculate_blake3)
});
