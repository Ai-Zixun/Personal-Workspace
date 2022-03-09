#

Related Videos:
* https://www.youtube.com/watch?v=eRgFNW4QFDc


# Key Ideas

1. Use Commodity Hardware
    * Commodity servers are cheap and good for horizontal scaling
    * Failures are common
        * Disk / Network / Server
        * OS
        * Human errors
    * The system need to design a way to deal with the high failure rate
2. Support large files
    * Multi-GB size files
    * Allow google to craw the web
    * Ideal for batch processing
3. File Operations
    * Read + Append only
        * No random writs
        * Mostly sequential reads
4. Chunks
    * Files are split into chunks
        * Each chunk of 64 mb
        * Identified by an unique 64 bits ID
        * Stored in the chunk servers
    * Which means:
        * A file might have several chunk
        * and the chunks are distributed in several chunk servers
5. Replicas
    * The chunks are replicated
        * by default 3 replicas
        * stored in different chunk servers
6. Master server (metadata server)
    * Stores the entire metadata of the system

## Read Operation

1. Client: Acquire the metadata of the files of the file from the metadata server
2. Client: Read the data from the chunk servers based on the metadata

## Write Operation

1. Client: Ask the metadata server for the location to write
2. Metadata Server: Return locations where the file should be written
    * 3 servers where the chunk will be stored
3. Client: Write to the closet replica server (Replica Server A) (write to the cache)
4. Replica Server (Closest): Write the file to the other two replica server (write to the cache)
5. Replica Server (Primary): once the primary metadata server knows that all server has the file in cache, it would instruct the servers to write the file into disk
6. Replica Server (Secondary): ack -> primary
7. Replica Server (Primary): ack -> client

## Heartbeats

* Healthy: Send hard beats message to master to allow the master to know that server is still alive
* Unhealthy: if the master does not receive the heartbeat, it would create a new copy for the chunks that is been stored in the unhealthy server.

## Master

* There is only ONE master server
* All operation are recorded in an append only log
    * The log would be used to recreate the states if the master crash
    * Checkpoints
        * background thread to compress the data that is behind the checkpoint

## Shadow Master

TODO