package org.example.test

import java.io._
import scala.io.{Source, StdIn}

object FileLearning {
  def main_read_file(args:Array[String]): Unit ={
    val writer=new PrintWriter(new File("test.txt"))
    writer.write("Test")
    writer.close()
  }

  def main_input(args:Array[String]): Unit ={
    println("Please input a number:")
    val num:Int=StdIn.readLine().toInt
    println("Your input is: "+num)
  }

  def main(args:Array[String]): Unit ={
    println("file content is:")
    Source.fromFile("test.txt").foreach{
      print
    }
  }

}
