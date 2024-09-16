fn _vector_collection() {
    let _v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    for val in v {
        println!("{}", val);
    }
    let mut other_v = Vec::new();
    other_v.push(5);
    other_v.push(6);
    other_v.push(7);
    other_v.push(8);
    for val in other_v {
        println!("{}", val);
    }

    {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        let third: &i32 = &v[2];
        println!("Third element: {}", third);
    }

    {
        let v = vec![1, 2];
        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("Third element: {}", third),
            None => println!("There's no third element"),
        }
    }
}

pub fn loop_vecs() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    println!("\nAgora vou iterar estes valores: \n");
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 25;
            println!("{}", i);
        }
    }
}
