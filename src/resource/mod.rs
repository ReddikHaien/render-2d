use std::{any::{Any, TypeId}, collections::HashMap};

pub mod texture;
pub mod sprite;
pub mod shader;
pub struct Resources{
    resource_names: HashMap<String,ResourceKey>,
    resources: HashMap<TypeId,Vec<Box<dyn Any>>>,
}

impl Resources {
    pub fn new() -> Self{
        Self{
            resource_names: HashMap::new(),
            resources: HashMap::new(),
        }
    }
    pub fn get_resource_key(&self, key: &str) -> ResourceKey{
        match self.resource_names.get(key){
            Some(x) => x.clone(),
            None => panic!("Invalid resource name {}",key)
        }
    }
    pub fn get_resource<'a, 'b: 'a, T: Sized + 'static>(&'a self, key: &ResourceKey) -> &'a T{
        self.resources.get(&key.class).expect(&format!("could not find group of type {:?}",key.class))[key.id].downcast_ref().unwrap()
    }

    pub fn add_resource<T: Sized + 'static>(&mut self, value: T, name: String) -> ResourceKey{
        let class = TypeId::of::<T>();
        let l = self.resources.get_mut(&class).expect(&format!("could not find group of type {:?}",class));
        l.push(Box::new(value));

        ResourceKey{
            class,
            id: l.len()-1
        }
    }

    pub fn register_type<T: ?Sized + 'static>(&mut self) -> bool{
        let class = TypeId::of::<T>();
        if self.resources.contains_key(&class){
            false
        }
        else{
            self.resources.insert(class, Vec::new());
            true
        }
    }
}


#[derive(Clone,Copy)]
pub struct ResourceKey{
    class: TypeId,
    id: usize,
}