//! Package speech provides functionality for speech recognizers along with their related configuration and event objects.
mod audio_data_stream;
mod auto_detect_source_language_config;
mod cancellation_details;
mod keyword_recognition_model;
mod recognition_event;
mod session_event;
mod source_language_config;
mod speech_config;
mod speech_recognition_canceled_event;
mod speech_recognition_event;
mod speech_recognition_result;
mod speech_recognizer;
mod speech_synthesis_bookmark_event;
mod speech_synthesis_event;
mod speech_synthesis_result;
mod speech_synthesis_viseme_event;
mod speech_synthesis_word_boundary_event;
mod speech_synthesizer;

// re-export structs directly under speech module
pub use self::audio_data_stream::AudioDataStream;
pub use self::auto_detect_source_language_config::AutoDetectSourceLanguageConfig;
pub use self::cancellation_details::CancellationDetails;
pub use self::keyword_recognition_model::KeywordRecognitionModel;
pub use self::recognition_event::RecognitionEvent;
pub use self::session_event::SessionEvent;
pub use self::source_language_config::SourceLanguageConfig;
pub use self::speech_config::SpeechConfig;
pub use self::speech_recognition_canceled_event::SpeechRecognitionCanceledEvent;
pub use self::speech_recognition_event::SpeechRecognitionEvent;
pub use self::speech_recognition_result::SpeechRecognitionResult;
pub use self::speech_recognizer::SpeechRecognizer;
pub use self::speech_synthesis_bookmark_event::SpeechSynthesisBookmarkEvent;
pub use self::speech_synthesis_event::SpeechSynthesisEvent;
pub use self::speech_synthesis_result::SpeechSynthesisResult;
pub use self::speech_synthesis_viseme_event::SpeechSynthesisVisemeEvent;
pub use self::speech_synthesis_word_boundary_event::SpeechSynthesisWordBoundaryEvent;
pub use self::speech_synthesizer::SpeechSynthesizer;
