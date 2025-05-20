use daoyi_cloud_models::models::common_result::CommonResult;
use salvo::prelude::*;

#[endpoint]
pub async fn hello(req: &mut Request) -> CommonResult<String> {
    CommonResult::ok(Some(format!("Hello, {}!", req.method())))
}
