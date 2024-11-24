pub mod lottery;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(lottery::create);
}