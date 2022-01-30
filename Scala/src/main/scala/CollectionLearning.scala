package org.example.test

object CollectionLearning {
  def main(args:Array[String]): Unit ={
    val x=List(1,2,3,4)
    val y=Set(1,3,5,7)
    val z=Map("one"->1,"two"->2,"three"->3)

    val t=(10, "spark")

    val e:Option[Int]=Some(5)

    val name=z get "one"
    name match {
      case Some(name)=>
        println("find value:"+name)
      case None=>
        println("Not found!")
    }

    val map = Map("France"->"Paris","Japan"->"Tokyo","China"->"Beijing")
    val key=map get "France2"
    var upper = key map {_.trim} filter{_.length!=0} map {_.toUpperCase}
    print(upper getOrElse "--")

  }
}
