use std::fmt;

pub fn test_from_to(){
    let my_str="hello";
    let my_string=String::from(my_str);

    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value:i32,
    }
    impl From<i32> for Number{
        fn from(item:i32) -> Self {
            Number{value:item}
        }
    }

    let num=Number::from(30);
    println!("my number is {:?}",num);

    let int1 =5;
    let num1:Number=int1.into();
    println!("My Number is {:?}",num1);


}

pub fn test_tryfrom_tryinto(){
    use std::convert::TryFrom;
    use std::convert::TryInto;

    #[derive(Debug,PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber{
        type Error=();
        fn try_from(value:i32)->Result<Self,Self::Error>{
            if value%2==0{
                Ok(EvenNumber(value))
            }else{
                Err(())
            }
        }
    }

    assert_eq!(EvenNumber::try_from(8),Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5),Err(()));

    let result:Result<EvenNumber,()>=8i32.try_into();


}

pub fn test_to_from(){
    struct Circle{
        radius:i32,
    }

    impl fmt::Display for Circle{
        fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
            write!(f,"Circle of radius {}",self.radius)
        }
    }

    let circle=Circle{radius:6};
    println!("{}",circle.to_string());

    let parsed:i32="5".parse().unwrap();
    let re=parsed*2;
    println!("result = {}",re)

}