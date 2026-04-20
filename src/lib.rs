/***
 *
 * === TinyTrees ===
 *
 * TinyTrees is a Rust library for building and training decision trees.
 * It is designed with a minimalist approach: no-std, no-alloc, safe and lightweight.
 * The library is optimized for 32-bit microcontrollers with FPU support,
 * such as the ARM Cortex-M4F (e.g., STM32F303 series).
 *
 *
 * === Personal considerations ===
 *
 * TinyTrees aims to be a solid library, but it is ultimately still part
 * of the original author's learning journey in Rust, embedded development,
 * and machine learning.
 *
 *
 * === Reference material ===
 *
 * Concepts:
 * https://doc.rust-lang.org/rust-by-example
 * https://docs.rust-embedded.org/book/
 * Artificial Intelligence: A Modern Approach, Global Edition
 *
 * Documentation and source code:
 * https://github.com/scikit-learn/scikit-learn
 */

#![no_std]
#[warn(unused)]
// #[warn(missing_docs)]
#[allow(dead_code)] // Todo: remove this

mod tree;
mod arena;
mod errors;