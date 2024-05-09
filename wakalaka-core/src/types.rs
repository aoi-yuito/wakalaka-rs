// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::serenity_prelude as serenity;

use crate::Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Throwable<T> = Result<T, Error>;

/* serde_json */
pub type JsonValue = serde_json::Value;

/* serenity-rs */
pub type SerenityClient = serenity::Client;
pub type SerenityContext = serenity::Context;
pub type SerenityReady = serenity::Ready;

/* poise-rs */
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, Error>;
pub type FrameworkError<'a> = poise::FrameworkError<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;
pub type CommandParameter = poise::CommandParameter<Data, Error>;

/* sqlx-rs */
pub type SqlxThrowable<T> = Result<T, sqlx::Error>;
