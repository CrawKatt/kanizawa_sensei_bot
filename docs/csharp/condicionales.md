Son grupos de sentencias o sentencias individuales que te permiten condicionar la decisión entre la elección de una opción y otra\.

Ejemplo en C\#:
```cs
using System;

class Program
{
    public static void Main(string[] args)
    {
        string color = "Verde";
    	if (color == "Verde") 
    	{
            Console.WriteLine("Puede continuar.");
    	}
    	else
    	{
            Console.WriteLine("El color no es verde");
    	}
    }
}
```