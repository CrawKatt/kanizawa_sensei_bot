El bucle while se usa para ejecutar un bloque de código mientras una condición sea verdadera\.

Ejemplo en Rust:
```cs
using System;

class Program
{
    static void Main()
    {
        int x = 1;

        while (x < 1000)
        {
            x *= 2;

            if (x == 64)
            {
                continue;
            }

            Console.WriteLine($"x = {x}");
        }
    }
}
```