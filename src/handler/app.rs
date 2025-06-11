use crate::{model::app::{App, CreateAppRequest, UpdateAppRequest}, state::AppState, middleware::auth::Claims};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use mongodb::bson::{doc, oid::ObjectId, DateTime};

#[derive(Debug, Deserialize)]
pub struct ListAppsQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

pub async fn create_app(
    state: web::Data<AppState>,
    claims: Claims,
    payload: web::Json<CreateAppRequest>,
) -> impl Responder {
    let user_id = match ObjectId::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(e) => return HttpResponse::Unauthorized().json(e.to_string()),
    };
    let app = App {
        id: None,
        name: payload.name.clone(),
        description: payload.description.clone(),
        owner_id: user_id,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    let collection = state.mongo_db.collection::<App>("apps");
    match collection.insert_one(app, None).await {
        Ok(result) => {
            match collection.find_one(doc! {"_id": result.inserted_id}, None).await {
                Ok(Some(app)) => HttpResponse::Ok().json(app),
                Ok(None) => HttpResponse::NotFound().json("App not found"),
                Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn list_apps(
    state: web::Data<AppState>,
    claims: Claims,
    query: web::Query<ListAppsQuery>,
) -> impl Responder {
    let user_id = match ObjectId::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(e) => return HttpResponse::Unauthorized().json(e.to_string()),
    };
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let collection = state.mongo_db.collection::<App>("apps");
    let filter = doc! {"owner_id": user_id};
    let options = mongodb::options::FindOptions::builder()
        .skip((page - 1) * page_size)
        .limit(page_size as i64)
        .build();

    match collection.find(filter, Some(options)).await {
        Ok(cursor) => {
            let mut apps = Vec::new();
            let mut cursor = cursor;
            while let Ok(true) = cursor.advance().await {
                if let Ok(app) = cursor.deserialize_current() {
                    apps.push(app);
                }
            }
            HttpResponse::Ok().json(apps)
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_app(
    state: web::Data<AppState>,
    claims: Claims,
    app_id: web::Path<ObjectId>,
) -> impl Responder {
    let user_id = match ObjectId::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(e) => return HttpResponse::Unauthorized().json(e.to_string()),
    };
    let collection = state.mongo_db.collection::<App>("apps");
    match collection.find_one(doc! {"_id": app_id.into_inner(), "owner_id": user_id}, None).await {
        Ok(Some(app)) => HttpResponse::Ok().json(app),
        Ok(None) => HttpResponse::NotFound().json("App not found or not owned by user"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn update_app(
    state: web::Data<AppState>,
    claims: Claims,
    app_id: web::Path<ObjectId>,
    payload: web::Json<UpdateAppRequest>,
) -> impl Responder {
    let user_id = match ObjectId::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(e) => return HttpResponse::Unauthorized().json(e.to_string()),
    };
    let collection = state.mongo_db.collection::<App>("apps");
    let filter = doc! {"_id": app_id.into_inner(), "owner_id": user_id};
    
    let mut update = doc! {"$set": {"updated_at": DateTime::now()}};
    if let Some(name) = &payload.name {
        update.get_document_mut("$set").unwrap().insert("name", name);
    }
    if let Some(description) = &payload.description {
        update.get_document_mut("$set").unwrap().insert("description", description);
    }

    let options = mongodb::options::FindOneAndUpdateOptions::builder()
        .return_document(mongodb::options::ReturnDocument::After)
        .build();

    match collection.find_one_and_update(filter, update, Some(options)).await {
        Ok(Some(app)) => HttpResponse::Ok().json(app),
        Ok(None) => HttpResponse::NotFound().json("App not found or not owned by user"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn delete_app(
    state: web::Data<AppState>,
    claims: Claims,
    app_id: web::Path<ObjectId>,
) -> impl Responder {
    let user_id = match ObjectId::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(e) => return HttpResponse::Unauthorized().json(e.to_string()),
    };
    let collection = state.mongo_db.collection::<App>("apps");
    let filter = doc! {"_id": app_id.into_inner(), "owner_id": user_id};
    match collection.delete_one(filter, None).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                HttpResponse::Ok().json(())
            } else {
                HttpResponse::NotFound().json("App not found or not owned by user")
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
