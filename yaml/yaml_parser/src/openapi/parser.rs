use crate::openapi::method::Method;
use std::fs;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

#[derive(Debug, Clone)]
pub struct OpenApi {
    info: Option<OpenApiInfo>,
    paths: Option<Vec<OpenApiPath>>,
}
#[derive(Debug, Clone)]
struct OpenApiInfo {
    version: Option<String>,
    title: Option<String>,
}
#[derive(Debug, Clone)]
struct OpenApiPath {
    path: String,
    method: Vec<Method>,
    //
}

impl OpenApi {
    pub fn new(file_name: String) -> Self {
        let mut api = OpenApi {
            info: None,
            paths: None,
        };

        let yaml = api._parse(file_name);
        let info = api._parse_info(&yaml);
        api.info = Some(info);
        let path = api._parse_path(&yaml);
        api.paths = Some(path);
        api
    }
    fn _load(&self, file_name: String) -> Yaml {
        let s = fs::read_to_string(file_name).unwrap().to_string();
        let docs = YamlLoader::load_from_str(&s).unwrap();
        let doc = &docs[0];
        doc.clone()
    }
    fn _parse_path(&self, yaml: &Yaml) -> Vec<OpenApiPath> {
        let paths: Vec<_> = yaml
            .as_hash()
            .unwrap()
            .into_iter()
            .filter(|s| s.0.as_str() == Some("paths"))
            .collect();
        let path_hash = paths[0].1;

        let methods: Vec<OpenApiPath> = path_hash
            .as_hash()
            .unwrap()
            .into_iter()
            .map(|s| self._create_path(s.0.as_str().unwrap().to_string(), s.1))
            .collect();
        methods
    }

    fn _create_path(&self, path: String, yaml: &Yaml) -> OpenApiPath {
        let methods: Vec<_> = yaml
            .as_hash()
            .unwrap()
            .into_iter()
            .map(|s| Method::new(s.0.as_str().unwrap(), s.1))
            .collect();

        OpenApiPath {
            path: path,
            method: methods,
        }
    }

    fn _parse_info(&self, yaml: &Yaml) -> OpenApiInfo {
        //
        //
        let info: Vec<_> = yaml
            .as_hash()
            .unwrap()
            .into_iter()
            .filter(|s| s.0.as_str() == Some("info"))
            .collect();
        let info_hash = info[0].1;
        //
        OpenApiInfo {
            version: Some(info_hash["version"].as_str().unwrap().to_string()),
            title: Some(info_hash["title"].as_str().unwrap().to_string()),
        }
    }

    fn _parse(&self, file_name: String) -> Yaml {
        let yaml = self._load(file_name);
        self._parse_info(&yaml);
        yaml
    }
}
