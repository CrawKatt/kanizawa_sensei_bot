Los métodos son similares a las funciones, pero se diferencian en que los métodos se definen dentro de un contexto, como una estructura o un Enum\.

Ejemplo en Rust:
```cs
using System;

class Rectangulo
{
    public uint Ancho { get; set; }
    public uint Alto { get; set; }

    public uint CalcularArea()
    {
        return Ancho * Alto;
    }
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

        Console.WriteLine("El área del rectángulo es: " + rectangulo.CalcularArea());
    }
}
```