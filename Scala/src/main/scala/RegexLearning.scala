package org.example.test

import scala.util.matching.Regex

object RegexLearning {

  def main(args:Array[String]): Unit ={
    var pattern="Scala".r
    var str="Scala is scalable and cool!"
    println(pattern findFirstIn str)

    pattern=new Regex("(S|s)cala")
    str="Scala is scalable and cool"
    println((pattern findAllIn str).mkString(","))

    pattern="(S|s)cala".r
    str="Scala is scalable and cool"
    println(pattern replaceFirstIn(str,"Java"))

  }
}
