use crate::error::E;
use openapiv3::*;
//use anyhow::{Result};
use hyper::Method;
use serde_yaml;
use std::path::Path;

pub fn read<P: AsRef<Path>>(filename: P) -> OpenAPI {
    let data = std::fs::read_to_string(filename).expect("OpenAPI file could not be read.");
    let spec =
        serde_yaml::from_str(&data).expect("Could not deserialize file as OpenAPI v3.0 yaml");
    debug!("The openapi after parsed {:?}", spec);
    spec
}

pub fn path_to_operation<'a>(
    item: &'a mut PathItem,
    method: &Method,
) -> Result<&'a mut Operation, E> {
    let inner =
        |op: &'a mut Option<Operation>| op.as_mut().ok_or(E::MethodError(format!("{:?}", method)));
    match *method {
        Method::DELETE => inner(&mut item.delete),
        Method::GET => inner(&mut item.get),
        Method::HEAD => inner(&mut item.head),
        Method::OPTIONS => inner(&mut item.options),
        Method::PATCH => inner(&mut item.patch),
        Method::POST => inner(&mut item.post),
        Method::PUT => inner(&mut item.put),
        _ => unimplemented!("Method not supported"),
    }
}

pub fn operation_list(item: &PathItem) -> Vec<&Operation> {
    let mut result = Vec::new();
    result.push(&item.delete);
    result.push(&item.get);
    result.push(&item.head);
    result.push(&item.options);
    result.push(&item.patch);
    result.push(&item.post);
    result.push(&item.put);
    result.iter().filter(|x| x.is_some()).map(|a| a.as_ref().unwrap()).collect()
}

pub fn parameter_location(parameter: &Parameter) -> String {
    match parameter {
        Parameter::Query { .. } => "query".to_string(),
        Parameter::Header { .. } => "header".to_string(),
        Parameter::Path { .. } => "path".to_string(),
        Parameter::Cookie { .. } => "cookie".to_string(),
    }
}

pub fn parameter_to_parameter_data(parameter: &Parameter) -> &ParameterData {
    match parameter {
        Parameter::Query { parameter_data, .. } => parameter_data,
        Parameter::Header { parameter_data, .. } => parameter_data,
        Parameter::Path { parameter_data, .. } => parameter_data,
        Parameter::Cookie { parameter_data, .. } => parameter_data,
    }
}

pub fn parameter_to_parameter_data_mut(parameter: &mut Parameter) -> &mut ParameterData {
    match parameter {
        Parameter::Query { parameter_data, .. } => parameter_data,
        Parameter::Header { parameter_data, .. } => parameter_data,
        Parameter::Path { parameter_data, .. } => parameter_data,
        Parameter::Cookie { parameter_data, .. } => parameter_data,
    }
}

pub fn used(description: &mut Option<String>) {
    *description = Some("1".to_string());
}
