Todo en python es un objeto, ya sea una variable una función un string (cadena de caracteres)\. 
Si no sabes que es un objeto es la instancia de una clase (un molde para crear métodos o funciones)\.

Un ejemplo sería los planos de un carro, esa sería la clase llamada carro\. 
Y sus instancias serán los modelos que se crean en base a la clase carro\.

Pero no te preocupes por ese tema por ahora, solo llevate la idea de que en Python *todo es un objeto.*


```python
# creemos dos variables
edad = 10
nombre = "pepito"

# como se que tipo de objeto es?
# con la funcion interna llamada type()
# type() nos va retornar el tipo de objeto de una variable

type(edad) # <class 'int'>
type(nombre) # <class 'str'>

# Como ven nos retorno class "int" y class "str"
# eso quiere decir que la variable nombre es instancia del objeto "str"
# y que la variable edad es instancia de "int"

# No te preocupes si no entiendes por ahora, solo ten encuenta de que en python 
# “Todo es un objeto”



# Recalcando, las variables en python no son mas que objetos instanciados
# de una clase. Pero entonces como hace python para saber que es una variable?

# Hagamos el siguiente ejemplo 
# Escriban en si terminal de python las siguientes instrucciones

id(edad) 
# Esto va retornar el id que la clase Int nos da. En mi caso fue este 140564029669384
# ahora ingresa esta instruccion.

repr(edad) # Este es el representador que tiene mi instancia de la clase int
# "10"

# =====================
# edad
# =====================
# id: 140564029669384
#
# valor: 10
# ====================

# Esta seria un variable o instancia de la clase de int
# asi seria de manera representativa.
```
