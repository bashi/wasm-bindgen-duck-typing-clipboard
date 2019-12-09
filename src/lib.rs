use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Blob;

#[wasm_bindgen]
extern "C" {
    pub type Clipboard;

    #[wasm_bindgen(method)]
    pub fn read(this: &Clipboard) -> js_sys::Promise;

    #[wasm_bindgen(method)]
    pub fn readText(this: &Clipboard) -> js_sys::Promise;

    #[wasm_bindgen(method)]
    pub fn write(this: &Clipboard, data: JsValue) -> js_sys::Promise;

    pub type ClipboardItem;

    #[wasm_bindgen(method, getter)]
    pub fn types(this: &ClipboardItem) -> Vec<JsValue>;

    #[wasm_bindgen(method)]
    pub fn getType(this: &ClipboardItem, type_: JsValue) -> js_sys::Promise;
}

async fn fetch_as_blob(url: &str) -> Result<Blob, JsValue> {
    use web_sys::{Request, RequestInit, RequestMode, Response};

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().expect("window");
    let response = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response = response.dyn_into::<Response>().unwrap();

    let blob = JsFuture::from(response.blob()?).await?;
    let blob = blob.dyn_into::<Blob>().unwrap();

    Ok(blob)
}

#[wasm_bindgen]
pub fn read_text_from_clipboard(clipboard: &Clipboard) -> JsValue {
    let text = clipboard.readText();
    text.into()
}

// TODO: Fix, this isn't working yet.
#[wasm_bindgen]
pub async fn fetch_and_copy_to_clipboard(clipboard: Clipboard, url: String) -> Result<(), JsValue> {
    let blob = fetch_as_blob(&url).await?;

    let item = Object::new();
    Reflect::set(&item, &blob.type_().into(), &blob)?;
    let items = js_sys::Array::new();
    items.push(&item);
    JsFuture::from(clipboard.write(items.into())).await?;

    Ok(())
}

#[wasm_bindgen]
pub async fn read_blobs_from_clipboard(clipboard: Clipboard) -> Result<JsValue, JsValue> {
    let items = JsFuture::from(clipboard.read()).await?;
    let items = items.dyn_into::<Array>()?;

    let blobs = Array::new();
    for item in items.values() {
        let item = ClipboardItem::unchecked_from_js(item?);
        for item_type in item.types() {
            let blob = JsFuture::from(item.getType(item_type)).await?;
            blobs.push(&blob);
        }
    }

    Ok(blobs.into())
}
