Meeting 4/13/2022

Scribe: Doug

Present: Rob, Natalie, Doug, Carson, Anna

Absent: None.

Goals: Go West project update and handoff coordination

- Anna leading meeting
- decided to use next regularly schedule meeting for handoff meeting
- Documentation planned to deliver User guide and Developers guide
- user guide needs to include instructions for starting from scratch
- requrest for List for next steps of developement 
- rust documentation currently being generated in a file thats included in git ingore. Discussed with Go West, decided thats probably appropriate to keep from haveing to generate so much material "churn" every time something new is added to the repo.


After Go West meeting meeting

- Targeting 21st to have code and documentation initially done to allow weekend to work on poster
- Everyone needs to be refining there sections they are working on for the rust documentaiton generation
- Developers guide currently generates webpage, need to setup on the Go west repo at github page, can link to it from the teams public github page.
- AST execution doesnt appear to have any work that needs to be done since most functionality is handled elsewere. Can be classified as a non task. Instead Anna is going to get started setting up project poster.
- Will be Assuming all function files are in the same directory as program.
- Discussion on modifying functions for Rust API
- As soon as user guide is complete enough for someone to try to use our program let Go West know so they can take a look so corrections can be made for final documentation.

__________________________________________________________________________
Meeting 4/8/2022

Scribe: Doug

Present: Natalie, Doug, Carson, Anna

Absent: Rob(unplanned Absent)//20 minute late, but extremely quick meeting.

Goals: Sprint 4 Status

- Will Talk about Poster design and Github pages during next meeting(Friday 4/15)

- Discussed how function file names will be made available from parser. Carson will have parser create a list of program names to have availble at top level.

-  Status
	- Carson: Working on Parsing for subset 8 and 9 today, should unblock current developement.
	- Anna: Should be able to get AST and function load once unblocked.
	- Doug: Should have the first ST test file for function and function call up by the end of the day. Rob and Doug were able to track down an error in a different part of the code that was preventing them from getting the nesting step execution working properly and got that pushed to the repo
	- Natalie: Focusing on storing function, C API can wait since it wont really block much	
	- Rob: Finished Step function for IF and Nesting. (After meeting update)

__________________________________________________________________________

Meeting 4/6/2022

Scribe: Doug

Present: Natalie, Doug, Rob, Carson, Anna

Absent: none

Goals: Sprint 3 retrospective and Sprint 4 assignments

- Went over remaining tasks of sprint 3
	- Completed step with the exception of nesting, pushing to sprint 4 to finish nesting
	- C API, Ened up being much more complicated than original estimates, more time needed to be allocated. Will attempt to finish today but may push to sprint 4 in needed.
	-  C API testing task pushed to sprint 4.

- Retrospective
	- Anna: Lot of work down this print, more work towards end of sprint which creted a lod of code needing to be merged together and the end. It is ok to have some tasks roll over to the nest sprint, just document what you did for Dr. Henderson. Sprint 4 will be work heavy.
	-  Rob: Time management got thrown off some with Spring break in middle of Sprint
	-  Natalie: Also thrown off with spring break, C API was more complicated and fixng one issue often caused other issues, so taking a lot of unexpected time to complet.
	- Doug: Pair programming went really well this sprint, worked heavily with rob on developing the step functionality for While and IF statements.

- Sprint 4 Assignments
	- Most tasks already assigned, assigned all remaining
	- added tasks:
		- github pages
		- create ST test programs for functions
		- added function calling functionality
	- discussed what the differences would be between the documentation for user guide, developer guide, readme, and github Pages. Rust document generation should make task much easier as long as we have the code itself well documented through out  
	- discussed whether poster should be included at a sprint task, will decide at a later date	 


__________________________________________________________________________

Meeting 4/3/2022

Scribe: Doug/Rob

Present: Natalie, Doug, Rob, Carson, Anna

Absent: none

Goals: Sprint 4 planning

- Set up new rolls for sprint 4
	- Scrum Master: Carson
	- Product Owner: Anna
	- Scribe: Douglas
	
- On track for finishing Sprint 3
- areas left on sprint 3
	-  Step function for IF
	-  C AST

- for sprint 4 will need to complete tasks 19-21 from project plan

- expand functionality to incoperate subset 8 and 9 which is essentially Function handling and varaible types that go with it.
 
- discussed how to go about file lookup for ST function files. Decided that parsing of the main program should return a list of function names, then each function would get parsed and stored in teh data structured. Not nested in the AST of the main program.

- Set up new sprint with goals, user stories, tasks

- Anna closing tasks for execute function in If and While since step function will now be covering functionality.

- Each member will create a task to document their respective sections of the code

- User guide will include everything you need for being a user of the library

- Plan to create a developer guide to go along with the user guide for the engineers that pick up this project next
	- Each perosn on the team will write there about their respective section of the code in the developer guide
	- Make a task for sprint 4 referencing user story #149 before adding your Section ot the developer guide  
- Assignment of tasks will wait until Wednesday Morning
__________________________________________________________________________

Meeting 4/1/2022

Scribe: Rob

Present: Natalie, Doug, Rob, Carson

Absent: Anna

Goals: Sprint 3 updates

- Carson is wrapping up truncation today or Monday
	- also, setting up the build file
- Doug is working with Rob in implementing the if, else if, and else
- Natalie is still working on the C API.
- Rob is working with Doug in wrapping up the if, else if, and else
	- waiting to start the test implmentation of the C API
		- done some research on testing C API within Rust
