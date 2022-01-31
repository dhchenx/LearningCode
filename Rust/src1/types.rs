
#![allow(overflowing_literals)]

pub fn test_casting(){
    let decimal=65.4321_f32;
    println!("{}",decimal);

    let integer = decimal as u8;
    let character =integer as char;
    println!("{}->{}->{}",decimal,integer,character);

    println!("1000 as a u16 is :{}",1000 as u16);
    println!("1000 as a u8 is :{}",1000 as u8);
}

pub fn test_literals(){
    let x=1u8;
    let y=2u32;
    let z=3f32;
    let i=1;
    let f=1.0;
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

pub fn test_inference(){
     let elem = 5u8;
    let mut vec=Vec::new();
    vec.push(elem);
    println!("{:?}",vec);
}

pub fn test_aliasing(){
    type NanoSecond=u64;
    type Inch = u64;
    #[allow(non_camel_case_types)]
    type u64_t=u64;

    let nanoseconds:NanoSecond=5 as u64_t;
    let inches:Inch=2 as u64_t;
    println!("{} nanoseconds + {} inches = {} unite?",nanoseconds,inches,nanoseconds+inches);

}