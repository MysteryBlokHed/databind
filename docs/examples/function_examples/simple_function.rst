Simple Function
===============

Example
-------

A function that increments a counter and logs when it's run.

``example/src/data/example/functions/main.databind``

.. code-block:: databind

   :func load
   :tag load
   :var counter .= 0
   :endfunc
   
   :func example
   tellraw @a "Example_function run"
   :var counter += 1
   :endfunc

Transpiled
----------

``example/out/data/example/functions/load.mcfunction``

.. code-block:: mcfunction

   scoreboard objectives add counter dummy
   scoreboard players set --databind counter 0

``example/out/data/example/functions/example.mcfunction``

.. code-block:: mcfunction

   tellraw @a "Example_function run"
   scoreboard players add --databind counter 1
