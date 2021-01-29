#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

pub mod config;
pub mod db;
pub mod model;
pub mod controller;
pub mod service;
pub mod utils;
pub mod middleware;
pub mod router;
