# `arts_server`

(And matchmaking/stats/meta-info)

## Launch Server

To launch a new server, navigate to the folder that contains your authentication server binary and then issue the following command changing the address and port to one of your choice:

`arts_server.exe --address 127.0.0.1 --http-port 2031 --ws-port 2032`

> Note: If you are not on Windows you will have to change the [.exe] file ending.

If you are running the server as part of compiling the project, use the following command below, changing the ip and port to one of your choice:

`cargo run -p arts_server -- --address 127.0.0.1 --http-port 2031 --ws-port 2032`
