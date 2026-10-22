

fn main() {
    boop();
    for2();
    while2main();
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining= 10;
        loop {
            println!("remaining={remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");
}

fn boop () {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn while2main () {
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index <5 {
        println!("the value is : {}", a[index]);
        index += 1;
    }
}

fn for2() {
    let a = [1,2,3,4,5];
    for element in a {
        println!("the value is : {element}");
    }
}