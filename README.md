# s3-time-machine

Experimental FUSE filesystem to implement a S3-backend for Apple's Time Machine
backup application.

## Concept

* Create a sparsebundle disk image on a FUSE filesystem
* Time Machine mounts the disk image as a HFS+ filesystem
* FUSE filesystem intercepts reads and writes of HFS+ btrees
* Local reads and writes mapped to S3 operations
