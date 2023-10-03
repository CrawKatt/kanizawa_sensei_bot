Los structs son tipos de datos personalizados que nos permiten agrupar diferentes valores en un solo tipo\.

Ejemplo en Rust:

```cs
using System;

struct Rectangulo
{
    public uint Ancho { get; set; }
    public uint Alto { get; set; }
}

class Program
{
    static void Main()
    {
        Rectangulo rectangulo = new Rectangulo
        {
            Ancho = 30,
            Alto = 50
        };
		
        uint resultado = rectangulo.Alto * rectangulo.Ancho;

        Console.WriteLine($"El área del rectángulo es: {resultado}");
    }
}
```