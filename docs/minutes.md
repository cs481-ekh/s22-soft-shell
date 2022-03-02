Meeting 3/2/2022

Scribe: Rob

Present: Carson, Doug, Natalie, Rob, and GoWest

Absent: Anna

Goals: Sponsor Update

- Doug updated GoWest about Sprint 1 and then the way ahead for Sprint 2
- GoWest was wondering when they can use the interpreter to run a ST program
- The team asked GoWest for a ST program that would be similar to something seen in production
- Since a lot of the code is from the customer, GoWest suggested pushing the code with edge cases
- Carson came up with the idea of testing ST code against a good inter
- After the team meeting...
- Doug and Rob are going to come up with test files to test edge cases for subset01

__________________________________________________________________________

Meeting 2/27/2022

Scribe: Anna

Present: All

Absent: None

Goals: Plan sprint 2

- Natalie will be new scrum master and is leading meeting, Doug will be new product owner
- Created new sprint board and milestone, user stories for this sprint coming from Project Plan
- Targeting completion of subset 4, parsing and execution, for this sprint
- Putting tasks for parsing and execution nodes under the same story because they are tightly linked
- Completed planning of all sprint 2 user stories and issues
- Do we assign issues ahead of time or just take them as we go?
	- Group decided we take them as we go

__________________________________________________________________________

Meeting 2/25/2022

Scribe: Natalie

Present: Carson, Anna, Doug, Natalie, Rob

Absent: N/A

Goals: Weekly team meeting

- Acknowledgement from Carson about receipt of email
- Subset 1 AST generation task is almost complete
	- Carson will have it pushed by the end of the day, otherwise he will ask someone else to finish it off
- Currently the structure is bottom-up with the parser generating the AST
	- Later, this may change to be top-down, where the parser fills in an already existing AST
- Discussion about documentation tools and where to put the generated documentation
	- Anna is working on this and will decide what makes the most sense
- We are close to finishing milestone two
	- We are on track to be able to parse subset 1 by the end of the sprint
	- Program handle task should be able to be completed in the next few days
	- Program load function task is blocked waiting on the program handle
- Discussion about line tracking within the program handle
	- We will start by just keeping track of the statement number
- When2meet to schedule our sprint two planning meeting for sometime in between Sunday and Wednesday

__________________________________________________________________________

Meeting 2/23/2022

Scribe: Natalie

Present: Carson, Anna, Doug, Natalie, Rob, Sky, Joshua, Jonas

Absent: N/A

Goals: Sponsor update

- Update on our current progress with the parser
- We will have knowledge tasks on the scrum board that are not linked to any code
	- These will have a knowledge task label
- Request from sponsor to include Josh and Jonas on all of our pull requests
	- We don't need to wait for them to review it in order to merge it, they'll review things if they have time
	- If there is something specific we want them to review, we should ping them on slack
- It was recommended that we put together documents that reflect the information we find in our knowledge gathering tasks

__________________________________________________________________________


Meeting 2/18/2022

Scribe: Natalie

Present: Carson, Anna, Doug, Natalie, Rob

Absent: N/A

Goals: Mid-sprint team meeting

- Updates on everyone's progress so far in the sprint
- Rob will start working on the parser for subset 1
- Carson will set up the documentation
- We will use rustfmt tool to format our code
- Discussed having a separate folder for resources related to rust
	- Decided against it
- Discussion about pull requests
	- Pull request commit is in the form "Closes #issue_number (#PR_number)"
	- Going forward, we need to put more information in pull request descriptions
	- Comments relevant to the code should be posted under the pull request
	- Comments not related to the code can be sent in discord
- We will make a user story about learning rust
	- Everyone's rust learning tasks should be linked to this user story
	- Other learning tasks should be linked to the story that they are being learned in order to complete

__________________________________________________________________________

Meeting 2/16/2022

Scribe: Natalie

Present: Carson, Anna, Doug, Natalie, Rob, Sky, Joshua, Jonas

Absent: N/A

Goals: Sponsor update

- Project proposal
	- New version addresses all concerns
	- Approval email has been sent
- Project state document explained to sponsors
- Sprint 0 is completed and Sprint 1 has started
	- We are targeting milestone 2 for sprint 1
- Questions from sponsor
	- Q: Are we committing directly to the repository?
	- A: We did for set up in sprint 0, but we are moving to a feature-branch workflow for the rest of the project
	- Q: How are we going to be able to test our work?
	- A: We are going to be working on the API functions as we are working on the interpreter, so we can use them to test

__________________________________________________________________________

Meeting 2/13/2022

Scribe: Natalie

Present: Carson, Anna, Doug, Natalie, Rob

Absent: N/A

Goals: Sprint One planning

- Assigned roles for sprint one
	- Product owner: Carson
	- Scrum master: Rob
	- Scribe: Natalie
- Created goals for sprint one
	- Complete milestone two
	- Parser will be able to generate the correct AST for subset one of ST
	- Data structures for context and program handle will be defined
- Decided on stretch goals for the sprint
	- Start the tree walker functions and the functions to modify program context
- Discord channel was created for scrum updates
- User stories were created
- Divided up work
	- Parser generator: Rob, Carson, Doug
	- Data structures: Anna, Natalie, Doug
- Updates on everyone's status with learning Rust

__________________________________________________________________________

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

