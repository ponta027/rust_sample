use crate::openapi::parameter::Parameters;
use yaml_rust::Yaml;
#[derive(Debug, Clone)]
///
pub struct Method {
    //
    method_name: Option<MethodEnum>,
    parameters: Parameters,
}
#[derive(Debug, Clone)]
pub enum MethodEnum {
    Get,
    Post,
    Put,
    Delete,
}

impl Method {
    pub fn new(method_name: &str, yaml: &Yaml) -> Self {
        let method_name_enum = Self::get_method_name(method_name);
        let parameters = Parameters::new(yaml);
        let method = Method {
            method_name: method_name_enum,
            parameters: parameters,
        };

        method
    }
    fn get_method_name(method_name: &str) -> Option<MethodEnum> {
        match method_name {
            "get" => Some(MethodEnum::Get),
            "put" => Some(MethodEnum::Post),
            "post" => Some(MethodEnum::Post),
            _ => None,
        }
    }
}
