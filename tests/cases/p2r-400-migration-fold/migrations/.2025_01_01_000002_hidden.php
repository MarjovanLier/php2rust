A stand-in for a macOS resource fork. Laravel's glob skips a name that starts with a dot;
if the tool read this file, it would not be a migration, and the hasTable guard on codes
would then make that table unknown.
