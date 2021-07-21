Folder Structure
================

How the folder structure of Databind works.

In a project started with ``databind create``, the file structure
might look something like this:

.. code-block:: text

   project_root
   │  databind.toml
   │  LICENSE
   │  README.md
   └──src
      │   pack.mcmeta
      │   pack.png
      └───data
          └───namespace
              └───functions
                      main.databind

All of the Databind-related files (other than the configuration file)
are contained in the `src/` directory. Other files such as the project's
license and the README are just in the root. These files are not generated
by default, but they've been added in the example to show where they might
be placed.

It's possible to create a project without using ``databind create``, but it's
not ideal and bugs caused by it generally won't be fixed.
