package org.example.test

import java.io.{FileNotFoundException, FileReader, IOException}

object ExceptionLearning {

  def main(args:Array[String]): Unit ={
    try {
      val f = new FileReader("input.txt")
    } catch {
      case ex:FileNotFoundException=>{
        println("Missing file exception")
      }
      case ex:IOException=>{
        print("IO Exception")
      }
    }finally {
      println("exiting finally")
    }

  }

}
