use daoyi_cloud_models::models::common_result::CommonResult;
use salvo::http::{ResBody, StatusCode};
use salvo::prelude::Json;
use salvo::{FlowCtrl, Response, handler};

#[handler]
pub async fn error_handler(&self, res: &mut Response, ctrl: &mut FlowCtrl) {
    let err_code = if let Some(code) = res.status_code {
        code.as_str().parse::<i32>().unwrap()
    } else {
        500
    };
    let err_info = if let ResBody::Error(e) = &res.body {
        e.brief.clone()
    } else {
        StatusCode::try_from(err_code.to_string().parse::<u16>().unwrap_or(500))
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            .to_string()
    };
    res.status_code(StatusCode::OK);
    res.render(Json(CommonResult::<String>::err(err_code, err_info)));
    ctrl.skip_rest();
}
