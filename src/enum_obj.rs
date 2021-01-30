#[derive(Clone)]
pub enum Animal {
    Cat(i32),
    Dog(i32, i32),
    Duck(i32, i32, i32),
    Crow(i32, i32, i32, i32),
    Frog(i32, i32, i32, i32, i32),
}
impl Animal {
    pub fn animal_id(&self) -> i32 {
        match self {
            Animal::Cat(_a) => 1,
            Animal::Dog(_a, _b) => 2,
            Animal::Duck(_a, _b, _c) => 3,
            Animal::Crow(_a, _b, _c, _d) => 4,
            Animal::Frog(_a, _b, _c, _d, _e) => 5,
        }
    }
    pub fn talk(&self) -> &'static str {
        match self {
            Animal::Cat(_a) => "Meow!",
            Animal::Dog(_a, _b) => "Woof!",
            Animal::Duck(_a, _b, _c) => "Quack!",
            Animal::Crow(_a, _b, _c, _d) => "Caw!",
            Animal::Frog(_a, _b, _c, _d, _e) => "Croak!",
        }
    }
    pub fn sum(&self) -> i32 {
        match self {
            Animal::Cat(a) => *a,
            Animal::Dog(a, b) => a + b,
            Animal::Duck(a, b, c) => a + b + c,
            Animal::Crow(a, b, c, d) => a + b + c + d,
            Animal::Frog(a, b, c, d, e) => a + b + c + d + e,
        }
    }
    pub fn rem(&self) -> i32 {
        match self {
            Animal::Cat(a) => *a,
            Animal::Dog(a, b) => b % a,
            Animal::Duck(_a, b, c) => c % b,
            Animal::Crow(_a, _b, c, d) => d % c,
            Animal::Frog(_a, _b, _c, d, e) => e % d,
        }
    }
}
