use actix_web::{
    get,
    post,
    put,
    delete,
    HttpResponse,
    Responder,
};

#[get("/task/")]
pub async fn get_tasks() -> impl Responder {
    HttpResponse::Ok().body("get_tasks")
}

#[get("/task/{task_global_id}")]
pub async fn get_task() -> impl Responder {
    HttpResponse::Ok().body("get_task")
}

#[post("/task/")]
pub async fn submit_task() -> impl Responder {
    HttpResponse::Ok().body("submit_task")
}

#[put("/task/{task_global_id}")]
pub async fn update_task() -> impl Responder {
    HttpResponse::Ok().body("update task")
}

#[delete("/task/{task_global_id}")]
pub async fn delete_task() -> impl Responder {
    HttpResponse::Ok().body("delete task")
}
