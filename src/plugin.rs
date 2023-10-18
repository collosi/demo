use anyhow::{Context, Result};
use std::path::Path;
use wasmtime::*;

#[derive(Default)]
pub struct StoreState {}

pub struct DemoRunner {
    instance: wasmtime::Instance,
    store: wasmtime::Store<StoreState>,
}

pub fn create_file<P>(path: P) -> anyhow::Result<DemoRunner>
where
    P: AsRef<Path>,
{
    // First the wasm module needs to be compiled. This is done with a global
    // "compilation environment" within an `Engine`. Note that engines can be
    // further configured through `Config` if desired instead of using the
    // default like this is here.
    // println!("Compiling module...");
    let engine = Engine::default();
    let module = Module::from_file(&engine, path)?;

    // After a module is compiled we create a `Store` which will contain
    // instantiated modules and other items like host functions. A Store
    // contains an arbitrary piece of host information, and we use `MyState`
    // here.
    // println!("Initializing...");
    let mut store = Store::new(&engine, Default::default());

    // // Our wasm module we'll be instantiating requires one imported function.
    // // the function takes no parameters and returns no results. We create a host
    // // implementation of that function here, and the `caller` parameter here is
    // // used to get access to our original `MyState` value.
    // println!("Creating callback...");
    // let hello_func = Func::wrap(&mut store, |mut caller: Caller<'_, MyState>| {
    //     println!("Calling back...");
    //     println!("> {}", caller.data().name);
    //     caller.data_mut().count += 1;
    // });

    // Once we've got that all set up we can then move to the instantiation
    // phase, pairing together a compiled module as well as a set of imports.
    // Note that this is where the wasm `start` function, if any, would run.
    // println!("Instantiating module...");
    // let imports = [hello_func.into()];
    let imports = [];
    let instance = Instance::new(&mut store, &module, &imports)?;
    Ok(DemoRunner { instance, store })
}

#[repr(C)]
#[derive(Clone)]
pub struct Rect {
    pub width: i32,
    pub height: i32,
}
impl DemoRunner {
    pub fn call_set_dimensions(
        &mut self,
        dpi: i32,
        min: &Rect,
        preferred: &Rect,
        max: &Rect,
    ) -> anyhow::Result<Rect> {
        if let Ok(set_dimensions) = self
            .instance
            .get_typed_func::<(i32, i32, i32, i32, i32, i32, i32), (i32, i32)>(
                &mut self.store,
                "set_dimensions",
            )
        {
            let (width, height) = set_dimensions.call(
                &mut self.store,
                (
                    dpi,
                    min.width,
                    min.height,
                    preferred.width,
                    preferred.height,
                    max.width,
                    max.height,
                ),
            )?;

            Ok(Rect { width, height })
        } else {
            Ok(preferred.clone())
        }
    }

    pub fn call_render(
        &mut self,
        time: i32,
        size: &Rect,
        handle_data: impl Fn(&[u8]) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        let run = self
            .instance
            .get_typed_func::<(i32, i32, i32), i32>(&mut self.store, "render")?;

        let ptr = run.call(&mut self.store, (time, size.width, size.height))? as usize;
        let len = size.width as usize * size.height as usize * 4;

        let memory = self
            .instance
            .get_memory(&mut self.store, "memory")
            .context("no memory")?;
        handle_data(&memory.data(&mut self.store)[ptr..(ptr + len)])
    }
}
