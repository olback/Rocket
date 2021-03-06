#![recursion_limit="256"]

#![doc(html_root_url = "https://api.rocket.rs/v0.5")]
#![doc(html_favicon_url = "https://rocket.rs/images/favicon.ico")]
#![doc(html_logo_url = "https://rocket.rs/images/logo-boxed.png")]
#![cfg_attr(nightly, feature(doc_cfg))]

#![warn(rust_2018_idioms)]

//! # Rocket - Core API Documentation
//!
//! Hello, and welcome to the core Rocket API documentation!
//!
//! This API documentation is highly technical and is purely a reference.
//! There's an [overview] of Rocket on the main site as well as a [full,
//! detailed guide]. If you'd like pointers on getting started, see the
//! [quickstart] or [getting started] chapters of the guide.
//!
//! You may also be interested in looking at the
//! [`rocket_contrib`](../rocket_contrib) documentation, which contains
//! automatic JSON (de)serialiazation, templating support, static file serving,
//! and other useful features.
//!
//! [overview]: https://rocket.rs/v0.5/overview
//! [full, detailed guide]: https://rocket.rs/v0.5/guide
//! [quickstart]: https://rocket.rs/v0.5/guide/quickstart
//! [getting started]: https://rocket.rs/v0.5/guide/getting-started
//!
//! ## Libraries
//!
//! Rocket's functionality is split into two crates:
//!
//!   1. Core - This core library. Needed by every Rocket application.
//!   2. [Contrib](../rocket_contrib) - Provides useful functionality for many
//!      Rocket applications. Completely optional.
//!
//! ## Usage
//!
//! Depend on `rocket` in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rocket = "0.5.0-dev"
//! ```
//!
//! See the [guide](https://rocket.rs/v0.5/guide) for more information on how to
//! write Rocket applications. Here's a simple example to get you started:
//!
//! ```rust,no_run
//! #[macro_use] extern crate rocket;
//!
//! #[get("/")]
//! fn hello() -> &'static str {
//!     "Hello, world!"
//! }
//!
//! #[launch]
//! fn rocket() -> rocket::Rocket {
//!     rocket::ignite().mount("/", routes![hello])
//! }
//! ```
//!
//! ## Features
//!
//! The `secrets` feature, which enables [private cookies], is enabled by
//! default. This necessitates pulling in additional dependencies. To avoid
//! these dependencies when your application does not use private cookies,
//! disable the `secrets` feature:
//!
//! ```toml
//! [dependencies]
//! rocket = { version = "0.5.0-dev", default-features = false }
//! ```
//!
//! [private cookies]: crate::http::CookieJar#private-cookies
//!
//! ## Configuration
//!
//! Rocket and Rocket libraries are configured via the `Rocket.toml` file and/or
//! `ROCKET_{PARAM}` environment variables. For more information on how to
//! configure Rocket, see the [configuration section] of the guide as well as
//! the [`config`] module documentation.
//!
//! [configuration section]: https://rocket.rs/v0.5/guide/configuration/
//!
//! ## Testing
//!
//! The [`local`] module contains structures that facilitate unit and
//! integration testing of a Rocket application. The top-level [`local`] module
//! documentation and the [testing chapter of the guide] include detailed
//! examples.
//!
//! [testing chapter of the guide]: https://rocket.rs/v0.5/guide/testing/#testing

#[allow(unused_imports)] #[macro_use] extern crate rocket_codegen;
pub use rocket_codegen::*;
pub use async_trait::*;

#[macro_use] extern crate log;

#[doc(hidden)]
pub use yansi;
pub use futures;
pub use tokio;

#[doc(hidden)] #[macro_use] pub mod logger;
#[macro_use] pub mod outcome;
pub mod local;
pub mod request;
pub mod response;
pub mod config;
pub mod data;
pub mod handler;
pub mod fairing;
pub mod error;
pub mod catcher;

// Reexport of HTTP everything.
pub mod http {
    //! Types that map to concepts in HTTP.
    //!
    //! This module exports types that map to HTTP concepts or to the underlying
    //! HTTP library when needed.

    #[doc(inline)]
    pub use rocket_http::*;
}

mod shutdown;
mod router;
mod rocket;
mod codegen;
mod ext;

#[doc(inline)] pub use crate::response::Response;
#[doc(hidden)] pub use crate::codegen::{StaticRouteInfo, StaticCatcherInfo};
#[doc(inline)] pub use crate::data::Data;
#[doc(inline)] pub use crate::config::Config;
#[doc(inline)] pub use crate::catcher::Catcher;
pub use crate::router::Route;
pub use crate::request::{Request, State};
pub use crate::rocket::{Cargo, Rocket};
pub use crate::shutdown::Shutdown;

/// Alias to [`Rocket::ignite()`] Creates a new instance of `Rocket`.
pub fn ignite() -> Rocket {
    Rocket::ignite()
}

/// Alias to [`Rocket::custom()`]. Creates a new instance of `Rocket` with a
/// custom configuration.
pub fn custom(config: Config) -> Rocket {
    Rocket::custom(config)
}

// TODO.async: More thoughtful plan for async tests
/// WARNING: This is unstable! Do not use this method outside of Rocket!
#[doc(hidden)]
pub fn async_test<R>(fut: impl std::future::Future<Output = R> + Send) -> R {
    tokio::runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .expect("create tokio runtime")
        .block_on(fut)
}

/// WARNING: This is unstable! Do not use this method outside of Rocket!
#[doc(hidden)]
pub fn async_main<R>(fut: impl std::future::Future<Output = R> + Send) -> R {
    tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .expect("create tokio runtime")
        .block_on(fut)
}
