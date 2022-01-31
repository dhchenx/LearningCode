use std::mem;

struct Point{
    x:f64,
    y:f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle{
    p1:Point,
    p2:Point,
}

impl Rectangle{
    fn area(&self)->f64{
        let Point{x:x1,y:y1}=self.p1;
        let Point{x:x2,y:y2}=self.p2;
        (x1-x2)*(y1-y2).abs()
    }

    fn perimeter(&self)->f64{
        let Point{x:x1,y:y1}=self.p1;
        let Point{x:x2,y:y2}=self.p2;
        2.0*((x1-x2).abs()+(y1-y2).abs())
    }

    fn translate(&mut self,x:f64,y:f64){
        self.p1.x+=x;
        self.p2.x+=x;
        self.p1.y+=y;
        self.p2.y+=y;
    }
}

struct Pair(Box<i32>,Box<i32>);

impl Pair{
    fn destroy(self){
        let Pair(first, second)=self;
        println!("destroying Pair({},{})",first,second);
    }
}

pub fn test_method(){
    let rectangle=Rectangle{
        p1:Point::origin(),
        p2:Point::new(3.0,4.0),
    };

    println!("the perimeter:{}",rectangle.perimeter());
    println!("the area:{}",rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0,1.0);

    let pair=Pair(Box::new(1),Box::new(2));
    pair.destroy();



}

pub fn test_closures(){

    fn function(i:i32)->i32{i+1};

    let closure_annotated=|i:i32|->i32{i+1};
    let closure_inferred=|i|i+1;

    let i=1;
    println!("function:{}",function(i));
    println!("annoated:{}",closure_annotated(i));
    println!("inferred:{}",closure_inferred(i));

    let one = || 1;
    println!("{}",one());

    // move usage
    let haystack=vec![1,2,3];
    let contains=move |needle| haystack.contains(needle);

    println!("{}",contains(&1));
    println!("{}",contains(&4));

    // println!("there are {} elements",haystack.len());

    fn apply<F>(f:F) where F:FnOnce() {
      f();
    };

    fn apply_to_3<F>(f:F)->i32 where F:Fn(i32)->i32{
      f(3)
    };

    use std::mem;
    let greeting ="hello";
    let mut farewell="goodbye".to_owned();

    let diary =||{
        println!("I said {}. ", greeting);

        farewell.push_str("!!!");
        println!("The I screamed {}.",farewell);
        println!("Now I can sleep.zzz");

        mem::drop(farewell);
    };

    apply(diary);

    let double=|x|->i32{2 * x};
    println!("3 doubled: {}",apply_to_3(double));

    fn apply1<F>(f:F) where
    F:Fn(){
        f();
    };

    let x=7;
    let print=||println!("{}",x);

    apply1(print);

    //return function
    fn create_fn()->impl Fn(){
        let text="Fn".to_owned();
        move || println!("This is a: {}",text)
    };

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();

        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();

    // examples
    //Iterator

    let vec1=vec![1,2,3];
    let vec2=vec![4,5,6];
    println!("2 in vect1: {}",vec1.iter().any(|&x|x==2));
    println!("2 in vect2: {}",vec2.into_iter().any(|x|x==2));

    let array1=[1,2,3];
    let array2=[4,5,6];

    println!("2 in array1: {}",array1.iter().any(|&x|x==2));
    println!("2 in array2: {}",array2.into_iter().any(|x|x==2));

    let vec1=vec![1,9,3,3,13,2];
    let v=vec1.iter().find(|&&x|x%2==0);
    let index=vec1.iter().position(|x|x%2==0);
    println!("{:?}",v);
    println!("{:?}",index);




}

pub fn test_higher_order_func(){

    fn is_odd(n:u32)->bool {
        n % 2 == 1
    }

    let upper=1000;

    let mut acc=0;
    // imperative approach
    for n in 0..{
        let n_squared=n*n;
        if n_squared>=upper{
            break;
        }else if is_odd(n_squared){
            acc+=n_squared;
        }
    }
    println!("imperative style: {}", acc);
    // Functional approach
    let sum_of_sequared_odd_numbers:u32=(0..).map(|n|n*n).take_while(|&n_squared|n_squared<upper)
        .filter((|&n_squared|is_odd(n_squared)))
        .fold(0,|acc,n_squared|acc+n_squared);
    println!("function style: {}",sum_of_sequared_odd_numbers);

}

pub fn test_diverging(){
    fn foo()->(){
       // panic!("this call never returens");
        println!("exit");
    }

    foo();

}