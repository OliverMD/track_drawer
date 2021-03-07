use crate::js_sys::Array;
use seed::prelude::{ElRef, JsValue};
use web_sys::{Blob, BlobPropertyBag, XmlSerializer};

/// Grabs the content on an SVG node & opens a download dialog
pub fn download_svg(el_ref: &ElRef<web_sys::HtmlElement>) {
    let svg_buf = XmlSerializer::new()
        .unwrap()
        .serialize_to_string(&el_ref.shared_node_ws.clone_inner().unwrap())
        .unwrap();

    let mut blob_type = BlobPropertyBag::new();
    blob_type.type_("image/svg+xml;charset=utf-8");

    let arr = Array::new_with_length(1);
    arr.set(0, JsValue::from_str(&svg_buf));

    let blob = Blob::new_with_str_sequence_and_options(&JsValue::from(arr), &blob_type).unwrap();
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
    let document = web_sys::window().unwrap().document().unwrap();
    let elem = document.create_element("a").unwrap();
    elem.set_attribute("href", &url).unwrap();
    elem.set_attribute("download", "Track Image").unwrap();
    let event = document.create_event("MouseEvents").unwrap();
    event.init_event("click");
    document.body().unwrap().append_with_node_1(&elem).unwrap();
    elem.dispatch_event(&event).unwrap();
    document.body().unwrap().remove_child(&elem).unwrap();
}
