// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.

use std::cmp::min;

pub fn create_basic_runtime(num_threads: usize) -> tokio::runtime::Runtime {
  tokio::runtime::Builder::new()
    .basic_scheduler()
    .enable_io()
    .enable_time()
    // This limits the number of threads for blocking operations (like for
    // synchronous fs ops) or CPU bound tasks like when we run dprint in
    // parallel for deno fmt.
    // The default value is 512, which is an unhelpfully large thread pool. We
    // don't ever want to have more than a couple dozen threads.
    .max_threads(num_threads)
    .build()
    .unwrap()
}

// TODO(ry) rename to run_local ?
pub fn run_basic<F, R>(future: F) -> R
where
  F: std::future::Future<Output = R>,
{
  let mut rt = create_basic_runtime(32);
  rt.block_on(future)
}

pub fn run_basic_custom_pool<F, R>(future: F, num_threads: usize) -> R
where
  F: std::future::Future<Output = R>,
{
  let mut rt = create_basic_runtime(num_threads);
  rt.block_on(future)
}
