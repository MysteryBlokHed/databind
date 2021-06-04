Simple Function
===============

Example
-------

A function that increments a counter and logs when it's run.

.. code-block:: databind

   :var counter .= 0
   :func example
   tellraw @a "Example_function run"
   :var counter += 1
   :endfunc

Transpiled
----------

``example.databind/data/example/functions/load.mcfunction``

.. code-block:: mcfunction

   scoreboard objectives add counter dummy
   scoreboard players set --databind counter 0

``example.databind/data/example/functions/example.mcfunction``

.. code-block:: mcfunction

   tellraw @a "Example_function run"
   scoreboard players add --databind counter 1
