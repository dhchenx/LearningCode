
pub fn test_mutability(){
    let an_integer=1u32;
    let a_boolean=true;
    let unit=();

    let copied_integer=an_integer;

    let _unused_variable_variable =3u32;
    let noisy_unused_variable=2u32;

    println!("an integer: {:?}",copied_integer);
    println!("an integer: {:?}",an_integer);

    //Mutability
    let _immutable_binding=1;
    let mut mutable_biding=1;

    println!("before mutation:{}",mutable_biding);
    mutable_biding+=1;

    println!("after mutation:{}",mutable_biding);

    // _immutable_binding+=1;

}

pub fn test_scope_and_shadowing(){
    let long_lived_binding=1;
    {
        let short_lived_binding=2;
        println!("inner short:{}",short_lived_binding);
    }

    // println!("outer short:{}",short_lived_binding);

    let shadowed_binding=1;
    {
        let shadowed_binding="abc";
        println!("shadowed in inner block: {}",shadowed_binding);
    }

    println!("outside inner block:{}",shadowed_binding);


}

pub fn test_declare_first(){

    let a_binding;

    {
        let x=2;
        a_binding=x*x;
    }

    println!("a binding : {}",a_binding);


}

pub fn test_freezing(){
    let mut _mutable_integer=7i32;
    {
        let _mutable_integer=_mutable_integer;
        // _mutable_integer=50;
    }

    _mutable_integer=30;
}