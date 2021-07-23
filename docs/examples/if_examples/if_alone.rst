Single If Statement
===================

A lone if statement.

Example
-------

``example/src/data/example/functions/main.databind``

.. code-block:: databind

   func main
   tag load
       var test := 1
       runif tvar test matches 1
           say Test is equal to 1
       end
   end
