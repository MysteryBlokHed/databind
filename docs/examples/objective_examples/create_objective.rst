Create Objective
================

Create a scoreboard objective.

Example
-------

.. code-block:: databind

   # Create an objective points and set everyone's score to 100
   :obj points dummy
   :sobj @a points = 100

Compiled
--------

.. code-block:: mcfunction

   scoreboard objectives add points dummy
   scoreboard players set @a points 100
