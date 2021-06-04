Loop
====

Functions that loop until a counter reaches 0.

Example
-------

``loop_example/data/loop/functions/load.databind``

.. code-block:: databind

   :var counter .= 5

   :func loop_main
   execute if :tvar counter matches ..0 run tellraw @a "Counter has reached 0"
   execute if :tvar counter matches 1.. run :call loop_above_0
   :endfunc

   :func loop_above_0
   tellraw @a "Counter is 1 or higher"
   :var counter -= 1
   :call loop_main
   :endfunc

Transpiled
----------

``loop_example.databind/data/loop/functions/load.mcfunction``

.. code-block:: databind

   scoreboard objectives add counter dummy
   scoreboard players set --databind counter 5

``loop_example.databind/data/loop/functions/main.mcfunction``

.. code-block:: mcfunction

   execute if score --databind counter matches ..0 run tellraw @a "Counter has reached 0"
   execute if score --databind counter matches 1.. run function loop:counter_above

``loop_example.databind/data/loop/functions/counter_above.mcfunction``

.. code-block:: mcfunction

   tellraw @a "Counter is 1 or higher"
   scoreboard players remove --databind counter 1
   function loop:main