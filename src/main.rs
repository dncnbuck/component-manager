use std::collections::HashMap;
use std::any::TypeId;
use core::fmt::Debug;
use std::any::type_name;

trait Component {    
    fn get_type_name(&self) -> &str;
}

impl Debug for dyn Component {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Component{}", self.get_type_name())
    }
}

#[derive(Debug)]
struct Foo{
    foo: u32
}

impl Component for Foo {
    fn get_type_name(&self) -> &str {
        type_name::<Foo>()
    }
}

#[derive(Debug)]
struct Bar{
    bar: u32
}
impl Component for Bar {
    fn get_type_name(&self) -> &str {
        type_name::<Bar>()
    }
}

#[derive(Debug)]
struct ComponentArray<T> {
    array: HashMap<usize, T>
}

impl <T> ComponentArray<T> {
    fn new() -> Self {
        ComponentArray { array: HashMap::new() }
    }

    fn insert_data(&mut self, component: T) {
        let new_index = self.array.len()+1;
        self.array.insert(new_index, component);
    }
}

trait _ComponentArray {
    fn get_type_name(&self) -> &str;
    
}

impl <T> _ComponentArray for ComponentArray<T> {
    fn get_type_name(&self) -> &str{
        type_name::<T>()
    }
    
}

impl Debug for dyn _ComponentArray {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ComponentArray<{}>",self.get_type_name())
    }
}


#[derive(Debug)]
struct ComponentManager {
    components: HashMap<TypeId, Box<dyn _ComponentArray>>,
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            components: HashMap::new()
        }
    }

    pub fn register<T:'static + Component>(&mut self) {
        let component_type = TypeId::of::<T>();
        let new_component_array = ComponentArray::<T>::new();
        self.components.insert(component_type, Box::new(new_component_array));

    }

    pub fn add_component<T: 'static + Component>(&mut self, component: T){
        let component_type = TypeId::of::<T>();
        
        match self.components.get_mut(&component_type) {
            Some(component_array) => {
                println!("{:?}", component_array);
                component_array.insert_data(component);
                ()
            },
            _ => (),

        }

    }
}


fn main() {
    let mut component_manager = ComponentManager::new();
    component_manager.register::<Foo>();
    component_manager.register::<Bar>();
    component_manager.add_component(Foo{foo:0});
}
