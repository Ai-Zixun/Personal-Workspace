# System Design Interview: An Insider's Guide

System Design Interview
* simulate in-team design process
* solve an ambiguous problem
* work with teammates/co-workers
* two co-workers working together

Want:
* ability to collaborate with teammates
* work under pressure
* work with ambiguous problem
* constructive problem solving skills

Not Want:
* over-engineering solution
* stubbornness
* do not answer without thinking, or without understanding the question's scope, requirements, backgrounds, etc.

Approaches
1. Comprehend the requirement
2. Propose high-level designs
3. Detailed designs
4. Discussions


Stage 1: Comprehend the requirement
* what does the customer/end-user wants?
  * what product feature is the customer looking for?
  * what problem is the customer trying to solve?
* what is the scale of the system?
  * how many users?
  * what is the avg/p100 qps
  * when does the customer need to scale 10x/100x
* understand the technology the team is using
  * AWS? GCP? Azure?
  * Java? Rust? Python?
  * GraphQL? gRPC? REST?

Stage 2: Propose high-level designs
* propose design for different scales
  * get the interviewers involved
* make some estimation on the traffic/scale
  * dive deep only if the interviewer is asking for deeper analysis
* design the api schema / database schema if it is suited to the question

Stage 3: Detailed design

Stage 4: Discussions
* limitation, bottlenecks, disaster recovery, dev-ops, etc.


Dos
* ask for clarification, do not make assumptions without verify with the interviewer
* thinking aloud - communicate
* make multiple design proposals
* design the most critical parts first
* ask for hints when stuck

Don'ts
* do not go into detailed design in the early stage
* do not thinking alone
