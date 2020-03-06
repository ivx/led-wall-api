# led-wall-api !!!!WIP!!!!
API to control LED-wall in the maki


## Status

#### Done:

- [x] Renderer which reads color from redis key `background:color:red`...
- [x] OCS API which set's redis keys when ocs calls are made

#### ToDo/Ideas:

Features:
- [ ] set "fusleiste" and other areas seperately
- [ ] set pixels explicitly
- [ ] draw shapes/animations???? (change brightness/size of shapes)

## Deployment

- Install rust on raspberry pi https://www.rust-lang.org/tools/install
- Install deps - `apt-get install libasound-dev libudev-dev libudev1 librust-libudev-sys-dev`
