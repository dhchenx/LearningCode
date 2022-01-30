package org.example.test

class Point(xc:Int, yc:Int){
  var x:Int=xc
  var y:Int=yc
  def move(dx:Int, dy:Int): Unit ={
    x=x+dx
    y=y+dy
    println("x's value:"+x)
    println("y's value:"+y)
  }
}

class Location(val xc:Int, val yc:Int, val zc:Int) extends Point(xc,yc){
  var z:Int=zc
  def move(dx:Int, dy:Int, dz:Int): Unit ={
    x=x+dx
    y=y+dy
    z=z+dz
    println("x="+x)
    println("y="+y)
    println("z="+z)

  }
}

object ClassLearning {

  def main(args:Array[String]): Unit ={
    val pt=new Point(10,20)
    pt.move(10,20)

    val loc=new Location(10,20,15)
    loc.move(10,10,5)
  }

  //////////////////////////////////////////////////

  class Person{
    var name=""
    override def toString=getClass.getName+"[Name="+name+"]"
  }

  class Employee extends Person{
    var salary=0.0
    override  def toString=super.toString+"[Salary="+salary+"]"
  }

  object Test extends App{
    val fred = new Employee
    fred.name="Fred"
    fred.salary=5000
    println(fred)
  }

  ///////////////////////////


}


