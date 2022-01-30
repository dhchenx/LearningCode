package org.example.test

object MatchLearning {
  def main(args:Array[String]): Unit ={
    println(matchTest(3))

    val alice= Person("Alice",25)
    val bob = Person("Bob",32)
    val charlie= Person("Charlie",32)
    for(person<-List(alice,bob,charlie)){
      person match {
        case Person("Alice",25)=>println("Hi, alice!")
        case Person("Bob",32)=>println("Hi, bob")
        case Person(name,age)=>
          println("Age: "+age+" Name: "+name)
      }
    }

  }

  def matchTest(x:Int):String=x match{
    case 1 => "one"
    case 2=> "two"
    case 3=>"three"
    case _=>"else"
  }

  case class Person(name:String, age:Int)

}
