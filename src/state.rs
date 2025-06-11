use mongodb::Database;

pub struct AppState {
    pub mongo_db: Database,
    pub jwt_secret: String,
}
