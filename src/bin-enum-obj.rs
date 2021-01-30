fn main () {
    let vec = cmp_polymorphism::create_enum_objs(1000000);
    let r1 = {
        let mut r = 0;
        for _i in 0..100 {
            let rr = cmp_polymorphism::sum_id_enum_objs(&vec);
            r += rr;
        }
        r
    };
    let r2 = {
        let mut r = 0;
        for _i in 0..100 {
            let rr = cmp_polymorphism::sum_sum_enum_objs(&vec);
            r += rr;
        }
        r
    };
    let r3 = {
        let mut r = 0;
        for _i in 0..100 {
            let rr = cmp_polymorphism::sum_rem_enum_objs(&vec);
            r += rr;
        }
        r
    };
    println!("{},{},{}", r1, r2, r3);
}
