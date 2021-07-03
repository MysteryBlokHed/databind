Calling
=======

Different ways to call a function.

``function`` command
--------------------

Built into mcfunctions. Requires a namespace.

``example/src/data/example/functions/main.databind``

.. code-block:: databind

   func example_func
   say Hello, World!
   end

   func main
   function example:example_func
   end

``call`` (infer namespace)
---------------------------

Add namespaces to functions while compiling.
Allows more freedom with directory names.

``example/src/data/example/functions/main.databind``

.. code-block:: databind

   func example_func
   say Hello, World!
   end

   func main
   call example_func
   end

Compiled, ``call example_func`` becomes ``function example:example_func``.

``call`` (explicit namespace)
------------------------------

``example/src/data/example/functions/main.databind``

.. code-block:: databind

   func example_func
       say Hello, World!
   end

   func main
       call example:example_func
   end

Effectively the same as the ``function`` command.
