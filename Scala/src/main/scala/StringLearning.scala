package org.example.test

object StringLearning {
  val greeting:String="Hello, World!"
  def main(args:Array[String]): Unit ={
    println(greeting)

    var h="hello"
    h="say hi"
    println(h)

    val buf=new StringBuilder;
    buf+='a'
    buf++="bcdef"
    buf++="dd"
    buf+='-'
    println("buf is "+buf.toString())

    var header="Spark Test"
    var len=header.length()
    println("String length is "+len)

    var s1="Hello"
    var s2="World"
    var s3=s1.concat(s2)
    println(s3)

    var x=2
    var y=3.0
    var z=6.0
    var fs=printf("%d + %.2f = %.1f",x,y,z)
    println(fs)



  }
}
