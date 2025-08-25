use rand::seq::IndexedRandom;
use serde_json::json;

pub fn build_server_status(players: i64) -> Result<serde_json::Value, anyhow::Error> {
    let config = qexed_config::get_global_config()?;
    let mut rand_item = &config.game.motd[0];
    if let Some(random_item) = config.game.motd.choose(&mut rand::rng()) {
        rand_item = random_item;
    }
    let v = json!({
        "version": {
            "name": "1.21.8",
            "protocol": 772
        },
        "players": {
            "max": config.game.max_player,
            "online": players,
            "sample": [
            ]
        },
        "description": {
            "text": rand_item,
        },
        "favicon": &config.game.favicon,
        "enforcesSecureChat": false
    });
    Ok(v)
}