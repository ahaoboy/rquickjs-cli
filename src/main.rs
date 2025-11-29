use rquickjs_core::{Context, Function, Result, Runtime, context::EvalOptions};

fn print(s: String) {
    println!("{s}");
}

#[cfg(all(
    not(target_os = "windows"),
    not(target_os = "android"),
    not(target_env = "musl")
))]
#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn main() -> Result<()> {
    let Some(path) = std::env::args().nth(1) else {
        println!("rquickjs <PATH>");
        return Ok(());
    };
    let rt = Runtime::new()?;
    rt.set_memory_limit(u32::MAX as usize);
    rt.set_max_stack_size(u32::MAX as usize);
    let ctx = Context::full(&rt)?;
    ctx.with(|ctx| -> Result<()> {
        let global = ctx.globals();
        global.set(
            "__print",
            Function::new(ctx.clone(), print)?.with_name("__print")?,
        )?;
        ctx.eval::<(), _>(
            r#"
globalThis.console = {
  log(...v) {
    globalThis.__print(`${v.join(" ")}`)
  }
}
"#,
        )?;

        let opt = EvalOptions::default();
        ctx.eval_file_with_options(path, opt)?
    })?;

    Ok(())
}
