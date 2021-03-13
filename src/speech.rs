use crate::audio::AudioConfig;
use crate::common::PropertyCollection;
use crate::error::{convert_err, Result};
use crate::ffi::{
    property_bag_release, recognizer_create_speech_recognizer_from_config,
    recognizer_event_handle_release, recognizer_get_property_bag, recognizer_handle_release,
    recognizer_session_event_get_session_id, speech_config_from_subscription,
    speech_config_get_property_bag, speech_config_release, SmartHandle, SPXEVENTHANDLE, SPXHANDLE,
    SPXHANDLE_EMPTY, SPXRECOHANDLE, SPXSPEECHCONFIGHANDLE,
};
use std::ffi::CString;
use std::os::raw::{c_char, c_void};

#[derive(Debug)]
pub struct SpeechConfig {
    pub handle: SmartHandle<SPXSPEECHCONFIGHANDLE>,
    properties: PropertyCollection,
}

impl SpeechConfig {
    fn from_handle(handle: SPXHANDLE) -> Result<SpeechConfig> {
        let mut prop_bag_handle = SPXHANDLE_EMPTY;
        unsafe {
            let ret = speech_config_get_property_bag(handle, &mut prop_bag_handle);
            convert_err(ret, "SpeechConfig::from_handle error")?;
        }
        let mut property_bag = PropertyCollection {
            handle: SmartHandle::create(
                "PropertyCollection",
                prop_bag_handle,
                property_bag_release,
            ),
        };

        property_bag.set_property_by_string("SPEECHSDK-SPEECH-CONFIG-SYSTEM-LANGUAGE", "Rust")?;

        let result = SpeechConfig {
            handle: SmartHandle::create("SpeechConfig", handle, speech_config_release),
            properties: property_bag,
        };
        Ok(result)
    }

    pub fn from_subscription<S>(subscription: S, region: S) -> Result<SpeechConfig>
    where
        S: Into<Vec<u8>>,
    {
        let c_sub = CString::new(subscription)?;
        let c_region = CString::new(region)?;
        let mut handle = SPXHANDLE_EMPTY;
        unsafe {
            let ret =
                speech_config_from_subscription(&mut handle, c_sub.as_ptr(), c_region.as_ptr());
            convert_err(ret, "SpeechConfig::from_subscription error")?
        }
        SpeechConfig::from_handle(handle)
    }
}

#[derive(Debug)]
pub struct SpeechRecognizer {
    handle: SmartHandle<SPXRECOHANDLE>,
    properties: PropertyCollection,
    speech_config: SpeechConfig,
    audio_config: AudioConfig,
}

impl SpeechRecognizer {
    fn from_handle(
        handle: SPXHANDLE,
        speech_config: SpeechConfig,
        audio_config: AudioConfig,
    ) -> Result<SpeechRecognizer> {
        let mut prop_bag_handle = SPXHANDLE_EMPTY;
        unsafe {
            let ret = recognizer_get_property_bag(handle, &mut prop_bag_handle);
            convert_err(ret, "SpeechRecognizer::from_handle error")?;
        }
        let property_bag = PropertyCollection {
            handle: SmartHandle::create(
                "PropertyCollection",
                prop_bag_handle,
                property_bag_release,
            ),
        };

        let result = SpeechRecognizer {
            handle: SmartHandle::create("SpeechRecognizer", handle, recognizer_handle_release),
            properties: property_bag,
            speech_config,
            audio_config,
        };
        Ok(result)
    }

    pub fn from_config(
        speech_config: SpeechConfig,
        audio_config: AudioConfig,
    ) -> Result<SpeechRecognizer> {
        let mut handle = SPXHANDLE_EMPTY;
        unsafe {
            convert_err(
                recognizer_create_speech_recognizer_from_config(
                    &mut handle,
                    speech_config.handle.get(),
                    audio_config.handle.get(),
                ),
                "SpeechRecognizer.from_config error",
            )?;
        }
        SpeechRecognizer::from_handle(handle, speech_config, audio_config)
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_session_started(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _pvContext: *mut c_void,
) {
    unimplemented!();
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_session_stopped(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _pvContext: *mut c_void,
) {
    unimplemented!();
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_speech_start_detected(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _vpvContext: *mut c_void,
) {
    unimplemented!();
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_speech_end_detected(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _pvContext: *mut c_void,
) {
    unimplemented!();
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_canceled(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _pvContext: *mut c_void,
) {
    unimplemented!();
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_recognizing(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _pvContext: *mut c_void,
) {
    unimplemented!();
}

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe extern "C" fn cb_recognized(
    _hreco: SPXRECOHANDLE,
    _hevent: SPXEVENTHANDLE,
    _pvContext: *mut c_void,
) {
    unimplemented!();
}

#[derive(Debug)]
pub struct SessionEvent {
    session_id: String,
    handle: SmartHandle<SPXEVENTHANDLE>,
}

impl SessionEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<SessionEvent> {
        //allocate zeroed buffer and convert to unsafe mutable ptr
        let buffer: *mut u8 = vec![0u8; 37].as_mut_ptr();
        unsafe {
            let ret = recognizer_session_event_get_session_id(handle, buffer as *mut c_char, 37);
            convert_err(ret, "SessionEvent::from_handle error")?;
            // TBD:not sure whether recognizer_session_event_get_session_id will allocate
            // 37 bytes exactly or it might allocate less? In that case we would have to
            // offset pointer byte by byte until we find terminating nul char(0) of C string
            // see also https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr
            let session_id_slice = std::slice::from_raw_parts(buffer, 37);
            let session_id = String::from_utf8(session_id_slice.to_vec())?;
            Ok(SessionEvent {
                session_id,
                handle: SmartHandle::create(
                    "SessionEvent",
                    handle,
                    recognizer_event_handle_release,
                ),
            })
        }
    }
}
