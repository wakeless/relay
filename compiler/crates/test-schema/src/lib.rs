/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#![deny(warnings)]
#![deny(rust_2018_idioms)]
#![deny(clippy::all)]

use lazy_static::lazy_static;
use schema::{build_schema_with_extensions, Schema, RELAY_EXTENSIONS};
use std::sync::Arc;

const TEST_SCHEMA_DATA: &str = include_str!("testschema.graphql");

lazy_static! {
    pub static ref TEST_SCHEMA: Arc<Schema> = Arc::new(
        build_schema_with_extensions(&[TEST_SCHEMA_DATA], &[RELAY_EXTENSIONS])
            .expect("Expected test schema to be valid")
    );
}

pub fn get_test_schema() -> Arc<Schema> {
    Arc::clone(&TEST_SCHEMA)
}

pub fn get_test_schema_with_extensions(extensions_sdl: &str) -> Arc<Schema> {
    Arc::new(
        build_schema_with_extensions(&[TEST_SCHEMA_DATA], &[extensions_sdl, RELAY_EXTENSIONS])
            .expect("Expected test schema (and extensions) to be valid"),
    )
}
