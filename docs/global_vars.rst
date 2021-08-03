Global Vars
===========

You can define global variables with a file called ``vars.toml`` in the project root.
Keys and values aren't put in a section of the ``.toml``, they're just in the file. For example:

.. code-block:: toml

   name="World"

This defines a global variable ``name`` that can be used in your code.

Types
-----

The TOML format supports datatypes other than just strings, such as
booleans and integers. Types that aren't strings are converted to
strings. Booleans that are ``true`` are turned into ``1``, and
``false`` ones are turned into ``0``. Floats like ``1.0`` are
truncated, but floats with non-zero decimals are left alone.

Using Global Vars
-----------------

To use a global variable in your code, use an ``&`` symbol followed
by the variable name. Like this:

.. code-block:: databind

   say Hello, &name!

Which, with the ``vars.toml`` defined above, becomes:

.. code-block:: mcfunction

   say Hello, World!

Instances of ``&varname`` are directly replaced, meaning that
escaping them with a ``%`` symbol doesn't work. This means that
the following code:

.. code-block:: databind

   say Hello, %&name!

won't stop the replacement of ``&name``.

When to use
-----------

Global variables are useful to let users more easily configure aspects
of your datapack. This does mean that the project must be recompiled
whenever the configuration is changed, and that users must have Databind
downloaded to use the project. If you are only configuring number values,
eg. an amount of time to wait for something, then it might be easier for
people using your datapack to have a ``config.mcfunction`` file somewhere in the
project that sets scoreboard values.
