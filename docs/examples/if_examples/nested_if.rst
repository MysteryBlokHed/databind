Nested If Statements
====================

Multiple if statements inside of each other.

``example/src/data/example/functions/main.databind``

.. code-block:: databind

   func main
   tag load
       var i := 0
       var j := 0
       runif tvar i matches 0
           runif tvar j matches 0
               say i is 0 and j is 0
           else
               say i is 0 and j is not
           end
       end
   end
