Create, Modify & Test
=====================

Example
-------

.. code-block:: databind

   # Create a variable called example and set it to 2
   :var example .= 2
   # Add 1 to example
   :var example += 1
   # Subtract 2 from example
   :var example -= 2
   # Set example to 1
   :var example = 1
   # Say something if example is 1
   execute if :tvar example matches 1 run say Variable example is equal to 1!

Transpiled
----------

.. code-block:: mcfunction

   scoreboard objectives add example dummy
   scoreboard players set --databind example 2
   scoreboard players add --databind example 1
   scoreboard players remove --databind example 2
   scoreboard players set --databind example 1
   execute if score --databind example matches 1 run say Variable example is equal to 1!
