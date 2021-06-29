Scoreboard Operations
=====================

Example
-------

Define two variables and use a scoreboard operation to multiply the first.

.. code-block:: databind

   var variable1 := 5
   var variable2 := 2
   sbop gvar variable1 *= gvar variable2

Compiled
--------

.. code-block:: mcfunction

   scoreboard objectives add variable1 dummy
   scoreboard players set --databind variable1 5
   scoreboard objectives add variable2 dummy
   scoreboard players set --databind variable2 2
   scoreboard players operation --databind variable1 *= --databind variable2
