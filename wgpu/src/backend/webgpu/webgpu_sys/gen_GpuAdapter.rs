// DO NOT EDIT THIS FILE!
//
// This module part of a subset of web-sys that is used by wgpu's webgpu backend.
//
// If you want to improve the generated code, please submit a PR to the https://github.com/rustwasm/wasm-bindgen repository.
//
// This file was generated by the `cargo xtask vendor-web-sys --version 0.2.91` command.
#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUAdapter , typescript_type = "GPUAdapter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuAdapter` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuAdapter;

    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapter" , js_name = features)]
    #[doc = "Getter for the `features` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/features)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`, `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn features(this: &GpuAdapter) -> GpuSupportedFeatures;

    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapter" , js_name = limits)]
    #[doc = "Getter for the `limits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/limits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`, `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn limits(this: &GpuAdapter) -> GpuSupportedLimits;

    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapter" , js_name = isFallbackAdapter)]
    #[doc = "Getter for the `isFallbackAdapter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/isFallbackAdapter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_fallback_adapter(this: &GpuAdapter) -> bool;

    # [wasm_bindgen (method , structural , js_class = "GPUAdapter" , js_name = requestAdapterInfo)]
    #[doc = "The `requestAdapterInfo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/requestAdapterInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_adapter_info(this: &GpuAdapter) -> ::js_sys::Promise;

    # [wasm_bindgen (method , structural , js_class = "GPUAdapter" , js_name = requestDevice)]
    #[doc = "The `requestDevice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/requestDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_device(this: &GpuAdapter) -> ::js_sys::Promise;

    # [wasm_bindgen (method , structural , js_class = "GPUAdapter" , js_name = requestDevice)]
    #[doc = "The `requestDevice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/requestDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`, `GpuDeviceDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_device_with_descriptor(
        this: &GpuAdapter,
        descriptor: &GpuDeviceDescriptor,
    ) -> ::js_sys::Promise;
}
