@main def sum: Unit =
  //pruebas punto 1.1
  val viaTrenes: Estado = (List('a','b'), List('d'),List('c'))
  val movimiento1: Estado = Uno(-1).cambiarEstado(viaTrenes)
  val movimiento2: Estado = Uno(2).cambiarEstado(viaTrenes)
  //println(movimiento1)
  //println(movimiento2)

  //pruebas punto 1.2.1
  val e1 = (List('a', 'b', 'c', 'd'), Nil, Nil)
  val e2 = aplicarMovimiento(e1, Uno(2))
  val e3 = aplicarMovimiento(e2, Dos(3))
  val e4 = aplicarMovimiento(e3, Dos(-1))
  val e5 = aplicarMovimiento(e4, Uno(-2))
/*
  println(e2)
  println(e3)
  println(e4)
  println(e5)
*/
  //pruebas punto 1.2.2
  val e = (List('a', 'b'), List('c'), List('d'))
//  println(aplicarMovimientos(e, List(Uno(1), Dos(1), Uno(-2))))

  val t1 = List('d', 'b', 'a', 'c')
  val t2 = List('d', 'b', 'c', 'a')

  val movs = definirManiobra(t1, t2)
  println(movs) // Esto imprimirá la lista de movimientos

//1.1

type Vagon = Any
type Tren = List[Vagon]
type Estado = (Tren, Tren, Tren)
type Maniobra = List[Movimiento] //Para el punto 1.2.2

trait Movimiento {
  def cambiarEstado(estadoActual: Estado) : Estado

}
case class Uno(n: Int) extends Movimiento {
  def cambiarEstado(estadoActual: Estado): Estado = estadoActual match {
    case (principal, uno, dos) if n > 0 => 
      val(vagonesSinMover,vagonesAMover) = if (n > principal.length){
        (Nil, principal)
      } 
      else principal.splitAt(n)
      (vagonesSinMover, vagonesAMover ++ uno , dos)
    case (principal, uno, dos) if n < 0 => 
      val(vagonesAMover,vagonesSinMover) = uno.splitAt(-n)
      (principal ++ vagonesAMover, vagonesSinMover, dos)
    case (principal, uno, dos) if n == 0 => estadoActual
  }
}
case class Dos(n: Int) extends Movimiento {
  def cambiarEstado(estadoActual: Estado): Estado = estadoActual match {
    case (principal, uno, dos) if n > 0 => 
      val(vagonesSinMover,vagonesAMover) = if (n >= principal.length){
        (Nil, principal)
      } 
      else principal.splitAt(n)
      (vagonesSinMover, uno , vagonesAMover ++ dos)
    case (principal, uno, dos) if n < 0 => 
      val(vagonesAMover,vagonesSinMover) = dos.splitAt(-n)
      (principal ++ vagonesAMover, uno , vagonesSinMover)
    case (principal, uno, dos) if n == 0 => estadoActual
  }
}

//1.2

//1.2.1

def aplicarMovimiento(e: Estado, m: Movimiento): Estado = m match {
  case u: Uno => u.cambiarEstado(e)
  case d: Dos => d.cambiarEstado(e)
}

//1.2.2


def aplicarMovimientos(e: Estado, movs: Maniobra): List[Estado] = movs match {
  case no_terminado: Maniobra if !movs.isEmpty => 
    val first = aplicarMovimiento(e, movs(0))
    first :: aplicarMovimientos(first, movs.tail) 
  case m: Maniobra if movs.isEmpty => List[Estado]()
}

//1.2.3


def definirManiobra(t1:Tren, t2:Tren) = definirManiobra_recursiva(t1,t2,true,true)

def definirManiobra_recursiva(t1: Tren, t2: Tren, uno: Boolean, dos: Boolean): Maniobra = {
  val tamano = t1.length

  t1 match {
    case vias_iguales: Tren if t1 == t2 => List(Uno(0))
    case vagon_igual: Tren if t1.head == t2.head => definirManiobra(t1.tail, t2.tail)
    case vagon_diferente: Tren if t1.head != t2.head && (uno  && dos ) => 
      List(Uno(tamano)) ++ definirManiobra_recursiva(t1,t2,false,true)
    case ultimo_primero: Tren if t1.last == t2.head && (!uno && dos)=> //vamos a comparar si el primero es igual al último, para de ese modo concatenarlo después
      List(Uno(-(tamano-1)))++ List(Dos(tamano-1)) ++ List(Uno(-1)) ++ definirManiobra_recursiva(t1,t2,true,false)
    case primero_ultimo: Tren if t1.head == t2.last && (uno && !dos) =>
      List(Dos(-1)) ++ List(Uno(1)) ++ definirManiobra_recursiva(t1,t2,false,false)
    case concatenar_resto: Tren if t1.head == t2.last && (!uno && !dos) =>
      definirManiobra_recursiva(t1.init.tail, t2,true,false) ++ List(Uno(-1))
    case concatenar_uno: Tren if t1 == t2.init.tail &&(uno && !dos) =>
      def devolver_uno_a_uno (n: Int) : Maniobra = {
        if(n <= 0) List[Movimiento]()
        else List(Dos(-1)) ++ devolver_uno_a_uno(n-1)
        }
       
      devolver_uno_a_uno(tamano)
    case _=> List[Movimiento]() 
  }
}

