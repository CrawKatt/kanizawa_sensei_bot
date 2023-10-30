Un Enum es un tipo que almacena diferentes variantes, almacena diferentes opciones\.

Ejemplo en C\#:
```cs
using System;

enum Season
{
    Spring,
    Summer,
    Autumn,
    Winter
}

class Program
{
    public static void Main(string[] args)
    {
        Season currentSeason = Season.Summer;
        Console.WriteLine("La temporada actual es: " + currentSeason);
    }
}
```