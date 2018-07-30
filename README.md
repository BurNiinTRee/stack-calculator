# Stack-calculator
This provides a stack based highly extensible virtual machine, a module containing basic arithmetic instructions,
and a text based frontend, acting as a simple stack-based calculator.


## The Virtual Machine
The vm itself provides two fundamental types: the Stack, and Tokens.

Tokens can be pushed onto the stack, and are then given the oportunity to modify the stack in case they are operators.

A third type called `ParserAggregator` is also provided; it is a convinience struct to convert strings into Tokens.

Different Tokens can be added to the machine by implementing the Value trait.
To be able to add it to a `ParserAggregator` instance, the `Parser` trait also needs to be
implemented on a potentially differnt type.
It requires a method returning a regular expression, which shall only match a string,
if this string can be parsed by the `Parser`, and a method which then actually parses the input and returns a Token.


## The Arithmetic Module
This module acts as a sort-of standard library. It provides various structs,
implementing `Parser` and `Value`, ready to be added to a `ParserAggregator`.
