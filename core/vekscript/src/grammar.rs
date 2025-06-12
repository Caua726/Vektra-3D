#[derive(Debug, PartialEq, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyValue {
    Vector(Vector3),
    Identifier(String),
}

#[derive(Debug, PartialEq)]
pub struct PropertyAssignment {
    pub name: String,
    pub value: PropertyValue,
}

#[derive(Debug, PartialEq)]
pub struct ObjectDefinition {
    pub name: String,
    pub properties: Vec<PropertyAssignment>,
}

#[derive(Debug, PartialEq)]
pub struct KeyframeDefinition {
    pub time: f64,
    pub object_name: String,
    pub properties: Vec<PropertyAssignment>,
}

#[derive(Debug, PartialEq)]
pub struct InterpolateDefinition {
    pub from: f64,
    pub to: f64,
    pub properties: Vec<PropertyAssignment>,
}

#[derive(Debug, PartialEq)]
pub struct ImportStatement {
    pub path: String,
    pub alias: String,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Import(ImportStatement),
    Object(ObjectDefinition),
    Keyframe(KeyframeDefinition),
    Interpolate(InterpolateDefinition),
}

pub type Program = Vec<Statement>;
