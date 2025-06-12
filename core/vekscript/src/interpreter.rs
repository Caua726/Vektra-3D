use std::collections::HashMap;
use crate::grammar::*;
use tobj;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
    pub mesh: Option<Mesh>,
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub time: f64,
    pub object_name: String,
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Debug, Default, Clone)]
pub struct World {
    pub objects: HashMap<String, Object>,
    pub keyframes: Vec<Keyframe>,
}

pub struct Interpreter {
    world: World,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            world: World::default(),
        }
    }

    pub fn interpret(&mut self, program: &Program) -> Result<&World, String> {
        for statement in program {
            self.execute(statement)?;
        }
        Ok(&self.world)
    }

    fn execute(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Import(stmt) => self.visit_import(stmt),
            Statement::Object(stmt) => self.visit_object(stmt),
            Statement::Keyframe(stmt) => self.visit_keyframe(stmt),
            Statement::Interpolate(stmt) => self.visit_interpolate(stmt),
        }
    }

    fn visit_import(&mut self, stmt: &ImportStatement) -> Result<(), String> {
        println!("Loading model: {} as {}", stmt.path, stmt.alias);

        // Assumindo que os modelos estão na pasta 'examples' por enquanto
        let model_path = format!("examples/{}", stmt.path);
        
        let (models, _materials) = tobj::load_obj(
            &model_path,
            &tobj::LoadOptions::default()
        ).map_err(|e| format!("Failed to load OBJ file '{}': {}", model_path, e))?;

        if let Some(model) = models.into_iter().next() {
            let mesh = model.mesh;
            let loaded_mesh = Mesh {
                vertices: mesh.positions,
                indices: mesh.indices,
            };
            
            let object = Object {
                name: stmt.alias.clone(),
                mesh: Some(loaded_mesh),
                properties: HashMap::new(),
            };
            self.world.objects.insert(stmt.alias.clone(), object);
        } else {
            return Err(format!("No models found in file '{}'", stmt.path));
        }

        Ok(())
    }

    fn visit_object(&mut self, stmt: &ObjectDefinition) -> Result<(), String> {
        println!("Interpreting object definition: {}", stmt.name);
        
        if let Some(object) = self.world.objects.get_mut(&stmt.name) {
            for prop in &stmt.properties {
                object.properties.insert(prop.name.clone(), prop.value.clone());
            }
        } else {
            return Err(format!("Object '{}' not found. Was it imported?", stmt.name));
        }
        
        Ok(())
    }

    fn visit_keyframe(&mut self, stmt: &KeyframeDefinition) -> Result<(), String> {
        println!("Interpreting keyframe at time: {}", stmt.time);

        let mut properties = HashMap::new();
        for prop in &stmt.properties {
            properties.insert(prop.name.clone(), prop.value.clone());
        }

        let keyframe = Keyframe {
            time: stmt.time,
            object_name: stmt.object_name.clone(),
            properties,
        };
        self.world.keyframes.push(keyframe);
        
        Ok(())
    }

    fn visit_interpolate(&mut self, stmt: &InterpolateDefinition) -> Result<(), String> {
        println!("Interpreting interpolation from {} to {}", stmt.from, stmt.to);
        // A lógica de interpolação real seria complexa.
        // Ela calcularia os estados intermediários entre os keyframes.
        // Por enquanto, vamos apenas registrar a intenção.
        Ok(())
    }
}
