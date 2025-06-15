use js_sys::{Array, Date, JsString};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::{
    Element, HtmlAnchorElement, HtmlButtonElement, HtmlInputElement, HtmlPreElement,
    HtmlSpanElement,
};

#[wasm_bindgen]
pub struct KeyGeneratorUI {
    generator: Option<crate::key_generator::KeyGenerator>,
    pattern_input: HtmlInputElement,
    generate_button: HtmlButtonElement,
    stop_button: HtmlButtonElement,
    download_button: HtmlButtonElement,
    attempts_span: HtmlSpanElement,
    rate_span: HtmlSpanElement,
    error_span: HtmlSpanElement,
    public_key: HtmlPreElement,
    private_key: HtmlPreElement,
    results: Element,
    last_update: f64,
    attempts_at_last_update: u64,
}

#[wasm_bindgen]
impl KeyGeneratorUI {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<KeyGeneratorUI, JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let pattern_input = document
            .get_element_by_id("pattern")
            .unwrap()
            .dyn_into::<HtmlInputElement>()?;

        let generate_button = document
            .get_element_by_id("generate")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()?;

        let stop_button = document
            .get_element_by_id("stop")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()?;

        let download_button = document
            .get_element_by_id("download")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()?;

        let attempts_span = document
            .get_element_by_id("attempts")
            .unwrap()
            .dyn_into::<HtmlSpanElement>()?;

        let rate_span = document
            .get_element_by_id("rate")
            .unwrap()
            .dyn_into::<HtmlSpanElement>()?;

        let error_span = document
            .get_element_by_id("error")
            .unwrap()
            .dyn_into::<HtmlSpanElement>()?;

        let public_key = document
            .get_element_by_id("public-key")
            .unwrap()
            .dyn_into::<HtmlPreElement>()?;

        let private_key = document
            .get_element_by_id("private-key")
            .unwrap()
            .dyn_into::<HtmlPreElement>()?;

        let results = document.get_element_by_id("results").unwrap();

        let ui = KeyGeneratorUI {
            generator: None,
            pattern_input,
            generate_button,
            stop_button,
            download_button,
            attempts_span,
            rate_span,
            error_span,
            public_key,
            private_key,
            results,
            last_update: Date::now(),
            attempts_at_last_update: 0,
        };

