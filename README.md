This crate is intended to provide core logic algorithms for Kalaxia V2.
It is used by both server and client side in order to synchronize builds.
Server uses this crate for actual logic, and client uses it for predicting
behaviors in order to not ask everytime the server to compute something not
critical.
