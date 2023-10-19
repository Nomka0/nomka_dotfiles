@main def sum: Unit =
  println(chip_not(List(1)))
  println(chip_and(List(1,1)))
  println(chip_and(List(1,0)))
  println(chip_or(List(1,1)))
  println(chip_or(List(1,0)))

  println(ha(List(0,0)))
  println(ha(List(1,0)))
  println(ha(List(0,1)))
  println(ha(List(1,1)))

  println(fa(List(1,1,0)))
  println(fa(List(1,1,1)))
  println(fa(List(0,1,0)))              
  println(fa(List(0,1,1)))      

  println(add_4(List(1,0,1,1) ++ List(1,0,1,0)))

  

  

//1.1
type Chip = List[Int] => List[Int] 

val chip_not = crearChipUnario((x: Int) => (1-x))
val chip_and = crearChipBinario((x: Int, y: Int) => (x*y))
val chip_or = crearChipBinario((x: Int, y: Int) => x + y - (x*y))

def crearChipBinario(funcion:(Int,Int)=>Int): Chip = {
  (inputList: List[Int]) => List(funcion(inputList.head,inputList.last)) 
}

def crearChipUnario(funcion: Int =>Int): Chip = {
  (inputList: List[Int]) => List(funcion(inputList.head)) 
}

//1.2

val ha = half_adder

def half_adder:Chip = {
  (inputList: List[Int]) =>   
  if (inputList == List(1,1)){
    chip_and(inputList) ++ chip_not(inputList) 
  } else {
    chip_and(inputList) ++ chip_or(inputList)  
  }         
}

//1.3

val fa = full_adder


def full_adder:Chip = {
    (inputList: List[Int]) => 
    val ha1 = ha(inputList.init)
    val ha2 = ha(List(inputList.last,ha1.last))
    chip_or(List(ha2.head,ha1.head)) ++ List(ha2.last)
}


def fa_recursivo: Chip = {
  (inputList: List[Int]) => {
    if (inputList.isEmpty) List[Int]()
    else if (inputList.length != 1) {
      val list1 = inputList.slice(0, inputList.length / 2)
      val list2 = inputList.slice(inputList.length / 2, inputList.length)
      val sum = fa(List(list1.last, list2.last))
      val carry = sum.head
      val result = sum.last
      val recursiveSum = fa_recursivo(list1.init ++ List(carry)) ++ List(result)
      recursiveSum ++ List(list2.last)
    } else {
      List(inputList.head)
    }
  }
}
val add_2 = adder(2)
val add_4 = adder(4)


def adder(n: Int): Chip = {
  (inputList: List[Int]) => {
    if(inputList.length == 2*n) {
      def concatenar(bit: Int, carry: Int, resultado: List[Int]): List[Int] = {
        if(bit < 0) carry :: resultado
        else {
          val a = inputList(bit)
          val b = inputList(bit+n)
          val s = (a ^ b) ^ carry
          val cout= (a & b) | ((a ^ b) & carry)
          concatenar(bit-1, cout, s :: resultado)
        }
      }
      concatenar(n-1, 0, List[Int]())
    }
    else List[Int]()
  }
}
