Loop Until False
================

Use an integer as a boolean to loop until false.

Example
-------

``example/src/data/example/functions/main.databind``

.. code-block:: databind

   :func load
   :tag load
   :var bool .= 1
   :while :tvar bool matches 1
   tellraw @a "Bool is true"
   :endwhile
   :endfunc
   
Compiled
--------

When while loops are compiled, functions with random characters
at the end are created. In compiled examples, these characters
will be ``abcd``.

``example/out/data/example/functions/load.mcfunction``

.. code-block:: mcfunction

   scoreboard objectives add bool dummy
   scoreboard players set --databind bool 1
   function example:while_abcd

``example/out/data/example/functions/while_abcd.mcfunction``

.. code-block:: mcfunction

   execute if score --databind bool matches 1 run function example:condition_abcd

``example/out/data/example/functions/condition_abcd.mcfunction``

.. code-block:: mcfunction

   tellraw @a "Bool is true"
   function example:while_abcd
