Databind Configuration
======================

Configuration File
------------------

Databind can be configured via a ``databind.toml`` file in the same
directory as the binary is being run in. A config file can also
be passed with the ``-c`` or ``--config`` option.

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
|                                       | Functions to transpile but not to generate an ``.mcfunction``       |
| ``function_out_exclusions             | file for. Will only be used if it is also included in               |
| = ["**/main.databind"]``              | the inclusions globs. Useful for files that only contain functions. |
|                                       | it will be transpiled without its own ``.mcfunction``               |
+---------------------------------------+---------------------------------------------------------------------+
| ``exclusions = []``                   | Specify what files not to copy over/transpile using globs           |
+---------------------------------------+---------------------------------------------------------------------+
|                                       | The output file or folder. If unspecified,                          |
| ``output = String``                   | creates new folder ending in ``.databind`` or a file called         |
|                                       | ``databind-out.mcfunction``                                         |
+---------------------------------------+---------------------------------------------------------------------+

Example Config
--------------

Below is a configuration file with all of the above settings.

.. code-block:: toml

   random_var_names = false
   var_display_names = false
   inclusions = ["**/*.databind"]
   function_out_exclusions = ["**/main.databind"]
   exclusions = []

(The output option is omitted to use the default folder of ``project_name.databind``)

CLI Arguments
-------------

Most options that can be set in the ``databind.toml`` file can
also be set using CLI arguments. The CLI arguments use dashes
instead of underscores (eg. ``--generate-func-tags`` instead
of ``generate_func_tags``) and may have different names or
shorthand.

Example use:

``databind -c config.toml -o ./out ./datapack``
