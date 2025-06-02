use crate::handler::app;
use actix_web::web;
use mongodb::bson::oid::ObjectId;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/apps")
            // 创建应用 - 需要认证
            .route("", web::post().to(app::create_app))
            // 获取应用列表 - 需要认证
            .route("", web::get().to(app::list_apps))
            // 获取单个应用 - 需要认证
            .route("/{app_id}", web::get().to(app::get_app))
            // 更新应用 - 需要认证
            .route("/{app_id}", web::put().to(app::update_app))
            // 删除应用 - 需要认证
            .route("/{app_id}", web::delete().to(app::delete_app)),
    );
}
