pub enum Routes {
    Query(QueryRoutes),
}

pub enum QueryRoutes {
    GetConfig,
    GetPoints,
}

impl Routes {
    pub fn as_str(&self) -> String {
        match self {
            Routes::Query(route) => format!("/query{}", route.as_str()),
        }
    }
}

impl QueryRoutes {
    pub fn as_str(&self) -> &str {
        match self {
            QueryRoutes::GetConfig => "/get_config",
            QueryRoutes::GetPoints => "/get_points",
        }
    }
}
