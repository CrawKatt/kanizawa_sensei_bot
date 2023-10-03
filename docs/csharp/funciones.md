Las funciones son bloques de código que se pueden reutilizar en diferentes partes de nuestro programa\.

Ejemplo en C\#:
```cs
using System;

class Program
{
    static void Saludar(string nombre)
    {
        Console.WriteLine("Hola " + nombre);
    }

    static void Main()
    {
        Saludar("Juan");
    }
}
```