Global Vars
===========

You can define global variables with a file called ``vars.ini``
in the project root. Keys and values aren't put in a section of the ``.ini``,
they're just in the root. For example:

.. code-block:: ini

   name=World

This defines a global variable ``name`` that can be used in your code.

Using Global Vars
-----------------

To use a global variable in your code, use an ``&`` symbol followed
by the variable name. Like this:

.. code-block:: databind

   say Hello, &name!

Which, with the ``vars.ini`` defined above, becomes:

.. code-block:: mcfunction

   say Hello, World!

Instances of ``&varname`` are directly replaced, meaning that
escaping them with a ``%`` symbol doesn't work. This means that
the following code:

.. code-block:: databind

   say Hello, %&name!

Just becomes:

.. code-block:: databind

   say Hello, %World!

``%World`` is effectively the same as ``World``, so the ``%`` symbol
won't appear in the compiled output.

When to use
-----------

Global variables are useful to let users more easily configure aspects
of your datapack. This does mean that the project must be recompiled
whenever the configuration is changed, and that users must have Databind
downloaded to use the project. If you are only configuring number values,
eg. an amount of time to wait for something, then it might be easier for
users of your datapack to have a ``config.mcfunction`` file somewhere in the
project that sets scoreboard values.
