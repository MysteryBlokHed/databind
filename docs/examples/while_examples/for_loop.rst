For Loop
========

A for loop-like while loop.

Example
-------

``example/data/example/functions/load.databind``

.. code-block:: databind

   :var i .= 10

   :while :tvar i matches 1..
   tellraw @a "Variable i is above 0"
   :var i -= 1
   :endwhile
   tellraw @a "Variable i is at 0"

Transpiled
----------

When while loops are transpiled, functions with random characters
at the end are created. In transpiled examples, these characters
will be ``abcd``.

``example.databind/data/example/functions/load.mcfunction``

.. code-block:: mcfunction

   scoreboard objectives add i dummy
   scoreboard players set --databind i 10
   function example:while_abcd
   tellraw @a "Variable i is at 0"

``example.databind/data/example/functions/while_abcd.mcfunction``

.. code-block:: mcfunction

   execute if score --databind i matches 1.. run function example:condition_abcd

``example.databind/data/example/functions/condition_abcd.mcfunction``

.. code-block:: mcfunction

   tellraw @a "Variable i is above 0"
   scoreboard objectives remove --databind i 1
   function example:loop_abcd
