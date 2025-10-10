use std::collections::HashMap;

use axum::{Router, routing::MethodRouter};
use hyper::Method;
use infrastructurex::web_info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum WebPathMethod {
    #[default]
    Get,
    Post,
}

impl std::fmt::Display for WebPathMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl WebPathMethod {
    pub fn to_method(&self) -> Method {
        match self {
            Self::Get => Method::GET,
            Self::Post => Method::POST,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
        }
    }
}
impl From<String> for WebPathMethod {
    fn from(value: String) -> Self {
        match value.as_str() {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => Self::Get,
        }
    }
}

impl From<&str> for WebPathMethod {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
impl From<Method> for WebPathMethod {
    fn from(value: Method) -> Self {
        match value {
            Method::GET => Self::Get,
            Method::POST => Self::Post,
            _ => Self::Get,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RouterGroup {
    pub final_path: String,
    pub method: WebPathMethod,
    #[serde(skip)]
    pub method_router: Option<MethodRouter>,
    sub_paths: HashMap<String, RouterGroup>,
    pub api_name: Option<String>,
}

impl RouterGroup {
    pub fn new() -> Self {
        RouterGroup::default()
    }
    pub fn nest(mut self, path: &str, web_path: RouterGroup) -> Self {
        self.sub_paths.insert(path.to_string(), web_path);
        self
    }

    pub fn merge(mut self, web_path: RouterGroup) -> Self {
        for (sub_key, sub_path) in web_path.sub_paths {
            self.sub_paths.insert(sub_key, sub_path);
        }
        self
    }

    pub fn route(
        mut self,
        path: &str,
        method: WebPathMethod,
        api_name: Option<&str>,
        method_router: MethodRouter,
    ) -> Self {
        self.sub_paths.insert(
            String::from(path),
            RouterGroup {
                method: method,
                api_name: api_name.map(|s| s.to_string()),
                method_router: Some(method_router),
                sub_paths: HashMap::new(),
                ..Default::default()
            },
        );
        self
    }

    fn concat_sub_paths_final_paths(&mut self, parent_path: &str) {
        for (sub_key, sub_path) in self.sub_paths.iter_mut() {
            let f_path = format!("{}{}", parent_path, sub_key);
            sub_path.concat_sub_paths_final_paths(&f_path);
            sub_path.final_path = f_path;
        }
    }

    fn is_last_level(&self) -> bool {
        self.sub_paths.is_empty()
    }

    fn final_to_path(mut self) -> Self {
        self.concat_sub_paths_final_paths("");
        self
    }

    fn get_last_level_paths(&self) -> Vec<&RouterGroup> {
        let mut last_level_paths = Vec::new();
        if self.is_last_level() {
            last_level_paths.push(self);
        }
        for sub_path in self.sub_paths.values() {
            last_level_paths.extend(sub_path.get_last_level_paths());
        }
        last_level_paths
    }

    // pub fn print_all_paths(&self) {
    //     for sub_path_data in self.sub_paths.values() {
    //         if sub_path_data.is_last_level() {
    //             web_info!("{}", sub_path_data.final_path);
    //         }
    //         sub_path_data.print_all_paths();
    //     }
    // }
}

impl Into<Router> for RouterGroup {
    fn into(self) -> Router {
        let router_group: RouterGroup = self.final_to_path();
        let expand_path = router_group.get_last_level_paths();

        let mut router = Router::new();
        for p in expand_path {
            if let Some(method_router) = p.method_router.clone() {
                router = router.route(&p.final_path, method_router);
                web_info!("[路由]:[{}]{}", p.method, p.final_path);
            }
        }
        router
    }
}
