use yaml_rust::Yaml;
#[derive(Debug, Clone)]
///
pub struct Parameters {
    //
    parameters: Vec<Parameter>,
}

#[derive(Debug, Clone)]
struct Parameter {
    name: String,
    in_type: Option<ParameterInType>,
}
#[derive(Debug, Clone)]
///
enum ParameterInType {
    //
    Query,
    Path,
}
impl Parameters {
    pub fn new(yaml: &Yaml) -> Self {
        //
        let parameters = Self::get_param(yaml);
        Parameters {
            parameters: parameters,
        }
    }
    fn get_param(yaml: &Yaml) -> Vec<Parameter> {
        let param: Vec<_> = yaml
            .as_hash()
            .unwrap()
            .into_iter()
            .filter(|s| s.0.as_str() == Some("parameters"))
            .flat_map(|s| s.1.as_vec().unwrap().into_iter())
            .map(|s| Self::ceate_parameter(s))
            .collect();
        param
    }
    fn ceate_parameter(yaml: &Yaml) -> Parameter {
        Parameter {
            name: yaml["name"].as_str().unwrap().to_string(),
            in_type: Self::convert_in_type(yaml["in"].as_str().unwrap()),
        }
    }
    fn convert_in_type(in_type: &str) -> Option<ParameterInType> {
        match in_type {
            "path" => Some(ParameterInType::Path),
            "query" => Some(ParameterInType::Query),
            _ => None,
        }
    }
}
