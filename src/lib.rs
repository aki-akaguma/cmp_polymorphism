pub mod enum_obj;
pub mod trait_obj;

pub fn do_trait_obj(
    count: i32,
) -> anyhow::Result<(
    (&'static str, i32),
    (&'static str, i32),
    (&'static str, i32),
    (&'static str, i32),
    (&'static str, i32),
)> {
    let a: Box<dyn trait_obj::Animal> = Box::new(trait_obj::Cat::new(count));
    let b: Box<dyn trait_obj::Animal> = Box::new(trait_obj::Dog::new(count, count + 1));
    let c: Box<dyn trait_obj::Animal> = Box::new(trait_obj::Duck::new(count, count + 1, count + 2));
    let d: Box<dyn trait_obj::Animal> =
        Box::new(trait_obj::Crow::new(count, count + 1, count + 2, count + 3));
    let e: Box<dyn trait_obj::Animal> = Box::new(trait_obj::Frog::new(
        count,
        count + 1,
        count + 2,
        count + 3,
        count + 4,
    ));
    Ok((
        (a.talk(), a.sum()),
        (b.talk(), b.sum()),
        (c.talk(), c.sum()),
        (d.talk(), d.sum()),
        (e.talk(), e.sum()),
    ))
}

pub fn do_enum_obj(
    count: i32,
) -> anyhow::Result<(
    (&'static str, i32),
    (&'static str, i32),
    (&'static str, i32),
    (&'static str, i32),
    (&'static str, i32),
)> {
    let a: Box<enum_obj::Animal> = Box::new(enum_obj::Animal::Cat(count));
    let b: Box<enum_obj::Animal> = Box::new(enum_obj::Animal::Dog(count, count + 1));
    let c: Box<enum_obj::Animal> = Box::new(enum_obj::Animal::Duck(count, count + 1, count + 2));
    let d: Box<enum_obj::Animal> = Box::new(enum_obj::Animal::Crow(
        count,
        count + 1,
        count + 2,
        count + 3,
    ));
    let e: Box<enum_obj::Animal> = Box::new(enum_obj::Animal::Frog(
        count,
        count + 1,
        count + 2,
        count + 3,
        count + 4,
    ));
    Ok((
        (a.talk(), a.sum()),
        (b.talk(), b.sum()),
        (c.talk(), c.sum()),
        (d.talk(), d.sum()),
        (e.talk(), e.sum()),
    ))
}

pub fn create_trait_objs(count: usize) -> Vec<Box<dyn trait_obj::Animal>> {
    let v: Vec<Box<dyn trait_obj::Animal>> = vec![
        Box::new(trait_obj::Cat::new(1)) as Box<dyn trait_obj::Animal>,
        Box::new(trait_obj::Dog::new(1, 2)) as Box<dyn trait_obj::Animal>,
        Box::new(trait_obj::Duck::new(1, 2, 3)) as Box<dyn trait_obj::Animal>,
        Box::new(trait_obj::Crow::new(1, 2, 3, 4)) as Box<dyn trait_obj::Animal>,
        Box::new(trait_obj::Frog::new(1, 2, 3, 4, 5)) as Box<dyn trait_obj::Animal>,
    ]
    .into_iter()
    .cycle()
    .take(count)
    .collect();
    v
}
pub fn sum_id_trait_objs(vec: &Vec<Box<dyn trait_obj::Animal>>) -> i32 {
    let mut acc = 0;
    for a in vec {
        acc += a.animal_id();
    }
    acc
}
pub fn sum_sum_trait_objs(vec: &Vec<Box<dyn trait_obj::Animal>>) -> i32 {
    let mut acc = 0;
    for a in vec {
        acc += a.sum();
    }
    acc
}
pub fn sum_rem_trait_objs(vec: &Vec<Box<dyn trait_obj::Animal>>) -> i32 {
    let mut acc = 0;
    for a in vec {
        acc += a.rem();
    }
    acc
}

pub fn create_enum_objs(count: usize) -> Vec<Box<enum_obj::Animal>> {
    let v: Vec<Box<enum_obj::Animal>> = vec![
        Box::new(enum_obj::Animal::Cat(1)) as Box<enum_obj::Animal>,
        Box::new(enum_obj::Animal::Dog(1, 2)) as Box<enum_obj::Animal>,
        Box::new(enum_obj::Animal::Duck(1, 2, 3)) as Box<enum_obj::Animal>,
        Box::new(enum_obj::Animal::Crow(1, 2, 3, 4)) as Box<enum_obj::Animal>,
        Box::new(enum_obj::Animal::Frog(1, 2, 3, 4, 5)) as Box<enum_obj::Animal>,
    ]
    .into_iter()
    .cycle()
    .take(count)
    .collect();
    v
}
pub fn sum_id_enum_objs(vec: &Vec<Box<enum_obj::Animal>>) -> i32 {
    let mut acc = 0;
    for a in vec {
        acc += a.animal_id();
    }
    acc
}
pub fn sum_sum_enum_objs(vec: &Vec<Box<enum_obj::Animal>>) -> i32 {
    let mut acc = 0;
    for a in vec {
        acc += a.sum();
    }
    acc
}
pub fn sum_rem_enum_objs(vec: &Vec<Box<enum_obj::Animal>>) -> i32 {
    let mut acc = 0;
    for a in vec {
        acc += a.rem();
    }
    acc
}
