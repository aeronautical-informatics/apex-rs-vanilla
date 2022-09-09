//! Bindings to a vanilla implementation of the ARINC653 P1/P2/P4 API

#![no_std]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

/// This module contains bindings to the services defined by ARINC653
pub mod bindings;

/// This module contains the mappings from [crate::bindings] to [apex_rs::bindings]
pub mod apex;
