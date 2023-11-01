# Chapter 1

## Overview

## Project Organization

The project is separated into four *crates*.

- arts_authentication
- arts_client
- arts_core
- arts_server

*Arts_authentication* is the single server responsible for maintaining user credentials and authenticating and loging users in. There is only one authentication server hosted by **NoahS**.

*Arts_client* is the official cross-platform client. It exports to all the officially supported platforms from this single binary.

*Arts_core* is the core library for base definitions shared by all the other crates. Most things should be defined here unless they are specifially and exclusively user facing - eg a UI component. Those would be defined in Arts_client.

*Arts_server* is the official backend server for hosting, running, and simulating games and connecting clients, processing client commands, sending push notifications, etc. A single binary can run a multitude of games and handle all of them.
