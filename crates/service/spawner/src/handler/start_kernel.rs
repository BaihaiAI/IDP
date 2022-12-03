use axum::extract::Json;
use common_model::Rsp;
use err::ErrorTrace;
use kernel_common::Header;

#[allow(clippy::unused_async)]
pub async fn start_kernel(Json(header): Json<Header>) -> Result<Rsp<()>, ErrorTrace> {
    if let Err(err) = kernel_common::spawn_kernel_process::spawn_kernel_process(header) {
        tracing::error!("{err:#?}");
        return Ok(Rsp::success(()).code(500));
    }
    Ok(Rsp::success(()))
}
