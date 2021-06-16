Databind CLI
============

What can be transpiled
----------------------

Either single mcfunction files or entire datapack folders can be transpiled.
When transpiling an entire folder, Databind will look for ``.databind`` files and
leave other files alone. Passing databind folder is required for using ``:func``.

Note that the namespace inference used for ``:func`` assumes a typical datapack
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

From an installation
^^^^^^^^^^^^^^^^^^^^

To transpile a single file, run ``databind file.databind``. A file called
``databind-out.mcfunction`` will be generated. To transpile a datapack folder,
run ``databind path/to/datapack``.  

With ``cargo run``
^^^^^^^^^^^^^^^^^^

After building Databind yourself, you can use ``cargo run`` to run it. Everything
works almost the exact same. You just need to add two dashes (``--``) after ``run``
(eg. ``cargo run -- file.databind`` or ``cargo run -- --help``).

More information is available from the CLI help menu (``databind --help``).
