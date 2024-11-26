# serveradmin-apiserver

For everyone who uses serveradmin (so definitely a lot of people) this small project will make it
possible to create, edit and delete servers like serveradmin is Kubernetes.

This tool is limited by what Serveradmin can do, so no logs, no execs, or any other actual Kubernetes
functionallity. It only allows the creation and management of objects via the `kubectl` tool and possibly
via 3rd party tools like Helm (untested).

## Usage

A simple `cargo run --bin serveradmin-apiserver` is enough to get the server started. It will use the usual
adminapi environment variables to figure out which serveradmin instance should be used (`SERVERADMIN_BASE_URL`).

The authentication works by setting the `User.token` in the kubeconfig (see [example.kubeconfig] for a basic
kubeconfig) to the user's Serveradmin token.
It will be passed to the adminapi so that all users can use the same apiserver with their own tokens and
without impersonation through some application account. That speaking the SSH authorization of severadmin
is not supported.

**NOTE:** The server has to be reachable via HTTPS, as otherwise `kubectl` will not pass the `User.token` in
the `Authorization` header to the server. To listen directly with SSL enabled some configuration through
environment variables has to be done:

- `HTTP_LISTEN_SSL=ON`
- `HTTP_LISTEN_SSL_CERT=$PATH_TO_PEM_CERT` (defaults to `./server-cert.pem`)
- `HTTP_LISTEN_SSL_KEY=$PATH_TO_PEM_KEY` (defaults to `./server-key.pem`)
