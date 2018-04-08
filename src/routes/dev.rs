use super::CORS;

use super::db::redis::RedisConnection;
use r2d2;
use redis::Commands;
use std::ops::Deref;

use r2d2_redis::RedisConnectionManager;
use rocket::Route;
use rocket_contrib::{Json, Value};

impl Deref for RedisConnection {
    type Target = r2d2::PooledConnection<RedisConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[get("/api/games")]
fn get_games(conn: RedisConnection) -> CORS<Json<Value>> {
    // CORS::any(Json(json!({
    // 	"results": {
    // 		"games": [{
    // 				"api_detail_url": "https://www.giantbomb.com/api/game/3030-46632/",
    // 				"id": 46632,
    // 				"name": "Anthem",
    // 				"site_detail_url": "https://www.giantbomb.com/anthem/3030-46632/"
    // 			},
    // 			{
    // 				"api_detail_url": "https://www.giantbomb.com/api/game/3030-42926/",
    // 				"id": 42926,
    // 				"name": "Kingdom Hearts III",
    // 				"site_detail_url": "https://www.giantbomb.com/kingdom-hearts-iii/3030-42926/"
    // 			},
    // 			{
    // 				"api_detail_url": "https://www.giantbomb.com/api/game/3030-67074/",
    // 				"id": 67074,
    // 				"name": "Farming Simulator 19",
    // 				"site_detail_url": "https://www.giantbomb.com/farming-simulator-19/3030-67074/"
    // 			},
    // 			{
    // 				"api_detail_url": "https://www.giantbomb.com/api/game/3030-66622/",
    // 				"id": 66622,
    // 				"name": "Battlefield 2018 (Working Title)",
    // 				"site_detail_url": "https://www.giantbomb.com/battlefield-2018-working-title/3030-66622/"
    // 			},
    // 			{
    // 				"api_detail_url": "https://www.giantbomb.com/api/game/3030-38456/",
    // 				"id": 38456,
    // 				"name": "Cyberpunk 2077",
    // 				"site_detail_url": "https://www.giantbomb.com/cyberpunk-2077/3030-38456/"
    // 			},
    // 			{
    // 				"api_detail_url": "https://www.giantbomb.com/api/game/3030-57216/",
    // 				"id": 57216,
    // 				"name": "The Last of Us Part II",
    // 				"site_detail_url": "https://www.giantbomb.com/the-last-of-us-part-ii/3030-57216/"
    // 			}
    // 		]
    // 	}
    // })))
    let result: String = conn.get("games").expect("Error Grabbing Games");
    let json_obj = Json(json!(result));
    CORS::any(json_obj)
}

pub fn get_routes() -> Vec<Route> {
    routes![get_games]
}
