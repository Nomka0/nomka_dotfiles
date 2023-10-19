Para demostrar esto usaremos inducción

# Base de la Inducción:

Para n=0, w^0 es la cadena vacía (ε), por lo que ∣w^0∣=0. Para m=0, wm también es la cadena vacía (ε), por lo que ∣wm∣=0. Por lo tanto, ∣w0∣+∣w0∣= 0 + 0 = 0, y w^0+0 = w^0, por lo que la igualdad se cumple en el caso base.

# Hipótesis de Inducción:

Supongamos que la igualdad ∣w^n+m∣=∣w^n∣+∣w^m∣ es cierta para un valor arbitrario k, donde k es un número natural, es decir:
∣w^k∣=∣w^n∣+∣w^(k-n)∣

# Paso de inducción 

Queremos demostrar que la igualdad también se cumple para k+1:
∣w^(k+1)∣=∣w^n∣+∣w^((k+1)-n)∣

Podemos expresar w^(k+1) de la siguiente manera:
w^(k+1)=w^k* w

* ∣w^(k+1)∣=∣w^k * w∣=∣w^k∣+∣w∣=∣w^n∣+∣w^(k−n)∣+1 //Usamos la definición de w^k 

* |w^(k+1)|=|w^n|+|w^(k-n)|+1 es igual a |w^n|+|w^((k+1)-n)|, ya que "k + 1 = (k-n)+1", así que, se demuestra que la igualdad se mantiene para k+1, y por lo tanto, la igualdad |w^(n+m)|=|w^n|+|w^m| se cumple para todos los npumero naturales n y m por unducción matemática

