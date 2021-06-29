Scoreboard Operations
=====================

Example
-------

Define two objectives and use a scoreboard operation to multiply the first.

.. code-block:: databind

   obj obj1
   obj obj2
   sobj @a obj1 = 5
   sobj @a obj2 = 2
   sbop @a obj1 *= @a obj2

Compiled
--------

.. code-block:: mcfunction

   scoreboard objectives add obj1 dummy
   scoreboard objectives add obj2 dummy
   scoreboard players set @a obj1 5
   scoreboard players set @a obj2 2
   scoreboard players operation @a obj1 *= @a obj2
