//! Outfox library crate
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "doubao")]
#[cfg_attr(docsrs, doc(cfg(feature = "doubao")))]
pub use outfox_doubao as doubao;
#[cfg(feature = "openai")]
#[cfg_attr(docsrs, doc(cfg(feature = "openai")))]
pub use outfox_openai as openai;
#[cfg(feature = "zhipu")]
#[cfg_attr(docsrs, doc(cfg(feature = "zhipu")))]
pub use outfox_zhipu as zhipu;
