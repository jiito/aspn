 # Work Log

 ## Entries

 DATE: 10-6-2022

 - DESCRIPTION:
   - Worked on setting up task management software and planning initial work for the first sprint.
   - Setup notion docs.
 - READ
   - https://github.com/ericm/pandit
   - https://www.techtarget.com/searchnetworking/tip/A-guide-to-distributed-network-architectures
   - https://info.varnish-software.com/blog/build-your-cdn-in-5-steps
   -

 DATE: 10-7-2022

 - DESCRIPTION:
 - Read papers about the diffrence in Decentralized and Distributed networks.
 - Started evaluating how the system may (not) scale.
 - READ:
   - https://dev.to/karanpratapsingh/system-design-content-delivery-network-cdn-bof
   - https://medium.com/must-know-computer-science/system-design-load-balancing-1c2e7675fc27
   - https://www.liveaction.com/resources/blog/centralized-vs-decentralized-vs-distributed-networks

 DATE: 10-10-2022

 - DESCRIPTION:
   - Corresponded with Pete about requirements of the system and potential points-of-failure.
   - Planned sprint tasks for week of work.
 - READ:
   - https://www.wired.com/2017/06/pied-pipers-new-internet-isnt-just-possible-almost/

 DATE: 10-17-2022

 - DESCRIPTION:
   - Found open source framework for building microservices with Wasm.
   - Read articles on how web services could be build using Wasm.
   - Began exploring using Spin as a starting point for building a network with these webservices.
 - READ:
   - https://www.wasm.builders/fermyon/rethinking-microservices-with-webassembly-and-spin-1gh3
   - https://webassembly.org/docs/security/

 DATE: 10-18-2022

 - DESCRIPTION:
   - Read through articles on load-balancing, HTTP, and TCP.
   - Investigated multiple open-source frameworks for building microservices with Wasm.
   - It seems that these open-source programs would be something to build off
     and the planned use of docker could be switched with Wasm.
   - Scheduled meeting with Pete to discuss implementation.
 - READ:
   - https://bparli.medium.com/adventures-in-rust-and-load-balancers-73a0bc61a192#:~:text=The%20load%20balancer%20copies%20the,the%20socket%20processing%20non%2Dblocking.
   - https://www.cs.carleton.edu/faculty/dmusican/cs348/webserver.html
   - https://www.oreilly.com/library/view/http-the-definitive/1565925092/ch04s01.html
   - https://www.fortinet.com/resources/cyberglossary/tcp-ip
   - https://www.pico.net/kb/how-do-multiple-tcp-clients-connect-simultaneously-to-a-single-port-on-a-server/
   - https://avinetworks.com/glossary/round-robin-load-balancing/
   - https://github.com/mhallin/loadbalancer-rs
   - https://rust-lang-nursery.github.io/rust-cookbook/net/server.html#listen-on-unused-port-tcpip
   - https://www.techtarget.com/searchitoperations/tip/Follow-these-6-steps-to-deploy-microservices-in-production
   - https://www.fermyon.com/blog/webassembly-component-model
   - https://spin.fermyon.dev/

 DATE: 10-20-2022

 - DESCRIPTION:
   - Meeting with Pete.
   - Starting to work on building the initial structure of the internal api by
     being specific and working through a specific example of a user using the
     application.
   - Investigated tradeoffs of certain file storage solutinons (Google Cloud
     Provider and AWS S3).
   - Investiaged which database to use as a backing for the uploading and
     retrieval of files.
 - READ:
   - https://developers.redhat.com/blog/2015/04/30/how-to-load-test-and-tune-performance-on-your-api
   - https://www.rust-lang.org/what/networking
   - https://www.reddit.com/r/rust/comments/cs2794/best_resources_for_learning_network_programming/
   - https://docs.rs/socket2/0.3.11/socket2/
   - https://cloudmounter.net/amazon-s3-vs-google-cloud-storage/
   - https://cloud.netapp.com/blog/cvo-blg-understanding-google-cloud-storage-costs#h_h2
   - https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/
   -

 DATE: 10-21-2022

 - DESCRIPTION:
   - Connected to the Cloud SQL instance.
   - Designed and diagramed the API endpoints.

 DATE: 10-23-2022

 - DESCRIPTION:
   - Worked through a tutorial setting up migrations with diesel and Postgres
   -
 - READ:
   - https://thomaslevesque.com/2020/03/28/using-the-oauth-2-0-device-flow-to-authenticate-users-in-desktop-apps/?utm_source=pocket_mylist
   - https://hasinthaindrajee.medium.com/browser-sso-for-cli-applications-b0be743fa656?utm_source=pocket_mylist
