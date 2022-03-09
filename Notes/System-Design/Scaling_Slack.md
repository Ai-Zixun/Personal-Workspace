# Scaling Slack - The Good, the Unexpected, and the Road Ahead

* Link: https://www.youtube.com/watch?v=_M-oHxknfnI

## Old Architecture

All user of the same workspace would use the same:
* Database shard (a MySQL database)
* Message server shard

MySQL Database
* Master-Master replication
* Availability over consistency

Client Experience
* Everything feels realtime (FAST)
* Full data model available in memory
* Updates appears instantly

## What has changed as Slack grow

* From only having **workspace** to have **enterprise** which can have multiple workspace
* Added shard channel where two enterprise can share a workspace

## Issue 1:

**ISSUE**
* Old architecture:
    * when a user connect to the workspace, it would download the entire metadata of the workspace from the message server to local memory
    * As the size of the workspace increase, the size of the metadata increase.

**SOLUTION**
* Lazy-load
    * Return an url from the server so the customer client can callback later to acquire the data
* Flannel cache
    * TODO:

## Issue 2:

**Issue**
* Sharding: Some of the database shard with large workspace would have a lot of traffic which introduce hot partition
* Old architecture:
    * the MySQL database is been sharded based on workspace
    * Which works well for small organization / workspace.
    * However, when working with large organization, some of the database would encounter bottleneck.

**SOLUTION**
* Use vitness
    * Link: https://vitess.io/
    * a sharding mid-ware that provides find-grained sharding

## Issue 3: