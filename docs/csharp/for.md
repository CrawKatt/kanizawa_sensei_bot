En C\#, el ciclo for nos permitirá iterar sobre una colección de datos\. Ya sea un vector, un Arreglo/Array, una tupla, etc\.

El ciclo for funcionará como un for each

Ejemplo en C\#:
```cs
for (int i = 0; i < 3; i++)
{
    Console.Write(i);
}
```

Ejemplo de algoritmo Fizz Buzz utilizando el ciclo for en C\#:
```cs
using System;

class Program
{
    public static void Main(string[] args)
    {
		for (int numero = 1; numero <= 101; numero++)
		{
    		if (numero % 3 == 0 && numero % 5 == 0)
    		{
        		Console.WriteLine("Fizz Buzz");
    		}
    		else if (numero % 3 == 0)
    		{
        		Console.WriteLine("Fizz");
    		}
    		else if (numero % 5 == 0)
    		{
        		Console.WriteLine("Buzz");
    		}
    		else
    		{
        		Console.WriteLine(numero);
    		}
	    }
    }
}
```