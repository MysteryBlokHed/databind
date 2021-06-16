Databind CLI
============

What Can Be Transpiled
----------------------

Databind transpiles Databind projects (see :ref:`Creating a Project`).
Databind will look for included files (``**/*.databind`` by default) and
leave other files alone.

Note that the namespace inference used for ``:func`` assumes a proper
file structure (``<datapack>/data/<namespace>/functions`` for functions), but it
**does not check if this is the case.** A ``minecraft/tags/functions/`` folder may
be generated in an unexpected place if an invalid folder is passed.

Using the CLI
-------------

.. code-block:: text

   USAGE:
      databind.exe [FLAGS] [OPTIONS] <DATAPACK>
      databind.exe [FLAGS] [OPTIONS] <SUBCOMMAND>

   FLAGS:
      -h, --help                 Prints help information
         --ignore-config        Ignore the config file. Used for testing
         --random-var-names     Add characters to the end of variable names. Does not work when using variables across
                                 multiple files
         --var-display-names    Change the display name of variables in-game to hide extra characters. Only relevant with
                                 --random-var-names
      -V, --version              Prints version information

   OPTIONS:
      -c, --config <config>    Configuration for the transpiler
      -o, --out <output>       The output file or directory

   ARGS:
      <DATAPACK>    The datapack (or file) to transpile

   SUBCOMMANDS:
      create    Create a new project
      help      Prints this message or the help of the given subcommand(s)

From an Installation
^^^^^^^^^^^^^^^^^^^^

When installed, you can access the CLI by running ``databind`` in any command line.
Running ``databind --help`` will output the text above.

With ``cargo run``
^^^^^^^^^^^^^^^^^^

After building Databind yourself, you can use ``cargo run`` to run it. Everything
works almost the exact same. You just need to add two dashes (``--``) after ``run``
(eg. ``cargo run -- --help``).
