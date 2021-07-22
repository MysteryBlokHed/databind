Macros
======

Macros in Databind are advanced functions that allow you to take arguments,
unlike traditional mcfunctions. All arguments must be surrounded
by double quotes (``"``). Here is a macro that says "Hello" to a name you pass:

.. code-block:: databind

   !def say_hello($name)
       say Hello, $name!
   !end

And here is how it would be called:

.. code-block:: databind

   ?say_hello("World")

The macro call above would become the following when compiled:

.. code-block:: mcfunction

   say Hello, World!

As you can see, the ``$name`` in the body of the macro was replaced
with the ``"World"`` string that was passed to it.

Macros that use Databind code
-----------------------------

Macros are able to use Databind code just like any other place in a
``.databind`` file. Here is a macro that creates a variable with a name
that is passed to it, then announces a message to all players:

.. code-block:: databind

   !def create_var($name)
       var $name := 5
       tellraw @a "A variable named $name was created."
   !end

Macros that call other macros
-----------------------------

Macros are also able to call other macros and pass arguments to them.

.. code-block:: databind

   !def macro_1($name)
      say Hello, $name!
   !end
   
   !def macro_2($name)
       # There is a % before 'call' here because 'call' is a Databind keyword
       # See the syntax table for info on escaping keywords
       say I am about to %call macro_1
       ?macro_1("$name")
   !end

Keep in mind that macro arguments must be surrounded by double quotes,
which is why ``macro_2``'s call of ``macro_1`` is ``"$name"`` instead of
just ``$name``.

Macros that define functions
----------------------------

Since macros can use any Databind code, this also means that they're able to
define functions. This makes it possible to create macros that set up a series
of functions to avoid copy + pasting code.

.. code-block:: databind

   !def create_toggle_function($funcname)
       # This appends '_load' to the end of the function name
       func $funcname_load
       tag load
           var $funcname_state := 0
           var $funcname_toggled := 0
       end

       # This appends '_on' to the end of the function name
       func $funcname_on
           say $funcname has been enabled
           var $funcname_state = 1
       end

       # This appends '_off' to the end of the function name
       func $funcname_off
           say $funcname has been disabled
           var $funcname_state = 0
       end

       # This appends '_toggle' to the end of the function name
       func $funcname_toggle
           say Toggling $funcname
           execute if tvar $funcname_state matches 1 run var $funcname_toggled = 1
           execute if tvar $funcname_state matches 1 unless tvar $funcname_toggled matches 0 run call $funcname_off
           execute if tvar $funcname_state matches 0 unless tvar $funcname_toggled matches 1 run call $funcname_on
           var $funcname_toggled = 0
       end
   !end

This entire macro creates four functions per call:

#. A function that loads when the datapack is loaded (``$funcname_load``)
#. A function that enables something (``$funcname_on``)
#. A function that disables something (``$funcname_off``)
#. A toggle function (calls ``$funcname_on`` when disabled and ``$funcname_off`` when enabled)

These functions can all be created by running the following line:

.. code-block:: databind

   ?create_toggle_function("my_function")

Of course, creating functions that only say "Enabled" or "Disabled" isn't
useful in most situations. What would be useful is to be able to pass commands
to run when the function is enabled, disabled, or toggled.

This is entirely possible using macros due to the fact that the arguments
passed can be multiline.

If we change the macro above to look like this:

.. code-block:: databind

   !def create_toggle_function($funcname, $on_cmds, $off_cmds)
       # This appends '_load' to the end of the function name
       func $funcname_load
       tag load
           var $funcname_state := 0
           var $funcname_toggled := 0
       end

       # This appends '_on' to the end of the function name
       func $funcname_on
           var $funcname_state = 1
           $on_cmds
       end

       # This appends '_off' to the end of the function name
       func $funcname_off
           var $funcname_state = 0
           $off_cmds
       end

       # This appends '_toggle' to the end of the function name
       func $funcname_toggle
           execute if tvar $funcname_state matches 1 run var $funcname_toggled = 1
           execute if tvar $funcname_state matches 1 unless tvar $funcname_toggled matches 0 run call $funcname_off
           execute if tvar $funcname_state matches 0 unless tvar $funcname_toggled matches 1 run call $funcname_on
           var $funcname_toggled = 0
       end
   !end

We're now able to pass commands to run when the function is enabled
and disabled. If we wanted a command that summoned an armor
stand when enabled and killed it when disabled, we could call the
macro like this:

.. code-block:: databind

   # This formatting is not required, it's just to make the code
   # easier to read
   ?create_toggle_function(
       "astand",

       "summon armor_stand ~ ~ ~
        say Created armor stand",

       "kill @e[type=armor_stand]
        say Killed armor stand",
   )

When compiled to a datapack, if we wanted to run our toggle function
in-game, we could run the following:

``/function namespace:astand_toggle``
