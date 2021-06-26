Databind Configuration
======================

Configuration File
------------------

Databind can be configured via the ``databind.toml`` generated
in the project's root. A config file can also be passed
with the ``-c`` or ``--config`` option.

This table represents the default values of the options
if no config changes are made.

+---------------------------------------+---------------------------------------------------------------------+
|                 Option                |                                Notes                                |
+=======================================+=====================================================================+
+---------------------------------------+---------------------------------------------------------------------+
| ``inclusions = ["**/*.databind"]``    | Specify what files to compile using globs                           |
+---------------------------------------+---------------------------------------------------------------------+
| ``exclusions = []``                   | Specify what files not to copy over/compile using globs             |
+---------------------------------------+---------------------------------------------------------------------+
| ``output = "out"``                    | The output file or folder                                           |
+---------------------------------------+---------------------------------------------------------------------+

Example Config
--------------

Below is a configuration file with all of the above settings.

.. code-block:: toml

   inclusions = ["**/*.databind"]
   exclusions = []
   output = "out"

CLI Arguments
-------------

Most options that can be set in the ``databind.toml`` file
can also be set using CLI arguments.

Example use:

``databind -c config.toml -o ./target ./datapack``
