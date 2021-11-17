use std::{any::{Any, TypeId}, collections::HashMap, ops::{Deref, DerefMut}, sync::{RwLock, RwLockReadGuard, RwLockWriteGuard}};

pub mod texture;
pub mod sprite;
pub mod shader;
#[derive(Default)]
pub struct Resources{
    rw_lock: RwLock<()>,
    resource_names: HashMap<String,ResourceKey>,
    resources: HashMap<TypeId,Vec<Box<dyn Any>>>,
}

unsafe impl Sync for Resources{}
unsafe impl Send for Resources{}
impl Resources {
    pub fn new() -> Self{
        Self{
            rw_lock: RwLock::new(()),
            resource_names: HashMap::new(),
            resources: HashMap::new(),
        }
    }
    pub fn get_resource_key(&self, key: &str) -> ResourceKey{

        let rn = self.rw_lock.read().unwrap();

        match self.resource_names.get(key){
            Some(x) => x.clone(),
            None => panic!("Invalid resource name {}",key)
        }
    }
    pub fn get_resource<'a, 'b: 'a, T: Sized + 'static>(&'b self, key: &ResourceKey) -> &'a T{
        let _lock = self.rw_lock.read().unwrap();
        let reference = self.resources.get(&key.class).expect(&format!("could not find group of type {:?}",key.class))[key.id].downcast_ref().unwrap();
        reference
    }

    pub fn add_resource<T: Sized + 'static>(&mut self, value: T, name: String) -> ResourceKey{
        let _lock = self.rw_lock.write().unwrap();

        let class = TypeId::of::<T>();
        let l = self.resources.get_mut(&class).expect(&format!("could not find group of type {:?}",class));
        l.push(Box::new(value));

        let key = ResourceKey{
            class,
            id: l.len()-1
        };

         self.resource_names.insert(name, key.clone());

         key
    }

    pub fn register_type<T: ?Sized + 'static>(&mut self) -> bool{
        let class = TypeId::of::<T>();
        let _lock = self.rw_lock.write().unwrap();
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
