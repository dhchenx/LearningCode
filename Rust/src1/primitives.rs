
pub fn use_primitives(){
    let logical:bool = true;
    let a_float:f64=1.2;
    let mut v=10;
    v=20;
    println!("One million is written as {}",1_00__000u32);

}

fn reverse(pair:(i32,bool))->(bool,i32){
    let (integer,boolean)=pair;
    (boolean,integer)
}

#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);

pub fn use_tuple(){
    let pair=(1,true);
    println!("pair is {:?}",pair);
    println!("reversed pair is {:?}",reverse(pair));

    let matrix=Matrix(1.0,2.0,3.0,4.0);
    println!("{:?}",matrix);

    let tuple=(1,"hello",4.5,true);
    let (a,b,c,d)=tuple;



}

fn analyze_slice(slice:&[i32]){
    println!("first element of the slice: {}",slice[0]);
    println!("the length of the slice: {}",slice.len());
}

pub fn use_array(){
    let xs:[i32;5] = [1,2,3,4,5];
    let ys:[i32;500]=[0;500];
    println!("{}",ys[5]);

    analyze_slice(&xs[1..3]);

}

