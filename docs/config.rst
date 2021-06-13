Databind Configuration
======================

Configuration file
------------------

Databind can be configured via a ``databind.toml`` file in the same
directory as the binary is being run in. A config file can also
be passed with the `-c` or `--config` option.

**Note: This table represents the default values of the options if no CLI arguments are passed.**

+--------------------------------------------+---------------------------------------------------------------------------------------+
| Option                                     | Effect                                                                                |
+============================================+=======================================================================================+
| ``random_var_names = false``               | (Not well-supported) Whether to randomly add characters to the end of variable names  |
+--------------------------------------------+---------------------------------------------------------------------------------------+
| ``var_display_names = false``              | Whether to update scoreboard display names for randomized variables                   |
+--------------------------------------------+---------------------------------------------------------------------------------------+
| ``to_transpile = ["**/*.databind"]``       | Specify what files to transpile using globs                                           |
+--------------------------------------------+---------------------------------------------------------------------------------------+
| ``output = String``                        | The output file or folder. If unspecified, creates new folder ending in ``.databind`` |
+--------------------------------------------+---------------------------------------------------------------------------------------+

CLI arguments
-------------

Most options that can be set in the ``databind.toml`` file can
also be set using CLI arguments. The CLI arguments use dashes
instead of underscores (eg. ``--generate-func-tags`` instead
of ``generate_func_tags``) and may have different names or
shorthand.

Example use:

``databind -c config.toml -o ./out ./datapack``