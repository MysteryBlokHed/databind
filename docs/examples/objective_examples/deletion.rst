Deletion
========

Example
-------

Define an objective and delete it.

.. code-block:: databind

   obj objective dummy
   delobj objective
   # or
   delvar objective

Compiled
--------

.. code-block:: mcfunction

   scoreboard objectives add objective dummy
   scoreboard objectives remove objective
