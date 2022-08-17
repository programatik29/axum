//! `Either*` types for combining extractors or responses into a single type.
//!
//! # As an extractor
//!
//! ```
//! use axum_extra::either::Either3;
//! use axum::{body::Bytes, Json};
//!
//! async fn handler(
//!     body: Either3<Json<serde_json::Value>, String, Bytes>,
//! ) {
//!     match body {
//!         Either3::E1(json) => { /* ... */ }
//!         Either3::E2(string) => { /* ... */ }
//!         Either3::E3(bytes) => { /* ... */ }
//!     }
//! }
//! #
//! # let _: axum::routing::MethodRouter = axum::routing::get(handler);
//! ```
//!
//! Note that if all the inner extractors reject the request, the rejection from the last
//! extractor will be returned. For the example above that would be [`BytesRejection`].
//!
//! # As a response
//!
//! ```
//! use axum_extra::either::Either3;
//! use axum::{Json, http::StatusCode, response::IntoResponse};
//! use serde_json::{Value, json};
//!
//! async fn handler() -> Either3<Json<Value>, &'static str, StatusCode> {
//!     if something() {
//!         Either3::E1(Json(json!({ "data": "..." })))
//!     } else if something_else() {
//!         Either3::E2("foobar")
//!     } else {
//!         Either3::E3(StatusCode::NOT_FOUND)
//!     }
//! }
//!
//! fn something() -> bool {
//!     // ...
//!     # false
//! }
//!
//! fn something_else() -> bool {
//!     // ...
//!     # false
//! }
//! #
//! # let _: axum::routing::MethodRouter = axum::routing::get(handler);
//! ```
//!
//! The general recommendation is to use [`IntoResponse::into_response`] to return different response
//! types, but if you need to preserve the exact type then `Either*` works as well.
//!
//! [`BytesRejection`]: axum::extract::rejection::BytesRejection
//! [`IntoResponse::into_response`]: https://docs.rs/axum/0.5/axum/response/index.html#returning-different-response-types

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    response::{IntoResponse, Response},
};

/// Combines two extractors or responses into a single type.
///
/// See the [module docs](self) for examples.
#[derive(Debug, Clone)]
pub enum Either<E1, E2> {
    #[allow(missing_docs)]
    E1(E1),
    #[allow(missing_docs)]
    E2(E2),
}

/// Combines three extractors or responses into a single type.
///
/// See the [module docs](self) for examples.
#[derive(Debug, Clone)]
pub enum Either3<E1, E2, E3> {
    #[allow(missing_docs)]
    E1(E1),
    #[allow(missing_docs)]
    E2(E2),
    #[allow(missing_docs)]
    E3(E3),
}

/// Combines four extractors or responses into a single type.
///
/// See the [module docs](self) for examples.
#[derive(Debug, Clone)]
pub enum Either4<E1, E2, E3, E4> {
    #[allow(missing_docs)]
    E1(E1),
    #[allow(missing_docs)]
    E2(E2),
    #[allow(missing_docs)]
    E3(E3),
    #[allow(missing_docs)]
    E4(E4),
}

/// Combines five extractors or responses into a single type.
///
/// See the [module docs](self) for examples.
#[derive(Debug, Clone)]
pub enum Either5<E1, E2, E3, E4, E5> {
    #[allow(missing_docs)]
    E1(E1),
    #[allow(missing_docs)]
    E2(E2),
    #[allow(missing_docs)]
    E3(E3),
    #[allow(missing_docs)]
    E4(E4),
    #[allow(missing_docs)]
    E5(E5),
}

/// Combines six extractors or responses into a single type.
///
/// See the [module docs](self) for examples.
#[derive(Debug, Clone)]
pub enum Either6<E1, E2, E3, E4, E5, E6> {
    #[allow(missing_docs)]
    E1(E1),
    #[allow(missing_docs)]
    E2(E2),
    #[allow(missing_docs)]
    E3(E3),
    #[allow(missing_docs)]
    E4(E4),
    #[allow(missing_docs)]
    E5(E5),
    #[allow(missing_docs)]
    E6(E6),
}

/// Combines seven extractors or responses into a single type.
///
/// See the [module docs](self) for examples.
#[derive(Debug, Clone)]
pub enum Either7<E1, E2, E3, E4, E5, E6, E7> {
    #[allow(missing_docs)]
    E1(E1),
    #[allow(missing_docs)]
    E2(E2),
    #[allow(missing_docs)]
    E3(E3),
    #[allow(missing_docs)]
    E4(E4),
    #[allow(missing_docs)]
    E5(E5),
    #[allow(missing_docs)]
    E6(E6),
    #[allow(missing_docs)]
    E7(E7),
}

/// Combines eight extractors or responses into a single type.
///
/// See the [module docs](self) for examples.
#[derive(Debug, Clone)]
pub enum Either8<E1, E2, E3, E4, E5, E6, E7, E8> {
    #[allow(missing_docs)]
    E1(E1),
    #[allow(missing_docs)]
    E2(E2),
    #[allow(missing_docs)]
    E3(E3),
    #[allow(missing_docs)]
    E4(E4),
    #[allow(missing_docs)]
    E5(E5),
    #[allow(missing_docs)]
    E6(E6),
    #[allow(missing_docs)]
    E7(E7),
    #[allow(missing_docs)]
    E8(E8),
}

macro_rules! impl_traits_for_either {
    (
        $either:ident =>
        [$($ident:ident),* $(,)?],
        $last:ident $(,)?
    ) => {
        #[async_trait]
        impl<S, B, $($ident),*, $last> FromRequest<S, B> for $either<$($ident),*, $last>
        where
            $($ident: FromRequest<S, B>),*,
            $last: FromRequest<S, B>,
            B: Send,
            S: Send,
        {
            type Rejection = $last::Rejection;

            async fn from_request(req: &mut RequestParts<S, B>) -> Result<Self, Self::Rejection> {
                $(
                    if let Ok(value) = req.extract().await {
                        return Ok(Self::$ident(value));
                    }
                )*

                req.extract().await.map(Self::$last)
            }
        }

        impl<$($ident),*, $last> IntoResponse for $either<$($ident),*, $last>
        where
            $($ident: IntoResponse),*,
            $last: IntoResponse,
        {
            fn into_response(self) -> Response {
                match self {
                    $( Self::$ident(value) => value.into_response(), )*
                    Self::$last(value) => value.into_response(),
                }
            }
        }
    };
}

impl_traits_for_either!(Either => [E1], E2);
impl_traits_for_either!(Either3 => [E1, E2], E3);
impl_traits_for_either!(Either4 => [E1, E2, E3], E4);
impl_traits_for_either!(Either5 => [E1, E2, E3, E4], E5);
impl_traits_for_either!(Either6 => [E1, E2, E3, E4, E5], E6);
impl_traits_for_either!(Either7 => [E1, E2, E3, E4, E5, E6], E7);
impl_traits_for_either!(Either8 => [E1, E2, E3, E4, E5, E6, E7], E8);