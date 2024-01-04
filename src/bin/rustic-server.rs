//! Main entry point for RusticServer

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use rustic_server::application::APP;

/// Boot RusticServer
fn main() {
    abscissa_core::boot(&APP);
}
