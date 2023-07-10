## Inspiration
I always wanted to learn new technologies but was limited due to my mediocre machine. I have 20 or more microservice applications with multiple middlewares running on Openshift environment and local development is just not possible due to the sheer number of CPU and RAM required to run the complete application. I was planning to move them to Quarkus to reduce the memory footprint and CPU while maintaining quick startup and low latency. There is always a steel learning curve to get familiarize with the technology. The constant changes require a lot of co-ordination and delays deployment to higher environment. The PR needs to be reviewed and tested on dev environment from multiple developer before running the E2E test and then merge to the main branch. Machine setup for onboarding new resources and making it ready for local development is a humongous task.

## What it does
Traditional Java stacks were engineered for monolithic applications with long startup times and large memory requirements in a world where the cloud, containers, and Kubernetes did not exist. Java frameworks needed to evolve to meet the needs of this modern world.

Quarkus, _the supersonic subatomic Java_, enables Java developers to create applications for a modern, cloud-native world. Quarkus is a Kubernetes-native Java framework tailored for GraalVM and HotSpot, crafted from best-of-breed Java libraries and standards. The goal is to make Java the leading platform in Kubernetes and serverless environments while offering developers a framework to address a wider range of distributed application architectures.

I have created around **22 templates** to empower myself and the developer community to embark on their journey to **_learn and play with Quarkus_** while talking to **multiple middleware** and **databases**. Most of these templates have a User Interface to navigate through the application.

## How I built it
I wanted to have a generic Dockerfile to build the different microservices. I looked at multiple images and finalized on [ubi8/openjdk-11](https://catalog.redhat.com/software/containers/ubi8/openjdk-11/5dd6a4b45a13461646f677f4) due to the security, features and being certified by RedHat.
The ubi8/openjdk-11 [run-java.sh](https://github.com/fabric8io-images/run-java-sh) and [Quarkus Configuration](https://quarkus.io/guides/all-config) provides a lot of flags to customize the application without changing the source code. The Quarkus also allows to build native images which can be run like any executable using `-Pnative` to the maven build process.
I then looked at the quarkus quickstarts to see samples which are available to learn the technology. As part of the hackathon, I have tried different middleware like DynamoDB, S3, Kafka, MongoDB, Postgres, AMQP, MQTT and also reactive programming using SmallRye Mutiny. 

## Challenges I ran into
I was not able to get the remote development to work on my Mac Machine. I checked with the excellent Bunnyshell support team and they were able to get the remote development to work on their machine. However that did not stop me from playing around with Quarkus in Bunnyshell environment. I was able to do [SSH to the component](https://documentation.Bunnyshell.com/docs/Bunnyshell-cli-ssh) and make the changes in the file. The Quarkus hot reloading feature allowed me to see the impact of my changes in real-time.

I tried `DockerImage` kind approach for DynamoDB, S3, Kafka and Zookeeper and use the `{{ components.NAME.image }}` but that was not getting interpolated. I used the docker compose available as part of `Database` kind to create the custom docker images and overcome that restriction.

## Accomplishments that I am proud of
I can now use Bunnyshell EaaS to be used in my organization to overcome some of the bottlenecks. I would be getting the best EaaS to manage my day to day activities. I will have dedicated environment to get PR tested and move to higher environment with much better velocity now. The developers would be productive from day 1 of their joining and can concentrate more on the functionality.

## What I have learned
There is no single way to achieve a solution. A small change can create havoc if not tested properly losing the customer confidence in the company brand value. It always helps to test the changes tested E2E in dedicated environment to have more confidence in the release process.

Bunnyshell provides me a unique way to avoid [New Developer vs Old Codebase](https://twitter.com/i/status/1646903684570415104) from Day 1 on a Friday night. 

![Happy Emoji](https://www.pngmart.com/files/15/Happy-Emoji-PNG.png)

## What's next for Multiple templates - Quarkus quickstarts & sample 3-tier app
The hackathon exposed me to a completely new way to look at my EaaS. I would request my team to learn and play with quarkus through Bunnyshell environment to see how we can best fit the current application migration to Quarkus.
