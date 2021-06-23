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
| ``random_var_names = false``          | (Not well-supported) Whether to randomly add characters             |
|                                       | to the end of variable names                                        |
+---------------------------------------+---------------------------------------------------------------------+
| ``var_display_names = false``         | Whether to update scoreboard display names for randomized variables |
+---------------------------------------+---------------------------------------------------------------------+
| ``inclusions = ["**/*.databind"]``    | Specify what files to transpile using globs                         |
+---------------------------------------+---------------------------------------------------------------------+
| ``exclusions = []``                   | Specify what files not to copy over/transpile using globs           |
+---------------------------------------+---------------------------------------------------------------------+
| ``output = "out"``                    | The output file or folder                                           |
+---------------------------------------+---------------------------------------------------------------------+

Example Config
--------------

Below is a configuration file with all of the above settings.

.. code-block:: toml

   random_var_names = false
   var_display_names = false
   inclusions = ["**/*.databind"]
   exclusions = []
   output = "out"

CLI Arguments
-------------

Most options that can be set in the ``databind.toml`` file
can also be set using CLI arguments. The CLI arguments use dashes
instead of underscores (eg. ``--random-var-names`` instead
of ``random_var_names``) and may have different names or shorthand.

Example use:

``databind -c config.toml -o ./target ./datapack``
