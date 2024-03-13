## A simple p2p gossiping application

task description:
https://hackmd.io/@r3XngjBBSumx2rU-hKU7Qg/BkbHS80cv

***How to use:***
```
./gossip --help

a simple p2p gossiping application

Usage: gossip [OPTIONS] --period <PERIOD> --port <PORT>

Options:
--period <PERIOD>    period for broadcasting messages in seconds
--port <PORT>        port of this service
-c, --connect <CONNECT>  address to connect, format: 127.0.0.1:8080
-h, --help               Print help
-V, --version            Print version
```


run in various terminal:

`./gossip --port 8080 --period 5`

`./gossip --port 8081 --period 10 --connect 127.0.0.1:8080`

`./gossip --port 8081 --period 15 --connect 127.0.0.1:8080`

***Shutdown:***  
`Ctrl+C`


***Output:***
```
2024-03-13 19:11:55.882210802 [INFO] Start service: local address 127.0.0.1:8080, period 5 sec
2024-03-13 19:12:00.941198279 [INFO] Received message from 127.0.0.1:8081
2024-03-13 19:12:05.884052591 [INFO] Sending message to {127.0.0.1:8081}
2024-03-13 19:12:09.693349227 [INFO] Received message from 127.0.0.1:8082
2024-03-13 19:12:10.884365622 [INFO] Sending message to {127.0.0.1:8082, 127.0.0.1:8081}
2024-03-13 19:12:10.941274967 [INFO] Received message from 127.0.0.1:8081
2024-03-13 19:12:15.884232031 [INFO] Sending message to {127.0.0.1:8082, 127.0.0.1:8081}
2024-03-13 19:12:20.884826584 [INFO] Sending message to {127.0.0.1:8081, 127.0.0.1:8082}
2024-03-13 19:12:20.94151248 [INFO] Received message from 127.0.0.1:8081
2024-03-13 19:12:24.694299309 [INFO] Received message from 127.0.0.1:8082
2024-03-13 19:12:25.884857309 [INFO] Sending message to {127.0.0.1:8082, 127.0.0.1:8081}
2024-03-13 19:12:30.884595508 [INFO] Sending message to {127.0.0.1:8081, 127.0.0.1:8082}
2024-03-13 19:12:30.941299759 [INFO] Received message from 127.0.0.1:8081
2024-03-13 19:12:35.884568276 [INFO] Sending message to {127.0.0.1:8082, 127.0.0.1:8081}
2024-03-13 19:12:39.694126597 [INFO] Received message from 127.0.0.1:8082
2024-03-13 19:12:40.884724915 [INFO] Sending message to {127.0.0.1:8082, 127.0.0.1:8081}
2024-03-13 19:12:40.941340288 [INFO] Received message from 127.0.0.1:8081
^C2024-03-13 19:12:42.593661634 [INFO] Shutdown..
2024-03-13 19:12:42.59384508 [INFO] Shutdown listener
2024-03-13 19:12:42.593870591 [INFO] Shutdown sender
```