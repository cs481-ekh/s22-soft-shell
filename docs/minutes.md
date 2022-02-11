Meeting 2/11/2022

Scribe: Carson
Present: Carson, Natalie, Doug, Rob, Anna
Absent: N/A
Goals: Finalize and Send Revized Project Plan to GoWest, Disscuss minimal deliverable document.

 - Plan needs formatiing pass
 	- Completed 
 -  Dissucssion around timeline and moving dtastructure implementation back 
 	-  Split context defininition between Milestone 3 and 4  
 - Discussed making precise MVP document for defineing the exact parts of the ST grammer we're implementing
 	- Doug and Rob have been working on it. 
 	- Really well defined itertive subsets of the grammer
 	- Picked subset 9 to capture the minimal deliverable to GoWest
 	- Will be complted by next Tues (2/15)
- Kanban board strucutre
	- Starting pull request reviews and feature branch workflow in Sprint 1 
	- Ignoring in Sprint 0 
- Carson sent finalized plan to GoWest
__________________________________________________________________________


Meeting 2/4/2022

Scribe: Anna (written after)
Present: Anna, Natalie, Carson, Doug
Absent: Rob
Goals: Complete project proposal

- Had previously split up sections of the proposal to work on independently
- Went through entire document collaboratively and made modifications, up to the middle of section 3.3
- Will meet unofficially Saturday as people are available to complete proposal

__________________________________________________________________________

Meeting 2/2/2022

Scribe: Rob
Present: Team SoftShell + Sky Logan, Joshua Downer, Jonas Abdo
Absent: N/A
Goals: License clarification, ANTLR, Final Project Proposal

Anna lead the meeting

Double checked the MIT license was ok for the GitHub Repo

Informed the GoWest team about CI requirements for project requirements

Talked about making the repo public vs private
	- Anna brought up the pros vs cons of time allowed for private vs public
	- GoWest team usually prefers private until at least the product is little more close to being complete
	
GoWest thinks that ANTLR would be a good way to go. The licesning(BST) is also good.

GoWest does not like the idea of GPL licensing

Informed the GoWest team about next Tuesday being when the Final Project Proposal will be complete.
	- Plan to go over the final project proposal with the GoWest team next Wednesday.
	- Joshua D requested a copy of the final project proposal prior to the meeting.
	- Plan to send the final project proposal to the GoWest team via Slack.

__________________________________________________________________________

Meeting 1/28/2022 (Initial Sponsor Meeting)
Scribe: Anna (scribed from recording)
Present: Team Soft Shell + Sky Logan, Joshua Downer, Jonas Abdo, Gerry Ens
Absent: N/A
Goals: Discuss and clarify project plan with sponsor

- Carson leading meeting agenda
- Anna recording
- reviewed block diagram and main project components
- discussed scope of project, language coverage
    - make sure to plan ahead for supporting language features like scope, exceptions
- discussed option of transpiler instead of interpreter
    - Gerry: not needed at the moment
- requested structured text samples
    - may be hard to find a good production example because they are usually complicated and connected to other pieces
    - for our purposes, external function calls will be C/Rust stubs
- Go West needs to review license of any library (including parser-generator) we use
- priority between more features and supporting more ST depends on what we get done
    - however, good language support is required ("95th percentile of most-used features in ST") to get the main value here
- on board with open-source license for software, do not want to lock it down
- ongoing communication
    - meeting weekly for first few weeks, then maybe every other week later
        - discuss internally and send GW schedule options
    - want to use a GW repository, their devs will check on our work
    - will use Slack to quickly reach out, along with email
- will show them completed project plan once we have it

____________________________________________________________________________

Meeting 1/26/2022

Scribe: Douglas

Present: Carson, Anna, Natalie, Rob, Doug

Absent: N/A

Goals: Test google meet, Sponser meeting plan, Contract

- Tested video and audiofor everyone
    -everyone was able to share video, speak, and hear
    - Anna is testing screen recording

- Writing outline for sponser meeting
    - shared google doc in drive
    - listed topics and question sections

- Discussed project timeline
    -will review and adjust after initial sponser meeting

- Discussed licensing agreements
    - interested in being open source
    - leaning towards standard open source types(MIT possibly)
    - need to see what sponser thinks

- Discussed program expections
    - get more examples from sponser
    - set expection for being able to run example code in proposal and structure for future developement.

- Created simple forign function interface in google drawings
    - layed out visual diagram of components
    - discussed interactions and purposes
    - plan to present to sponser at meeting

- Discussed how the program is callable from C and rust
    -how is function passed?
    -need more clarification from sponser

- Discussed how we want to present
    - dont want to be to rigid so we can adapt to sponser
    - Natalie incharge of check off topics on outline and covering any questions missed

- Contract
    - Ran out of time, will try to meet after spoonser meting friday

- Next meeting
    - We will meet directly after the the sponser meeting on friday
____________________________________________________________________________




Meeting 1/19/2022

Scribe: Carson
Present: Carson, Anna, Natalie, Rob, Doug
Absent: N/A

Goals: Time to meet, work split/interests, project approach, start on draft

- Decided on time to meet
  - 1:30 - 2:30 Friday's
- Project appoach
  - Decided to start down path of creating seperate C version to get first working version
  - Simultaniously start learning/exploring implimentation in Rust
- Work split
  - Doug and Rob
    - looking at ISO Structured text - Doug lead
  - Natalie and Anna
    - Start implementing a version in C using Flex lexer library.
  - Carson
    -  Look at overall Rust structure, how to implement cleanly in rust, C bindings, packaging, libraries ect. Potential help from Rob.
- Started working on Project Plan Draft
  - Anna took lead
- Started work on email to GoWest
  - Carson working on draft

