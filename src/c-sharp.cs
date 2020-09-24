/*
 * C#
 * Takes user input and says hello
 * To compile: mcs c-sharp.cs 
 */
using System;

class Hello{
	static void Main(){
		// prompt
		Console.Write("Enter your name: ");

		// get input
		string name = Console.ReadLine();

		// say hello
		Console.WriteLine("Hello " + name + "!");
	}
}

