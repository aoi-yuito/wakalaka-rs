// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

const APP_USER_AGENT: &str = concat!(
    std::env!("CARGO_PKG_NAME"),
    "/",
    std::env!("CARGO_PKG_VERSION")
);
const POSTMAN_USER_AGENT: &str = "PostmanRuntime/7.36.0";
