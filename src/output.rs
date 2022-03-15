/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::io::{self, Write};

pub trait SuperConsoleOutput: Send + Sync + 'static {
    /// Called to produce output. This may be called without pre_render if we are finalizing. This
    /// should flush if possible.
    fn output(&mut self, buffer: Vec<u8>) -> anyhow::Result<()>;
}

pub(crate) struct StderrSuperConsoleOutput;

impl SuperConsoleOutput for StderrSuperConsoleOutput {
    fn output(&mut self, buffer: Vec<u8>) -> anyhow::Result<()> {
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        handle.write_all(&buffer)?;
        handle.flush()?;
        Ok(())
    }
}