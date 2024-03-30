# ENVINI

Allows to declarative define environment variables that are mapped to ini file values.


Idea:
Create a static mapping file, that maps simple env names to ini value pointers:

Static config file example, obviously a ini file :)

```ini
[KF2_WEB_ADMIN_PASSWORD]
ini_file = ~/Downloads/test.ini
section = Engine.AccessControl
key = AdminPassword
```

Or another example:

```ini
[KF2_ENCODING]
ini_file = ~/Downloads/test.ini
section =
key = UTF-8
```
