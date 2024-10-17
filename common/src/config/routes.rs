pub enum Routes {
    Query(QueryRoutes),
    Setup(SetupRoutes),
}

pub enum QueryRoutes {
    GetConfig,
    GetPoints,
}

pub enum SetupRoutes {
    DeployPaintTheMoon,
    DeployPaintHarvest,
    InitialMoonSetup,
}

impl Routes {
    pub fn as_str(&self) -> String {
        match self {
            Routes::Query(route) => format!("/query{}", route.as_str()),
            Routes::Setup(route) => format!("/setup{}", route.as_str()),
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

impl SetupRoutes {
    pub fn as_str(&self) -> &str {
        match self {
            SetupRoutes::DeployPaintTheMoon => "/deploy_paint_the_moon",
            SetupRoutes::DeployPaintHarvest => "/deploy_paint_harvest",
            SetupRoutes::InitialMoonSetup => "/initial_moon_setup",
        }
    }
}
