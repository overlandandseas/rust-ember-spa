use super::CORS;

use super::db::redis::RedisConnection;
use super::game::game::{get_games_vec, GamesList};
use r2d2;
use std::ops::Deref;

use r2d2_redis::RedisConnectionManager;
use rocket::Route;
use rocket_contrib::Json;

impl Deref for RedisConnection {
    type Target = r2d2::PooledConnection<RedisConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[get("/api/games")]
fn get_games(conn: RedisConnection) -> CORS<Json<GamesList>> {
    // let result: String = conn.get("games").expect("Error Grabbing Games");
    // let json_obj = Json(json!(result));
    let json_obj = get_games_vec(&conn);
    CORS::any(json_obj)
}

pub fn get_routes() -> Vec<Route> {
    routes![get_games]
}
