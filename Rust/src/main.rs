mod mylib;
use std::fs::File;

fn main1(){
    let mut a=2;
    a=4;
    let b=22;
    let c=44;
    let d;
    d=a+b;
    print!("hello");

    let x=5;
    let x=x+1;
    let x=x*2;
    println!("The value of x is : {}",x);

    let x=2.0;
    let y:f32=3.0;
    let z=2_2;

    println!("x={}",x);
    println!("z={}",z);

    let tup:(i32,f64,u8)=(500,5.4,1);
    let (x,y,z)=tup;
    println!("{}",y);

    let a=[1,2,3,4,5];
    let b=["Jan","Feb","March"];
    let c:[i32;5]=[1,2,3,4,5];

    let a=1;
    let b=2;
    let result=calc(a,b);
    println!("result is {}",result);

    let y={
        let t=3;
        t+1
    };
    println!("{}",y);

    if result < 5 {
        println!("<5");
    }else {
        println!(">5");
    }

    let a=3;
    let number = if a>0 {1} else {-1};

    println!("r={}",number);

    let mut n=10;
    let mut sum=0;
    while n>0 {
        n-=1;
        sum+=n;
    }
    println!("re = {}",sum);

    let a=[10,20,30,40,50];
    for i in a.iter(){
        println!("value is {}",i);
    }

    for i in 0..5{
        println!("v is {}, {}",i, a[i]);
    }
    let mut n=10;
    loop{
        n-=1;
        if n < 0 {
            println!("breaking...");
            break;
            
        }
    }
    println!("exit 1");

    let a=[1,2,3,4,5,6];
    let mut i=0;
    let loc=loop{
        i+=1;
        if a[i]==3{
            break i;
        }
    };

    println!("loc -> {} ", loc);

    let s1=String::from("Hello");
    let s2=s1.clone();
    println!("{}, world!",s2);

    let s1=String::from("hello");
    let mut s2=&s1;
    let s3=s2;
    s2=&s3;
    println!("{}",s2);

    let s=String::from("broadcast");
    let part1=&s[0..5];
    let part2=&s[5..9];
    println!("{} = {} + {}",s,part1,part2);

    let runoob=Site{
        domain:String::from("www.runoob.com"),
        name:String::from("RUNNOOB"),
        nation:String::from("China"),
        found:2013
    };

    println!("{}",runoob.domain);

    // println!("rect1 is {:?}",runoob);

    struct Rectangle{
        width:u32,
        height:u32
    }

    impl Rectangle{
        fn area(&self)->u32{
            self.width*self.height
        }
        fn create(width:u32,height:u32)->Rectangle{
            Rectangle{width,height}
        }
    }

    let rect1=Rectangle{width:30,height:30};
    println!("rect1's area is {}",rect1.area());

    let rect=Rectangle::create(30,50);

    enum Book{
        Papery, Electronic
    }

    let book=Book::Papery;
    // println!("{}",book);

    enum Book1{
        Papery{index:u32},
        Electronic{url:String}
    }

    let book=Book1::Papery{index:1001};
    let ebook=Book1::Electronic{url:String::from("url...")};
    match book{
        Book1::Papery{index}=>{
            println!("Papeery book {}", index)
        },
        Book1::Electronic{url}=>{
            println!("E-book {}",url)
        }
    }

    enum Option<T>{
        Some(T),
        None,
    }
    let opt=Option::Some("Hello");
    match opt{
        Option::Some(something)=>{
            println!("{}",something)
        }
        Option::None=>{
            println!("none")
        }
    }
    
    let i=0;
    match i{
        0=>println!("zero"),
        _=>{},
    }

    let i=0;
    if let 0 = i{
        println!("zero!");
    }

    println!("This is the main module: {}",mylib::message());

    govern();

    println!("v = {}",(PI/2.0).sin());

    // panic!("error occured!");
    println!("Hello, Rust!");

    
    
    let f=File::open("hello.txt");
    match f{
        Ok(file)=>{
            println!("File Opened!");
        },
        Err(err)=>{
            println!("Failed to open the file");
        }
    }
    
    struct Point<T>{
        x:T,
        y:T
    }

    impl<T> Point<T>{
        fn x(&self)->&T{
            &self.x
        }
    }

    let p1=Point{x:1,y:2};
    let p2=Point{x:1.0,y:2.2};

    println!("p.x={}",p1.x());
    
    let args=std::env::args();
    println!("{:?}",args);

}



use std::f64::consts::PI;

use crate::nation::government::govern;

mod nation{
    pub mod government{
        pub fn govern(){
            println!("govern!");
        }
    }
}

struct Site{
    domain: String,
    name: String,
    nation: String,
    found: u32
}


fn calc(a:i32,b:i32)->i32{
    return a+b;
}

use std::io::stdin;

fn main2(){
    let mut str_buf=String::new();
    stdin().read_line(&mut str_buf).expect("Failed to read line.");
    println!("your input line is: {}",str_buf);
}
use std::fs;

fn main3(){
    
    let text=fs::read_to_string("D:\\UIBEResearch\\perl\\cuis_umls_sim.txt").unwrap();
    println!("{}",text);
}

fn main4(){
    let vector:Vec<i32>=Vec::new();
    let vector=vec![1,2,4,8];
    
    let mut vector=vec![1,2,4,8];
    vector.push(15);
    println!("{:?}",vector);
}

fn main5(){
    let mut v=vec![1,2,4,8];
    println!("{}",match v.get(0){
        Some(value)=>value.to_string(),
        None=>"None".to_string()
    });
}

fn main6(){
    let string=String::new();
    let one=1.to_string();
    let float=1.3.to_string();
    println!("{}",float);
}
use std::collections::HashMap;

fn main(){
    let mut map=HashMap::new();
    map.insert("color","red");
    map.insert("size","10 m^2");
    println!("{}",map.get("color").unwrap());
}

