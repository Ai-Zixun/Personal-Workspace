# The Evolution of Reddit.com's Architecture

* Link: https://www.youtube.com/watch?v=nUcO7n4hek4


## Components

TODO: Plantuml--000

CDN
* Send requests to distinct stacks depending on domain, path, cookies, etc.

R2: The monolith
* Written in Python
* The oldest single component of Reddit

Frontend:
* Written in Node js
* Modern frontends using shard server/client code


New Backend Services:
* Written in Python
* Splitting off from r2
* Common library/framework to standardize
* Thrift or HTTP depending on clients
* List of the services:
  * API
  * Search
  * Things
  * Listing
  * Rec

### r2 Deep Dive

r2 - Cassandra
r2 - APP
r2 - Things
r2 - Cassandra

### Listings

The foundation of Reddit: an ordered list of links

Cached Results:
* Rather than querying every time, Reddit cache the list of Links IDS
* Run the query and cache the results
* Invalidate cache on new submissions and votes

Cache
* Served from memcached
* Persisted to Cassandra

Jobs - Vote Queue

Mutate in Place
* running the query is too slow for how quickly things changes
* add sort info to cache and modify the cached result in place
* Locking is required

### Vote queue pileups

Scale out?
* Add more processors? -> made it worse
* Observability? unable to dig into the issue at higher granularity
* Root cause? -> lock
* More processors are wait on the lock

Solution?
* Partition the job queue based on the subreddit
* Fewer processor waiting for the same lock

### Outliers

Split up processing

Old:
* Job queue

New:
* Level 1:
  * Anti-cheating queue
* Level 2:
  * Subreddit queries
  * Domain queries
  * Profile queries

## Learing:
* Timers to give you a cross section
* P99 to show the problem root cause
* Having a way to dig into the root cause
* Locks are bad for throughput
  * If you must use a lock, use the right partitioning to reduce contention

## Things




Auto Scale
* Stateful vs Stateless
  * They need to be handled differently when dealing with autoscaling
