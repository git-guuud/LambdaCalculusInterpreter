## A simple lambda calculus interpreter written in rust

### Note:
This interpreter does NOT perform α-reductions so try not to reuse variable names


side note: The graph sadly breaks for even slightly large input, as the graph library used is not designed for large trees. ;(
# How to use?
Enter a lambda calculus expression in the left text area, and click "parse".  
The expression will be parsed and displayed as a tree on the right side.  
You can then click "β-reduce" to reduce the expression step by step.  
Or click "solve" to reduce the expression to its normal form.

## What is lambda calculus?

Lambda calculus is a model of computation that uses function abstraction and application, It has the following rules:
1. (function definition) λx.M is a function which takes an argument x and returns M.
2. (β-reduction) f y applies the function f to y replacing all occurences of x in M with y.
That's it!!

Surprisingly, Just these two rules are enough to express any computable function.

One can begin to use abstraction to define usual functions.  
For example, if you define the natural numbers as follows (church numerals):  
0 = λf.λx.x  
1 = λf.λx.f x  
2 = λf.λx.f (f x)  
3 = λf.λx.f (f (f x))  
...  
Then you can define addition as: add = λm.λn.λf.λx.m f (n f x)  
multiplication as: mult = λm.λn.λf.m (n f)  
exponentiation as: exp = λm.λn.n m  

Similarly, if you define booleans as:  
true = λx.λy.x  
false = λx.λy.y  
Then you can define logical operations as:  
and = λp.λq.((p q) p)  
or = λp.λq.((a b) a)  
not = λp.((p false) true)  
ifthenelse = λp.λa.λb.p a b  

You can also emulate data structures like lists, pairs, and trees.  
For example, a pair can be defined as:  
pair = λx.λy.λf.f x y  
first = λp.p (λx.λy.x)  
second = λp.p (λx.λy.y)  

Anything computable can be expressed in lambda calculus!

