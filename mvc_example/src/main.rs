
extern crate sapper;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate typemap;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use sapper::{SApp, SAppWrapper, Request, Response, Result, SModule};
use typemap::Key;



mod blog;
use blog::BlogModule;

#[derive(Clone)]
struct MyApp;
// must impl it
// total entry and exitice
impl SAppWrapper for MyApp {
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("{}", "in SAppWrapper before.");
        
        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        println!("{}", "in SAppWrapper after.");
        
        Ok(())
    }
}

pub struct A_INT;
impl Key for A_INT { type Value = Arc<Box<usize>>; }
pub struct A_HashMap;
impl Key for A_HashMap { type Value = HashMap<&'static str, &'static str>; }
pub struct A_Mutex;
impl Key for A_Mutex { type Value = Arc<Mutex<HashMap<&'static str, &'static str>>>; }


pub fn main() {
    env_logger::init().unwrap();
    
    
    
    let mut sapp = SApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        .init_global(Box::new(move |req: &mut Request| -> Result<()> {
            println!("in init_global {:?}", req.query_string());
            req.ext_mut().insert::<A_INT>(a_global.clone());
            req.ext_mut().insert::<A_HashMap>(a_hash.clone());
            req.ext_mut().insert::<A_Mutex>(a_mutex.clone());
            
            Ok(())
        }))
        .with_wrapper(Box::new(MyApp))
        .add_module(Box::new(Biz))
        .add_module(Box::new(Foo));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run();
    
}
