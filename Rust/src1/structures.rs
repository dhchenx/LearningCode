


pub fn use_structure(){
    #[derive(Debug)]
    struct Person{
        name:String,
        age:i32,
    }
    let name = String::from("Peter");
    let age=27;
    let peter=Person{name,age};
    println!("{:?}",peter);

    struct Point{
        x:u32,
        y:u32,
    }

    let p=Point{x:1,y:2};
    println!("({},{})",p.x,p.y);

    let Point{x:a,y:b}=p;

    struct Pair(u32,f32);
    let pair=Pair(1,0.1);
    println!("{} - {}",pair.0,pair.1)

}

enum WebEvent{
    PageLoad,
    PageUnload,
    KeyPress(char),
    Click(u32,u32),
}

fn inspect(event:WebEvent){
    match event{
        WebEvent::PageLoad=>println!("page loaded"),
        WebEvent::PageUnload=>println!("page unloaded"),
        WebEvent::KeyPress(c)=>println!("pressed {}",c),
        WebEvent::Click(x,y)=>println!("cliked {},{}",x,y),
    }
}

type WE=WebEvent;

use crate::structures::List::*;

enum List{
    Cons(u32,Box<List>),
    Nil,
}

impl List{
    fn new()->List{
        Nil
    }

    fn prepend(self, elem:u32)->List{
        Cons(elem,Box::new(self))
    }

    fn len(&self)->u32{
        match *self{
            Cons(_,ref trail)=>1+trail.len(),
            Nil=>0,
        }
    }

    fn stringify(&self)->String{
        match *self{
            Cons(head,ref trail)=>{
                format!("{},{}",head,trail.stringify())
            },
            Nil=>{
                format!("Nil")
            }
        }
    }

}

pub fn use_enums(){

    let pressed=WebEvent::KeyPress('H');
    let clicked=WebEvent::Click(111,323);

    inspect(pressed);
    inspect(clicked);
    inspect(WebEvent::PageLoad);
    inspect(WE::PageUnload);

    use WebEvent::Click;

    inspect(Click(1,2));

    enum Number{
        Zero,
        One,
        Two
    }

    println!("zero is {}",Number::Zero as i32);

    let mut list=List::new();
    list=list.prepend(1);
    list=list.prepend(2);
    list=list.prepend(3);

    println!("linked list has length:{}",list.len());
    println!("{}",list.stringify());

}

pub fn use_constants(){
    static LANGUAGE:&str="Rust";
    const THRESHOLD:i32=10;

}