package org.example.test

import scala.util.control.Breaks

object LoopLearning {

  def main(args:Array[String]): Unit ={
    var a=1;
    while(a<10){
      a+=1
      println(a)
    }

    // using break
    var loop=new Breaks
    loop.breakable{
      while(true){
        a+=1
        if (a>100){
          println("breaking a= "+a)
          loop.break
        }
      }
    }

  }
}
