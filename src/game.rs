pub mod game {
    use db::redis::RedisConnection;
    use redis::Commands;
    use rocket_contrib::Json;
    use serde_json;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    #[derive(Serialize, Deserialize)]
    struct HypeGame {
        game_info: Game,
        hype_map: HashMap<String, u16>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GamesList {
        games: Vec<HypeGame>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    struct Game {
        name: String,
        id: usize,
        site_detail_url: String,
        api_detail_url: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GamesListSansHype {
        games: Vec<Game>,
    }

    pub fn get_games_vec(conn: &RedisConnection) -> Json<GamesList> {
        let json_string: String = conn.get("games").expect("error retrieving games");
        let games_list_sans_hype: GamesListSansHype = serde_json::from_str(&json_string).unwrap();
        let games_list = GamesList {
            games: games_list_sans_hype
                .games
                .iter()
                .map(|vg| {
                    let hype_iter: Vec<(String, u16)> =
                        conn.hgetall(vg.id).expect("error loading hashmap");
                    HypeGame {
                        hype_map: HashMap::from_iter(hype_iter),
                        game_info: vg.clone(),
                    }
                })
                .collect(),
        };
        Json(games_list)
    }

}
