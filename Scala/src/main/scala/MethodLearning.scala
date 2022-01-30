package org.example.test

object MethodLearning {

  class Test{
    def method(x:Int)=x+3
    val func=(x:Int)=>x+3
  }

  def addInt(a:Int,b:Int):Int={
    var sum:Int=0;
    sum=a+b
    sum
  }

  def printMe(): Unit ={
    println("hello scala!")
  }

  def main(args:Array[String]): Unit ={
    var a=1
    var b=2
    var c=addInt(a,b)
    println(c)
    printMe()

    var factor=3
    val multiplier=(i:Int)=>i*factor
    println("multiplier(2) = "+multiplier(2))

    val volume=(w:Int,h:Int,l:Int)=>w*h*l
    println("volume(2,3,4) = "+volume(2,3,4))

  }
}
