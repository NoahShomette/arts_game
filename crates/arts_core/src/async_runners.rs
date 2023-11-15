use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
pub fn run_async<F>(future: F, runtime: &bevy::tasks::TaskPool) -> Option<bevy::tasks::Task<()>>
where
    F: Future<Output = ()> + Send + 'static,
{
    Some(runtime.spawn(future))
}

#[cfg(target_arch = "wasm32")]
pub fn run_async<F>(future: F, runtime: &bevy::tasks::TaskPool) -> Option<bevy::tasks::Task<()>>
where
    F: Future<Output = ()> + 'static,
{
    runtime.spawn_local(future);
    None
}
