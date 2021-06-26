Deletion
========

Example
-------

Define a variable and delete it.

.. code-block:: databind

   :var variable .= 1
   :delvar variable
   # or
   :delobj variable

Compiled
--------

.. code-block:: mcfunction

   scoreboard objectives add variable dummy
   scoreboard players set --databind variable 5
   scoreboard objectives remove variable
