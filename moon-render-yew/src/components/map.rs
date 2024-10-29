use common::{MAX_HEIGHT, MAX_WIDTH};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use yew::prelude::*;

#[function_component(Map)]
pub fn map() -> Html {
    let canvas_ref = use_node_ref();
    let pixels_ref = use_state(Vec::new); // pixel data

    // load the image into the canvas and extract bitmap data
    let load_image = {
        let canvas_ref = canvas_ref.clone();
        let pixels_ref = pixels_ref.clone();
        Callback::from(move |input_event: Event| {
            let input = input_event.target_unchecked_into::<web_sys::HtmlInputElement>();
            if let Some(file_list) = input.files() {
                let file = file_list.get(0).unwrap();

                let reader = web_sys::FileReader::new().unwrap();
                let canvas_ref = canvas_ref.clone();
                let pixels_ref = pixels_ref.clone();
                let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));

                let onload_callback = {
                    let file = file.clone();
                    let image = image.clone(); // clone the html element for the closure
                    Closure::wrap(Box::new(move || {
                        // set image source
                        image
                            .borrow()
                            .set_src(&web_sys::Url::create_object_url_with_blob(&file).unwrap());

                        // wait until the image is fully loaded before drawing
                        let canvas_ref = canvas_ref.clone();
                        let pixels_ref = pixels_ref.clone();
                        let image_clone = image.clone();
                        let onload_image = Closure::wrap(Box::new(move || {
                            let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                            let context = canvas
                                .get_context("2d")
                                .unwrap()
                                .unwrap()
                                .dyn_into::<CanvasRenderingContext2d>()
                                .unwrap();

                            // resize canvas to fit the image's dimensions or scale it to fit the predefined canvas size
                            let canvas_width = canvas.width() as f64;
                            let canvas_height = canvas.height() as f64;
                            let image_width = image_clone.borrow().width() as f64;
                            let image_height = image_clone.borrow().height() as f64;

                            // scale to fit the canvas
                            let scale_x = canvas_width / image_width;
                            let scale_y = canvas_height / image_height;
                            let scale = scale_x.min(scale_y);

                            context.scale(scale, scale).unwrap();
                            context
                                .draw_image_with_html_image_element(&image_clone.borrow(), 0.0, 0.0)
                                .unwrap();

                            // extract image pixel data
                            let image_data = context
                                .get_image_data(
                                    0.0,
                                    0.0,
                                    canvas.width() as f64,
                                    canvas.height() as f64,
                                )
                                .unwrap();
                            let pixels = image_data.data();

                            // store the pixel data
                            pixels_ref.set(pixels.to_vec());
                        })
                            as Box<dyn FnMut()>);

                        image
                            .borrow()
                            .set_onload(Some(onload_image.as_ref().unchecked_ref()));
                        onload_image.forget();
                    }) as Box<dyn FnMut()>)
                };

                reader.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
                reader.read_as_data_url(&file).unwrap();

                onload_callback.forget();
            }
        })
    };

    // recolor specific pixels
    // let recolor_image = {
    //     let canvas_ref = canvas_ref.clone();
    //     let pixels_ref = pixels_ref.clone();
    //     Callback::from(move |_| {
    //         let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
    //         let context = canvas
    //             .get_context("2d")
    //             .unwrap()
    //             .unwrap()
    //             .dyn_into::<CanvasRenderingContext2d>()
    //             .unwrap();

    //         let mut pixels = (*pixels_ref).clone();

    //         // modify specific pixels (e.g: recolor only the top-left 100x100 pixels)
    //         for i in (0..100 * 100 * 4).step_by(4) {
    //             pixels[i] = 0; // R
    //             pixels[i + 1] = 255; // G
    //             pixels[i + 2] = 0; // B
    //             pixels[i + 3] = 255; // A
    //         }

    //         // create new ImageData from modified pixel array
    //         let new_image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
    //             wasm_bindgen::Clamped(&pixels),
    //             canvas.width(),
    //             canvas.height(),
    //         )
    //         .unwrap();

    //         // draw new ImageData to canvas
    //         context.put_image_data(&new_image_data, 0.0, 0.0).unwrap();
    //     })
    // };

    html! {
        <>
        <div class = "outer-map-container">
            <input type="file" accept="moon/jpg" onchange={load_image} />
            <div class = "map-container">
            <canvas ref={canvas_ref} width={format!("{}", MAX_WIDTH)} height={format!("{}", MAX_HEIGHT)}></canvas>
            </div>
            // <Button name = "Recolor image" class_name = "transaction-btn" button_type = "button" on_click={recolor_image}/>
        </div>
        </>
    }
}
