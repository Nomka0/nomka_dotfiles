@main def sum: Unit =
  println(chip_not(List(1)))
  println(chip_and(List(1,1)))
  println(chip_and(List(1,0)))
  println(chip_or(List(1,1)))
  println(chip_or(List(1,0)))

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

