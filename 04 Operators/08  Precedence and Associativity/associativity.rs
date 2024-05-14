// Associativity
/*
If two or more operators of the same precedence appear in a statement, then which operator will be evaluated first is defined by the associativity.
*/

/*
Left associativity occurs when an expression is evaluated from left to right. An expression such as a ~ b ~ c, in this case, would be interpreted as (a ~ b) ~ c where ~ can be any operator.

The operators below can be chained as left associative.

as
*, /, %
+, -
<< >>
&
^
|
&&
||
The comparison, assignment, and the range operator cannot be chained at all.
*/