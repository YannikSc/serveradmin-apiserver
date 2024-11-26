# serveradmin-apiserver

For everyone who uses serveradmin (so definitely a lot of people) this small project will make it
possible to create, edit and delete servers like serveradmin is Kubernetes.

This tool is limited by what Serveradmin can do, so no logs, no execs, or any other actual Kubernetes
functionallity. It only allows the creation and management of objects via the `kubectl` tool and possibly
via 3rd party tools like Helm (untested).
