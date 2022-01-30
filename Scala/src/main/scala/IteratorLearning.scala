package org.example.test

object IteratorLearning {
  def main(args:Array[String]): Unit ={

    val it=Iterator("Baidu","Google","Runoob","Taobao")
    while(it.hasNext){
      println(it.next())
    }

    val values=Iterator(22,33,11,23,14,34)
    println("max value: " + values.max) // only call once
    values.size
    // println("min value: " + values.min)
    println("iterator size: "+values.size)
  }
}
