Los generics nos permiten crear tipos de datos genéricos, que pueden ser de cualquier tipo\.

Ejemplo en C\#:
```cs
using System;

class Generic<T>
{
    public T Valor { get; set; }

    public Generic(T valor)
    {
        Valor = valor;
    }
}

class Program
{
    static void Main()
    {
        Generic<int> entero = new Generic<int>(5);
        Generic<double> flotante = new Generic<double>(5.0);

        Console.WriteLine("Valor de 'entero': " + entero.Valor);
        Console.WriteLine("Valor de 'flotante': " + flotante.Valor);
    }
}
```