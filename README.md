dagda
=====

TODO
----

- [] agent binary
- [] server binary. fork exec
- [] add server to pool by installing agent and calling home (pull)
- [] agent heartbeat (removed from the pool after N fails)
- [] add server to pool by adding kube config (push)
- [] add server to pool by adding ssh config (push)
- [] pull config from private git repo. sync N seconds
- [] execute arbitrary commands manually on linux host. ssh
- [] execute arbitrary commands manually on linux host. ssm
- [] execute arbitrary commands manually on kubernetes
- [] execute scripted commands manually on linux host. ssh
- [] execute scripted commands manually on linux host. ssm
- [] execute scripted commands manually on kubernetes
- [] store config in <TBD>. Force DB from the get-go?
- [] store execution logs in <TBD>. Is git a very silly idea?
- [] workflow orchestration (call another workflow with inputs set)


install agent (optional)
-------------------------

```sh
curl -L https://somewhere/agent -O dagda_agent && chmod +x dagda_agent
# install agent
./dagda_agent <commands>
              --name= set agent name
# uses nic as identifier
# send cpu info, ram info, disk info
# send heartbeats
# sets up systemd unit
# pull based?
./dagda_agent listen --home 1.2.3.4:90001
./dagda_agent listen --sub <kafka info / event bus info>
```

install server
--------------

```sh
curl -L https://somewhere/server -O dagda_server && chmod +x dagda_server
./dagda_server <commands>
# github auth
# okta auth
```

example tasks
-------------

value: high
challenge: secure key storage
challenge: crons

1. spin up server
2. go to settings > repos
3. add private key for repo
4. test repo connection
5. save repo connection
6. go to servers view
7. click on create new server and specify "kubernetes" as the type
8. add config for kubernetes config and give it a reference (name)
9. go to jobs view
10. select git repo
11. specify path (in repo) to job
12. specify parameters
13. specify docker image for execution  # <-- perhaps a docker image library?
14. select server (kubernetes server)
15. select kubernetes cluster by reference (added in step 2)
16. specify an existing namepace and pod
17. execute and get near-realtime logs from pod as it executes
18. handle success / error. persist logs. show execution log (history)


value: high
challenge: connecting to containers with no shell

1. spin up server
2. add config for kubernetes config and give it a reference (name)
3. go to commands view
4. enter arbitrary command
5. select kubernetes cluster by reference (added in step 2)
6. specify an existing namepace and pod
7. execute and get near-realtime logs from pod as it executes

value: low

1. spin up server
2. add config for kubernetes config and give it a reference (name)
3. go to commands view
4. enter arbitrary command
5. choose generic image (e.g. ubuntu-22.04) or specify custom
6. select kubernetes cluster by reference (added in step 2)
7. execute and get near-realtime logs from pod as it executes
