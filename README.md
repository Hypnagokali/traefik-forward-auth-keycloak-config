# Example configuration: Traefik, oauth2-proxy, Keycloak
This is an example configuration to handle authentication at the proxy level (Traefik), so that all apps share the same base session.

The apps receive the ID token, which allows them to read the user's roles and other claims. When a user logs out from one app, the shared session is destroyed (not yet implemented), and none of the apps can be accessed anymore.
