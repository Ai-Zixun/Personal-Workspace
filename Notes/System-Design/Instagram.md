# Scaling Instagram Infrastructure

https://www.youtube.com/watch?v=hnpzNAPiC0E

## Background

Daily Traffic (at 2017)
* 400 million users
* 4 billion likes
* 100 million photo/video uploads
* top account: 110 million followers

## scaling

Scale Out
* Able to add server when needed

Scale Up
* Able to utilize the resource on hands

Scale dev team
* Enable the dev team to make changes without been constrained

## Disaster recovery

* Storage
  * Needs to be consistent across data centers
* Computing
  * Driven by user traffic, as needed basis

## Scale Out - Storage

### PostgreSQL - User, Media, Friendship, etc.

Master
* Serving write traffic
Replica
* Serving Read traffic
* User read from the nearest server

Master and replicas are in different data centers

### Cassandra - User feeds, activity, etc.

Multiple replicas (no master)
replicas in different data centers

### Computing

Async tasks done in the same region with the main server

### Memcached
* High performace key-value cache in memory
* Millions of reads/writes per seconds
* Sensitive to network condition
* Cross region operation is prohibitive

No global consistency => Caching inconsistency issue across the database

=> Solution
=> PostgreSQL has a deamon process to invalide the memcached entry when a new entry comes in
=> higher load on the PostgreSQL

=> Solution
=> Use Memcache Lease

### Memcache lease

* user A request for a resource fro memcache
* cache miss, memcached grant user A the right to "FILL" the data
* user B request for the same resource
* cache miss, memcached asks user B to wait/stale
* user A read from DB
* user A lease-set the memecached
* user B get the data from memcached


## Scale Up - Use as few as server as possible

CPU
* Monitor
  * Change to the CPU with feature changes
* Analyze
  * Python CProfile
* Optimize
  * Clean up dead code
  * Move memory from private region to the shard memory


## Scaling Team

What does Instagram wants:
* Automatically handle cache
* Define relations, not worry about implementation
* Self service by product engineers
* Infra focuses on scale