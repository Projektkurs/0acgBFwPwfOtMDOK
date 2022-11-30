Steps to install:

- install cargo with ```curl https://sh.rustup.rs -sSf | sh```
- install flutter 
- install bcm2835 (http://www.airspayce.com/mikem/bcm2835/)
- init submodules with ```it submodule init; git submodule update```

the users name is hardcoded as being "pk"

to run the crate, use ```./run.sh```
if you are runing the app for the first time, you also need to build the flutter application.
this can be done with ```./run.sh buildflutter```. It starts the programm afterwards

the default ip of the application is 192.168.0.201:8000. This can be changed in webserver/Rocket.toml
 
debuging: rm .Xauthority
