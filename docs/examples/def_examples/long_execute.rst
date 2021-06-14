Long Execute Commands
=====================

Using definitions for a long execute command.

Example
-------

``def_example/data/loop/functions/main.databind``

.. code-block:: databind

   :def LONG_EXECUTE execute as @a[scores={custom_item_obj=1..},nbt={SelectedItem:{id:"minecraft:carrot_on_a_stick",tag:{custom_item:1b}}}] at @s

   :func tick
   :tag tick
   LONG_EXECUTE run summon lightning_bolt ^ ^ ^5
   LONG_EXECUTE run summon lightning_bolt ^ ^ ^-5
   LONG_EXECUTE run summon lightning_bolt ^5 ^ ^
   LONG_EXECUTE run summon lightning_bolt ^-5 ^ ^
   :endfunc

Transpiled
----------

``def_example.databind/data/loop/functions/load.mcfunction``

.. code-block:: mcfunction

   execute as @a[scores={custom_item_obj=1..},nbt={SelectedItem:{id:"minecraft:carrot_on_a_stick",tag:{custom_item:1b}}}] at @s run summon lightning_bolt ^ ^ ^5
   execute as @a[scores={custom_item_obj=1..},nbt={SelectedItem:{id:"minecraft:carrot_on_a_stick",tag:{custom_item:1b}}}] at @s run summon lightning_bolt ^ ^ ^-5
   execute as @a[scores={custom_item_obj=1..},nbt={SelectedItem:{id:"minecraft:carrot_on_a_stick",tag:{custom_item:1b}}}] at @s run summon lightning_bolt ^5 ^ ^
   execute as @a[scores={custom_item_obj=1..},nbt={SelectedItem:{id:"minecraft:carrot_on_a_stick",tag:{custom_item:1b}}}] at @s run summon lightning_bolt ^-5 ^ ^
