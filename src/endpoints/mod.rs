mod bankid_no;
mod bankid_se;
mod document;
mod freja;
mod ftn;
mod mitid;
mod vipps;

#[cfg(feature = "async")]
pub use bankid_no::BankIdNoEndpoint;
#[cfg(feature = "async")]
pub use bankid_se::BankIdSeEndpoint;
#[cfg(feature = "async")]
pub use document::DocumentEndpoint;
#[cfg(feature = "async")]
pub use freja::FrejaEndpoint;
#[cfg(feature = "async")]
pub use ftn::FtnEndpoint;
#[cfg(feature = "async")]
pub use mitid::MitIdEndpoint;
#[cfg(feature = "async")]
pub use vipps::VippsEndpoint;

#[cfg(feature = "blocking")]
pub use bankid_no::BankIdNoBlockingEndpoint;
#[cfg(feature = "blocking")]
pub use bankid_se::BankIdSeBlockingEndpoint;
#[cfg(feature = "blocking")]
pub use document::DocumentBlockingEndpoint;
#[cfg(feature = "blocking")]
pub use freja::FrejaBlockingEndpoint;
#[cfg(feature = "blocking")]
pub use ftn::FtnBlockingEndpoint;
#[cfg(feature = "blocking")]
pub use mitid::MitIdBlockingEndpoint;
#[cfg(feature = "blocking")]
pub use vipps::VippsBlockingEndpoint;
