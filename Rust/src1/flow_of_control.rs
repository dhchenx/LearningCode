
pub fn test_if(){
    let n=5;
    if n>10{
        println!(">10");
    }else if n>5{
        println!("<10 and >5");
    }
    else{
        println!("<5")
    }

    let m=if n>10{1}else{0};

    println!("m={}",m);

}

pub fn test_loop(){
    let mut count=0u32;
    println!("let's count");
    loop{
        count+=1;
        if count==3{
            println!("three");
            continue;
        }
        println!("{}",count);
        if count==5{
            println!("ok this is enough");
            break;
        }
    }

    // break outer loops
    'outer:loop{
        println!("outer");
        'inner:loop {
            println!("inner");
            break 'outer;
        }
        println!("never reached")
    }

    println!("exit outer loop!");

    // returning from the loops
    let mut counter=0;
    let result=loop{
        counter+=1;
        if counter==10{
            break counter*2;
        }
    };
    println!("{}",result);
    assert_eq!(result,20);

}

pub fn test_while(){
    let mut n=1;
    while n<101{
        if n%15==0{
            println!("fizzubzz");
        }else if n%3==0 {
            println!("fizz")
        }else if n%5==0 {
            println!("{}", n)
        }
        n+=1;
    }



}

pub fn test_for(){
    for n in 1..11{
        println!("{}",n);
    }


    for n in 1..=100{
        println!("{}",n);
    }

    let names=vec!["Bob","Frank","Ferris"];
    for name in names.iter(){
        match name {
            &"Ferris"=>println!("this is a rustacean among us!"),
            _=>println!("Hello, {}!",name),
        }
    }

    println!("names:{:?}",names);

    for name in names.into_iter(){
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello"),
        }
    }
    // borrow for modification
    // println!("names: {:?}", names);
    let mut names1=vec!["Bob","Frank","Ferris"];

    for name in names1.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!", //borrow for modification
            _ => "Hello",
        }
    }

    println!("names: {:?}", names1);
}

pub fn test_match() {
    let number = 13;

    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("this is prime!"),
        12..=19 => println!("a teen"),
        _ => println!("are not special"),
    }


    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("{}", binary);


    //destructuring
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("first is 0"),
        (1, ..) => println!("first is 1 adn the rest does not matter!"),
        _ => println!("else"),
    }

    //pointer
    let reference = &4;
    match reference {
        &val => println!("get a value :{:?}", val),
    }

    match *reference {
        val => println!("get a rea value: {:?}", val),
    }

    //ref
    let _not_a_reference = 3;
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;

    match (value) {
        ref r => println!("got a reference to the a value: {:?}", r),
    }


    let pair = (2, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("they are twins"),
        (x, y) if x + y==0 => println!("kaboom!"),
        (x, _) if x % 2 == 1 => println!("the first on is odd"),
        _ => println!("No correlation"),
    }

    //binding
    fn age()->u32{
        15
    }
    match age(){
        0=>println!("zero"),
        n@1..=12=>println!("1-12"),
        n@13..=19=>println!("13-19"),
        n=>println!("old!"),
    }

    fn some_number() -> Option<u32> {
        Some(42)
    }
    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n)      => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _            => (),
    }


}

pub fn test_if_let(){
    let optional=Some(7);
    match optional{
        Some(i)=>{
            println!("a real value:{}",i);
        },
        _=>{},
    }

    let number=Some(7);
    let letter:Option<i32>=None;
    let emoticon:Option<i32>=None;
    if let Some(i)=number{
        println!("Matched {:?}",i)
    }

    if let Some(i)=letter {
        println!("Matched {:?}", i);
    }else{
        println!("error");
    }
}

pub fn test_while_let(){
    let mut optional=Some(0);

    while let Some(i)=optional{
        if i>9{
            println!(">9");
            optional=None;
        }else{
            println!("{:?}",i);
            optional=Some(i+1);
        }
    }
}

