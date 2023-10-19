package object FuncionesRecursivas {

  def tamIter(cont: Int, l: List[Int]): Int = {
    if (l.isEmpty) cont
    else tamIter(cont + 1, l.tail)
  }

  def tamI(l: List[Int]) = tamIter(0, l)

  def menoresQue(l: List[Int], v: Int): List[Int] = {
    if (l.isEmpty) {
      List[Int]()
    }
    else if (l.head < v) {
      l.head :: menoresQue(l.tail, v)
    }
    else {
      menoresQue(l.tail, v)
    }
  }

  def noMenoresQue(l: List[Int], v: Int): List[Int] = {
    if (l.isEmpty) {
      List[Int]()
    }
    else if (l.head >= v) {
      l.head :: noMenoresQue(l.tail, v)
    }
    else {
      noMenoresQue(l.tail, v)
    }
  }

  def k_elem(l: List[Int], k: Int): Int = {
    //devuelve el k-esimo elemento de l: supone 0 < k <= longitud de l_List
    val menores = menoresQue(l, l.head)
    val noMenores = noMenoresQue(l, l.head)
    /*
      println(menores)
      println(noMenores)
    */
    if (k <= tamI(menores)) {
      // El k-ésimo elemento está en la lista de menores
      k_elem(menores, k)
    }
    else if (k == tamI(menores) + 1) {
      // El k-ésimo elemento es el pivote actual
      l.head
    }
    else if (tamI(noMenores) == 1) {
      // El k-ésimo elemento es el pivote actual de los mayores
      noMenores.head
    }
    else {
      // El k-ésimo elemento está en la lista de no menores
      k_elem(noMenores, k - tamI(menores) - 1)
    }
  }

  def ordenar(l: List[Int]): List[Int] = {
    if (l.isEmpty) {
      List[Int]()
    } else {
      val pivote = l.head
      // parece un poco confuso, pero básicamente lo que estamos haciendo es, dentro de los valores "menores" y "mayores" llamar a noMenoresQue u menoresQue, cada vez con un nuevo pivote, y excluyendo el valor anteriormente evaluado. De ese modo, se hacen listas cada vez más chiquitas hasta que se cumpla la condición de arriba (l.isEmpty == true), de ese modo se organiza la lista, se llama hasta que la lista esté vacía, y así se va a dar con una lista ordenada. Al final solo se concatena todo
      val mayores = ordenar(noMenoresQue(l.tail, pivote))
      val menores = ordenar(menoresQue(l.tail, pivote))

      //println(menores)
      //println(mayores)
      menores ++ List(pivote) ++ mayores
    }
  }

}
