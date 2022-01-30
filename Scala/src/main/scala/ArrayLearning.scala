package org.example.test

object ArrayLearning {

  def main(args:Array[String]): Unit ={
    var z:Array[String]=new Array[String](3)
    z(0)="Hello"
    z(1)="World"
    z(2)="!"
    println(z)

    z=Array("Hi","Me")

    var myList=Array(1.9,2.9,3.4,3.5)
    for(x<-myList){
      println(x)
    }

    var total=0.0
    for(i<-0 to (myList.length-1)){
      total+=myList(i)
    }
    println("Total is "+total)



  }

}
