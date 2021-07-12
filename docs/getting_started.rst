Getting Started
===============

Get started with Databind.

Installation
------------

Databind is installed using `cargo <https://www.rust-lang.org/tools/install>`_.
With cargo installed, run ``cargo install databind`` to get the latest version.
If Rust is in your path, then you should be able to access the CLI by running
``databind`` in any command line.

Creating a Project
------------------

To create a new project, use the ``databind create`` command.

.. code-block:: text

   USAGE:
   databind create [OPTIONS] <NAME>

   FLAGS:
   -h, --help       Prints help information
   -V, --version    Prints version information

   OPTIONS:
   --description <DESCRIPTION>    The pack description [default: A databind pack]
   --path <PATH>                  The path to create the pack in

   ARGS:
       <NAME>    The name of the project

Example use:

``databind create my_project`` to create a new project in a folder
called ``my_project``.

``databind create --description "My first project" my_project``
to create a new project with the description ``My first project``.

``databind create --path . my_project`` to create a new project
in the current directory. Only works if empty.

Writing Code
------------

Below is the default ``main.databind`` file. ``.databind`` files
**can only be used** to contain function definitions.

.. code-block:: databind

   func main
       tag load
       tellraw @a "Hello, World!"
   end

First, a function named main is defined. The name can be changed, it doesn't
have to be main. Then, it is tagged with ``load``. This tag is
normal to datapacks and means that a function will run when the datapack is
initially loaded. After that, an ordinary ``tellraw``, and then ``end``
to close the function definition.

When compiled, this will create a file called ``main.mcfunction`` that contains
the following:

.. code-block:: mcfunction

   tellraw @a "Hello, World!"

A ``load.json`` file will also be generated in ``minecraft/tags/functions``
to give the function a load tag.

Building
--------

To build your project, run ``databind`` in the root directory of your project.
Alternatively, you can run ``databind <PATH>`` where ``<PATH>`` is the path to
your project.

Additional Files
----------------

You are able to create as many ``.databind`` files and as many namespaces as
you'd like. You are also able to mix normal ``.mcfunction`` files with ``.databind``
files, meaning you don't have to have a project that only uses Databind. This
is helpful if you want to convert a normal datapack to a Databind project.
Databind files cannot contain anything other than function definitions, so something
such as this alone in a ``.databind`` file:

.. code-block:: databind

   say Hello, World!

Would not generate any output.

See Examples
------------

If you want to see some examples of language features, go to the :ref:`examples:Examples`.
Otherwise, you may continue to the next page.
