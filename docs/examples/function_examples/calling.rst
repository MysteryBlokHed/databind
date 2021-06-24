Calling
=======

Different ways to call a function.

``function`` command
--------------------

Built into mcfunctions. Requires a namespace.

``example/src/data/example/functions/main.databind``

.. code-block:: databind

   :func example_func
   say Hello, World!
   :endfunc

   function example:example_func

``:call`` (infer namespace)
---------------------------

Add namespaces to functions while compiling.
Allows more freedom with directory names.

``example/src/data/example/functions/main.databind``

.. code-block:: databind

   :func example_func
   say Hello, World!
   :endfunc

   :call example_func

Compiled, ``:call example_func`` becomes ``function example:example_func``.

``:call`` (explicit namespace)

``example/src/data/example/functions/main.databind``

.. code-block:: databind

   :func example_func
   say Hello, World!
   :endfunc

   :call example:example_func

Effectively the same as the ``function`` command.
