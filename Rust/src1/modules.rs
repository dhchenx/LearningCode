use crate::hello_world::show_color;

pub fn test_modules(){
    test_function();
}

mod my_mod{

    pub fn say_hi(s:String){
        println!("Hi, {}!",s);
        think();
    }

    fn think(){
        println!("I am thinking");
    }

    pub mod nested_mod{
        pub fn set_age(age:u32){
            println!("age:{}",age);
        }
    }

}

fn test_function(){

    my_mod::say_hi(String::from("Chen"));
    my_mod::nested_mod::set_age(11);

    show_color();

    // println!("add={}",add::value(1,2))


}