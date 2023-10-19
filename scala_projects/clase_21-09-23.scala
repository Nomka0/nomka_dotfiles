@main def hello: Unit =
  val s = Prod(Numero(5),Numero(2))
  print(eval(s))

trait Expr 
case class Numero (n:Int) extends Expr
case class Suma(e1:Expr, e2:Expr) extends Expr
case class Prod(e1:Expr, e2:Expr) extends Expr



def eval (e: Expr) : Int = e match {
  case Numero(n)=> n
  case Suma(e1, e2)=> eval(e1) + eval(e2)
  case Prod(e1,e2)=> eval(e1) * eval(e2)
}