        let mut ui = ui;
        ui.setup_event_listeners()?;
        Ok(ui)
    }

    fn setup_event_listeners(&mut self) -> Result<(), JsValue> {
        let generate_button = self.generate_button.clone();
        let pattern_input = self.pattern_input.clone();
        let mut ui = self.clone();
        let generate_callback = Closure::wrap(Box::new(move || {
            let pattern = pattern_input.value();
            if pattern.is_empty() {
                return;
            }
            ui.start_generation().unwrap();
        }) as Box<dyn FnMut()>);
        generate_button.add_event_listener_with_callback(
            "click",
            generate_callback.as_ref().unchecked_ref(),
        )?;
        generate_callback.forget();

        let stop_button = self.stop_button.clone();
        let mut ui = self.clone();
        let stop_callback = Closure::wrap(Box::new(move || {
            ui.stop_generation();
        }) as Box<dyn FnMut()>);
        stop_button
            .add_event_listener_with_callback("click", stop_callback.as_ref().unchecked_ref())?;
        stop_callback.forget();

        let download_button = self.download_button.clone();
        let ui = self.clone();
        let download_callback = Closure::wrap(Box::new(move || {
            ui.download_keys().unwrap();
        }) as Box<dyn FnMut()>);
        download_button.add_event_listener_with_callback(
            "click",
            download_callback.as_ref().unchecked_ref(),
        )?;
        download_callback.forget();

        Ok(())
    }

    fn start_generation(&mut self) -> Result<(), JsValue> {
        let pattern = self.pattern_input.value();
        if pattern.is_empty() {
            return Ok(());
        }

        self.generator = Some(crate::key_generator::KeyGenerator::new(vec![pattern])?);
        self.generate_button.set_disabled(true);
        self.stop_button.set_disabled(false);
        self.download_button.set_disabled(true);
        self.error_span.style().set_property("display", "none")?;
        self.results.class_list().add_1("hidden")?;

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let pattern_input = self.pattern_input.clone();
        let generate_button = self.generate_button.clone();
        let stop_button = self.stop_button.clone();
        let download_button = self.download_button.clone();
        let attempts_span = self.attempts_span.clone();
        let rate_span = self.rate_span.clone();
        let error_span = self.error_span.clone();
        let public_key = self.public_key.clone();
        let private_key = self.private_key.clone();
        let results = self.results.clone();
        let mut last_update = self.last_update;
        let mut attempts_at_last_update = self.attempts_at_last_update;
        let mut generator = self.generator.take();

        let generate_callback = Closure::wrap(Box::new(move || {
            if generator.is_none() {
                return;
            }

            let gen = generator.as_mut().unwrap();
            if let Some(key_pair) = gen.generate_batch(1000) {
                public_key.set_text_content(Some(&key_pair.public_key()));
                private_key.set_text_content(Some(&key_pair.private_key()));
                results.class_list().remove_1("hidden").unwrap();
                download_button.set_disabled(false);
                generate_button.set_disabled(false);
                stop_button.set_disabled(true);
                generator = None;
                return;
            }

            attempts_span.set_text_content(Some(&gen.get_attempts().to_string()));

            let now = Date::now();
            let elapsed = (now - last_update) / 1000.0;
            if elapsed >= 1.0 {
                let attempts = gen.get_attempts();
                let rate = (attempts - attempts_at_last_update) as f64 / elapsed;
                rate_span.set_text_content(Some(&format!("{:.0}", rate)));
                last_update = now;
                attempts_at_last_update = attempts;
            }

            if let Some(error) = gen.get_last_error() {
                error_span.set_text_content(Some(&error));
                error_span.style().set_property("display", "block").unwrap();
                generate_button.set_disabled(false);
                stop_button.set_disabled(true);
                generator = None;
            }
        }) as Box<dyn FnMut()>);

        web_sys::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                generate_callback.as_ref().unchecked_ref(),
                0,
            )?;
        generate_callback.forget();

        Ok(())
    }

    fn stop_generation(&mut self) {
        self.generate_button.set_disabled(false);
        self.stop_button.set_disabled(true);
        self.generator = None;
    }

    fn generate_batch(&mut self) -> Result<(), JsValue> {
        console::log_1(&JsValue::from_str("generate_batch called"));
        if self.generator.is_none() {
            console::log_1(&JsValue::from_str("No generator initialized"));
            return Ok(());
        }
        let generator = self.generator.as_mut().unwrap();
        console::log_1(&JsValue::from_str(&format!("Generator state: {:?}", generator)));

        if let Some(key_pair) = generator.generate_batch(1000) {
            console::log_1(&JsValue::from_str("Found matching key pair"));
            self.public_key.set_text_content(Some(&key_pair.public_key()));
            self.private_key.set_text_content(Some(&key_pair.private_key()));
            self.results.class_list().remove_1("hidden")?;
            self.download_button.set_disabled(false);
            self.stop_generation();
            return Ok(());
        }
        console::log_1(&JsValue::from_str("No key pair found in this batch"));

        self.attempts_span.set_text_content(Some(&generator.get_attempts().to_string()));

        let now = Date::now();
        let elapsed = (now - self.last_update) / 1000.0;
        if elapsed >= 1.0 {
            let attempts = generator.get_attempts();
            let rate = (attempts - self.attempts_at_last_update) as f64 / elapsed;
            self.rate_span.set_text_content(Some(&format!("{:.0}", rate)));
            self.last_update = now;
            self.attempts_at_last_update = attempts;
        }

        if let Some(error) = generator.get_last_error() {
            console::log_1(&JsValue::from_str(&format!("Error: {}", error)));
            self.error_span.set_text_content(Some(&error));
            self.error_span.style().set_property("display", "block")?;
            self.stop_generation();
        }

        Ok(())
    }

    fn download_keys(&self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let timestamp = Date::now().to_string();
        let public_key_blob = web_sys::Blob::new_with_str_sequence(&Array::of1(&JsString::from(
            self.public_key.text_content().unwrap(),
        )))?;
        let private_key_blob = web_sys::Blob::new_with_str_sequence(&Array::of1(&JsString::from(
            self.private_key.text_content().unwrap(),
        )))?;

        let public_key_url = web_sys::Url::create_object_url_with_blob(&public_key_blob)?;
        let private_key_url = web_sys::Url::create_object_url_with_blob(&private_key_blob)?;

        let public_key_link = document
            .create_element("a")?
            .dyn_into::<HtmlAnchorElement>()?;
        public_key_link.set_attribute("href", &public_key_url)?;
        public_key_link.set_attribute("download", &format!("id_ed25519_{}.pub", timestamp))?;
        document.body().unwrap().append_child(&public_key_link)?;
        public_key_link.click();
        document.body().unwrap().remove_child(&public_key_link)?;

        let private_key_link = document
            .create_element("a")?
            .dyn_into::<HtmlAnchorElement>()?;
        private_key_link.set_attribute("href", &private_key_url)?;
        private_key_link.set_attribute("download", &format!("id_ed25519_{}", timestamp))?;
        document.body().unwrap().append_child(&private_key_link)?;
        private_key_link.click();
        document.body().unwrap().remove_child(&private_key_link)?;

        web_sys::Url::revoke_object_url(&public_key_url)?;
        web_sys::Url::revoke_object_url(&private_key_url)?;

        Ok(())
    }
}

impl Clone for KeyGeneratorUI {
    fn clone(&self) -> Self {
        Self {
            generator: None,
            pattern_input: self.pattern_input.clone(),
            generate_button: self.generate_button.clone(),
            stop_button: self.stop_button.clone(),
            download_button: self.download_button.clone(),
            attempts_span: self.attempts_span.clone(),
            rate_span: self.rate_span.clone(),
            error_span: self.error_span.clone(),
            public_key: self.public_key.clone(),
            private_key: self.private_key.clone(),
            results: self.results.clone(),
            last_update: self.last_update,
            attempts_at_last_update: self.attempts_at_last_update,
        }
    }
}
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let mut ui = KeyGeneratorUI::new()?;
    ui.start_generation()?;
    Ok(())
}
