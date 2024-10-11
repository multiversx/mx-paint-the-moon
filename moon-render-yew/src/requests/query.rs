use common::{get_request, Config, Points, QueryRoutes, Routes};

pub async fn get_all_points() -> Result<Points, String> {
    let config = Config::new();
    let dest = format!(
        "{}{}",
        config.microservice_url(),
        &Routes::Query(QueryRoutes::GetPoints).as_str()
    );
    let response = get_request::<Points>(&dest).await;

    match response {
        Ok(points) => Ok(points),
        Err(err) => Err(format!("Error fetching points: {err:?}")),
    }
}

pub async fn get_config() -> Result<Config, String> {
    let config = Config::new();
    let dest = format!(
        "{}{}",
        config.microservice_url(),
        &Routes::Query(QueryRoutes::GetConfig).as_str()
    );
    let response = get_request::<Config>(&dest).await;

    match response {
        Ok(config) => Ok(config),
        Err(err) => Err(format!("Error fetching config: {err:?}")),
    }
}
