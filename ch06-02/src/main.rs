fn main() {
    let a: Option<Option<i32>> = Option::Some(Option::Some(54));

    match a {
        Some(Some(mut a)) => {
            a = a+1;
            println!("Nice {a}")
        },
        Some(None) => println!("Not Nice"),
        None => println!("Nope"),
    }

    match a {
        Some(Some(mut a)) => {
            a = a+1;
            println!("Nice {a}")
        },
        Some(None) => println!("Not Nice"),
        None => println!("Nope"),
    }

    match &a {
        Some(Some(mut a)) => {
            a = a+1;
            println!("Nice {a}")
        },
        Some(None) => println!("Not Nice"),
        None => println!("Nope"),
    }

    match a {
        Some(Some(mut a)) => {
            a = a+1;
            println!("Nice {a}")
        },
        Some(None) => println!("Not Nice"),
        //None => println!("Nope"),
        //Some(None) => println!("Not Nice"),
    }

    
}
