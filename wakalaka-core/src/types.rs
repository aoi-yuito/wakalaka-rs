// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::serenity_prelude as serenity;

use crate::Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type FrameworkError<'a> = poise::FrameworkError<'a, Data, Error>;

pub type Throwable<T> = Result<T, Error>;
pub type SqlxThrowable<T> = Result<T, sqlx::Error>;

pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, Error>;

pub type SClient = serenity::Client;
pub type SContext = serenity::Context;
pub type SReady = serenity::Ready;
