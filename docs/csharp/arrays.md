*Array:* En C\#, los arrays son estructuras de datos que almacenan un número fijo de elementos del mismo tipo\.

Ejemplo en C\#: 

```cs
using System;

class Program
{
    public static void Main(string[] args)
    {
        int[] array = new int[] {1, 2, 3, 4, 5};
        Console.WriteLine(array[0]);
    }
}
```

Consejo: En C\#, los Arrays son objetos y se rigen por la regla de los índices\. A cada elemento le corresponde un índice y los índices comienzan en cero\. 

Si tomamos nuestro ejemplo el índice en dicho ejemplo es:
```
0 -> 1 
1 -> 2 
2 -> 3 
3 -> 4 
4 -> 5
```