pub trait Animal {
    fn animal_id(&self) -> i32;
    fn talk(&self) -> &'static str;
    fn sum(&self) -> i32;
    fn rem(&self) -> i32;
    fn box_clone(&self) -> Box<dyn Animal>;
}
impl Clone for Box<dyn Animal> {
    fn clone(&self) -> Box<dyn Animal> {
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct Cat(i32);
#[derive(Clone)]
pub struct Dog(i32, i32);
#[derive(Clone)]
pub struct Duck(i32, i32, i32);
#[derive(Clone)]
pub struct Crow(i32, i32, i32, i32);
#[derive(Clone)]
pub struct Frog(i32, i32, i32, i32, i32);

impl Cat {
    pub fn new(a: i32) -> Self {
        Self(a)
    }
}
impl Dog {
    pub fn new(a: i32, b: i32) -> Self {
        Self(a, b)
    }
}
impl Duck {
    pub fn new(a: i32, b: i32, c: i32) -> Self {
        Self(a, b, c)
    }
}
impl Crow {
    pub fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        Self(a, b, c, d)
    }
}
impl Frog {
    pub fn new(a: i32, b: i32, c: i32, d: i32, e: i32) -> Self {
        Self(a, b, c, d, e)
    }
}

impl Animal for Cat {
    fn animal_id(&self) -> i32 {
        1
    }
    fn talk(&self) -> &'static str {
        "Meow!"
    }
    fn sum(&self) -> i32 {
        self.0
    }
    fn rem(&self) -> i32 {
        self.0
    }
    fn box_clone(&self) -> Box<dyn Animal> {
        Box::new(self.clone())
    }
}
impl Animal for Dog {
    fn animal_id(&self) -> i32 {
        2
    }
    fn talk(&self) -> &'static str {
        "Woof!"
    }
    fn sum(&self) -> i32 {
        self.0 + self.1
    }
    fn rem(&self) -> i32 {
        self.1 % self.0
    }
    fn box_clone(&self) -> Box<dyn Animal> {
        Box::new(self.clone())
    }
}
impl Animal for Duck {
    fn animal_id(&self) -> i32 {
        3
    }
    fn talk(&self) -> &'static str {
        "Quack!"
    }
    fn sum(&self) -> i32 {
        self.0 + self.1 + self.2
    }
    fn rem(&self) -> i32 {
        self.2 % self.1
    }
    fn box_clone(&self) -> Box<dyn Animal> {
        Box::new(self.clone())
    }
}
impl Animal for Crow {
    fn animal_id(&self) -> i32 {
        4
    }
    fn talk(&self) -> &'static str {
        "Caw!"
    }
    fn sum(&self) -> i32 {
        self.0 + self.1 + self.2 + self.3
    }
    fn rem(&self) -> i32 {
        self.3 % self.2
    }
    fn box_clone(&self) -> Box<dyn Animal> {
        Box::new(self.clone())
    }
}
impl Animal for Frog {
    fn animal_id(&self) -> i32 {
        5
    }
    fn talk(&self) -> &'static str {
        "Croak!"
    }
    fn sum(&self) -> i32 {
        self.0 + self.1 + self.2 + self.3 + self.4
    }
    fn rem(&self) -> i32 {
        self.4 % self.3
    }
    fn box_clone(&self) -> Box<dyn Animal> {
        Box::new(self.clone())
    }
}
