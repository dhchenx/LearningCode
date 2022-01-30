package org.example.test

class Outer{
  class Inner{
    private def f(): Unit ={
      println("f")
    }
    class InnerMost {
      f()
    }
  }
  // (new Inner).f() //错误
}

class Super{
  protected def f(): Unit ={
    println("f")
  }
  class sub extends Super{
    f()
  }
  class Other{
    // (new Super).f() error!
  }
}

object ModifierLearning {
  def main(args:Array[String]): Unit ={

  }
}