- Discussed planning for Sprint 4, the proposed meeting time is 8pm on Sunday April 3, 2022
__________________________________________________________________________

Meeting 3/30/2022

Scribe: Rob

Present: All

Absent: none

Goals: Meeting with Go West

- Partly through Sprint 3
- Currently, Execute function up to subset 7 and the C API
- Anna had a question about precision to Go West, Josh thinks that the approach we are taking is appropriate for the project. Possibly, have future work be to investigate the precision issue.
- Josh from Go West had a question on expectations on what they should expect for what work will be completed and ones that we feel we won't be able to complete. As a team, we have not identified any and are still aiming to complete milestones that we have established
- Also, mentioned that we are planning to do project handover documents in Sprint 4
- Talked about the if implementation between Rob and Carson
- Anna is almost done with their work for the Sprint looking for extra work
- C API is scheduled to be completed by Friday April 1, 2022
__________________________________________________________________________
Meeting 3/18/2022

Scribe: Rob

Present: All

Absent: none

Goals: Sprint 2 Retrospective and Sprint 3 Discussion

- Run and Step implementation held up testing execution methods
	- important for Sprint 3 is the update of the Step method to support while loops
- How does everyone see their workload
	- everyone seemed to feel the load was fine for each person; no complaints
- Are there any tasks blocking workflow
	- Parser will be worked on today, should unblock the execution development
- Discussion of execution in While to allow for step over functionality
- While and If implementation with step and run
	- Proposed shifting list to a stack
		- whatever is on the top of the stack will be executed
		- executes a statement then push the statement list to the stack
		- the method was demonstrated via screen share
	- Another proposal was a tupple for the While that contained another vector
	- If we used the stack method, how would we tell the user the line where the code failed?
		- currently done with the program counter
	- a counter proposal is to implement a stack in the step function
		- this then wouldn't require change to the AST
- Meeting reached the hour mark, discussion ongoing, plan is to take the conversation off line	
__________________________________________________________________________
Meeting 3/16/2022

Scribe: Rob

Present: All + Go West

Absent: none

Goals: Update Go West on Progress & Demo Interpreter

- Go West is ok with going to everyother week

- Carson demod the interpreter

- Go West had questions about scoping and nested scopes
	- Doug has already been researching this issue with Structured Text and more to follow

- Statement list discussion in regards to a while loop
	- possibly use a stack of program counters to keep track
	- there will be an additional meeting to discuss this further
	
- Discussion of why run and step are not methods in the program handle
	- currently, run and step are functions
	- the C API will need to be functions

- Accessing the context via the program handle
	- this is not able to be done in C
	- instead of passing the value of the context pass a reference
__________________________________________________________________________

Meeting 3/13/2022

Scribe: Rob

Present: All

Absent: none

Goals: Plan Sprint 3

- Anna will be the new Scrum Master, Natalie will be the Product Owner, and Rob will be the scribe

- Based on the assumption that Sprint 3 is 3 weeks due to Spring Break

- Built a new project board for Sprint 3 as well as a new Milestone for 4

- Goal for Sprint 3 is work towards Milestone 4
	- set up C API and integration test
	- Working to interpret a program up to subset 7
	- create user guide explaining API including usage examples
	
- Generated user stories and tasks for Sprint 3

- In the planning we came up with 10 tasks and assigned them to members of the team
__________________________________________________________________________
Meeting 3/11/2022

Scribe: Anna

Present: Anna, Doug, Rob, Carson

Absent: Natalie

Goals: regular team meeting

- went around and did updates
- discussed implementation of run/step -- run will loop step
- discussed if declarations should get processed statically during load or dynamically upon stepping
    - we think functions cannot create globals
    - either approach would work
- next sprint planning meeting will be this Sunday at 5
- demo plan: create a simple Rust executable that uses our library to execute a simple ST program
__________________________________________________________________________

Meeting 3/9/2022

Scribe: Anna

Present: Team Soft Shell + Jonas Abdo, Joshua Downer

Absent: Sky Logan

Goals: Go West check-in

- going off agenda doc in drive
- our updates:
    - halfway through sprint 2
    - testing groups of example programs
    - can execute AST now
    - can manipulate variables in context
    - will have full flow after run/step completed
    - our work is on-schedule, will complete sprint goals
- proposed moving to meeting every other week, AFTER next week where we will demo ST program execution next week
    - Jonas says yes, we will also check with Sky
- Joshua would like to try and run our code from our present instructions, will make an issue if it doesn't work
__________________________________________________________________________

Meeting 3/4/2022

Scribe: Anna

Present: All

Absent: None

Goals: regular meeting + sprint 1 retrospective

- sprint 1 retrospective comments
	- remembering scrum updates
	- update others on blockers sooner
	- stay in our specific roles so we go faster, but communicate more about them
	- break up tasks if they turn out to be too large
- we are encouraging but not requiring scrum updates on sunday in addition to usual tuesday/thursday
- clarified list of currently blocked tasks and how things will get done
- discussed where step/run fit in and connection between them
- discussed structure of ast node execution
- discussed how to organize contexts for different function calls
	- some part of that implementation can be done now but won't be used until later subsets
- expecting spare time in sprint 1, will stretch to implement subset 7

__________________________________________________________________________

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

