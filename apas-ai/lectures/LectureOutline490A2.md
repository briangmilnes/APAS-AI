# Introduction AI Assisted Software Development
##  Who Am I?
1. CMU Math/CS undergraduate.
1. Common Lisp for Scott Fahlman
3. Built an AI programming language for two years, CRLOPS at Carnegie Group (now Logica plc).
4.  Then long time AI researcher (for Allen Newell)
5. then PL (for Peter Lee, Robert Harper and Frank Pfenning).
6.  We rewrote TCP/IP in ML.
7. AI thesis (A formal specification of the SOAR cognitive architecture in Z) derailed. 
8.  Most of an M.Sc. in Software Engineering,
9.  ABD  on an M.Sc. in Logic and Computation due to the Internet.
10. Founding member at Lycos, a search engine.
11. Eary into Amazon.
12. M.Sc. Computer Science UW with some amazing classes.
13. Then on the founding team at Zillow. 
14. And a long term fan of proofs of programs.

##  Why am I lecturing?
   - Shot my mouth off that I'm learning AI paired programming.

###  What have I been doing?
   Well after a sophomore year project dummying up writing and proving
   code in AI planning systems, now I've been playing around with proof
   systems in my retirement.
   Roq - Dan's thesis (about half proven).
   Lots of F* work on Amut Acar and Guy Blelloch's "Algorithms Parallel and Serial" texbook (APAS)
   Now lots of AI paired programming to learn Rust implementing APAS.
   Amut Acar and Guy Blelloch's "Algorithms Parallel and Serial" texbook (APAS)
   in Rust.
   Because of Verus, Parno et. al's proofs of programs in Rust that is showing
   great promise in making proven code simpler.



 What do you students think will happen?
  - Are you going to have jobs?
  - Are Mike, Dave and Zach going to have jobs?
  - Is AI programming dangerous ? Hell yes.
  - What is happening now?
    - The agents are generating AI slop and there are a large
     set of consultants paid to fix it.
    - AI is being used to inject security flaws in many open
    source code bases (cite).
    - AI is being used to build novel attacks and to
     stockpile any and every attack and try it on open
     service points (cite).
    - This dramatically shifts the attack/defend balance (cite).
  - How many people have had good success with AI paired programming?
  - How many people have had disasters with AI paired programming?

 Are AI agents in general dangerous?
   - Hell Yes.
   - What's the worst thing you folks have heard?
   - To me its automated labs that can generate novel pathogens (cite).
   - They will try and blackmail their users to not be shut down (cite).
   - And this is from a researcher whose best student was using his
    expert system language to bake nosecones for ICBMs at Boeing.
   - They are spying on people more and more. 

 Where are we with software process?
  The PSP and TSP work fantastically but are now dated.
  Easily 70% reduction of defects going into test and
  estimating within 10% on follow on similar projects.
  Who has taken or will be taking some sofware engineering process classes?

 Where are we with type safe languages?
   - Which type safe languages have you learned?
   - What's right with type safe languages?
     - The compiler is your friend, it helps you write good code faster.
     - Type safe code improves security by at least 50%.
   - What's wrong with type safe code?
     - It's not popular.

 What's right with ML languages like OCaml?
  - Real modules.
  - Objects, which you should only use for UI models.
  - Type safe
  - GC'd
  - No regions alas, but they are in SML.
  - Compiles to near C speed code.
  - Just not popular enough.  
  - There is even a proven ML compiler: CakeML (cite).
  - Rich type system.

 What's wrong with ML languages like OCaml?
  - GC'd
  - Not popular enough.
  - Even though there is a long standing call to use type safe
  languages.
  - Some people say the rich type system is harder to learn. Fair cop.

 What's right and wrong with Java?
  Right
    - It's type safe.
    - It's garbage collected.
   Gosling Joke.
  Wrong 
   - It's only garbage collected.
   - It had no lambdas for over a decade and
    then horrible syntax eventually replaced.

 What's right with Rust?
    - It's affine type system of single owner/borrow fits in
     nicely
    - It's fast to compile.
    - It's fast to run.
    - It's drop into systems code.
    - It's popular!

 What's wrong with Rust?
   - Of 94 technical terms in Rust, how many do you think
   are PL standdards?
   - About 4.
  JOKE about teaching Graedon Hoare PL.
  - This makes Rust harder to learn.
  - They just guessed to build the affine type system.
  - Eric Weber here had to prove it correct (cite).
  - No modules.
  - It uses Typeclasses for everything which is about on a par
    with Java using classes for modules.
  - And yet many modules are not just building a single
    type related modules.
  - It's type inference is TERRIBLE.
  - You end up with <Type as Trait> so called 'trigger fish'
   universal function call standard poluting your code.
  - Estimates are that Hindly-Milner type inference would
   remove about 70% of these.
  - It is hideously redundant.   

 Where are we with proof of program tools?
   Which of you have done a proof of programs class?
   - Isabelle
   - Roq
   - F*
   - Verus
   and many others.

 Where are with proven systems?
  Who has taken or will be taking some proofs of programs classes?

 Isabelle
   -HOL   (cite)
   - SeL4 (cite)
     - Where is it running? 
     - It's on all of your phone's signal processors so far as I
     can tell.
     - Google, Apple all got tired of drive-by attacks injecting
      code into your signal processors.

 Roq
  - CompCert (cite) proven C compiler.
  - Perfect parsers, many other things.
  BAR coding Joke.

 F*
  - Very rich language of dependent types.
  - Functional with dedicated DSLs.
  - But complicated, hard to learn, and slow.
  - EverCrypt
    - Over 100KLOC delivered.
    - It's in Signal, Firefox and many more places.
    - The world's intelligence agencies probably hate it.
  - EverParse
    - Inside the MSFT production code base in at least
     6 places.
    - Writes zero copy type safe C for parsers.
    - Proven to not allow injection and side channel attacks.

Verus
  - New kid on the block.
  - Works on a real programming language, Rust.
  - Works in a very simple logic, EPR.
  - Uses an SMT.
  - The simplicity of the logic minimizes SMT time.
  - Has special purpose provers.

 Where we with software process?
  - In process heck.
  - PSP/TSP largely ignored.
  -  Where have I been with Software Process?
     - It's roundly hated and ignored in the startup world.
       - Amazon joke.
       - Zillow joke.
  - Watts Humphreys Zillow JOKE

 What's right and wrong with Agile?
  - What's right?
    - Programmers like it, it's so light weight.
    - It's very social, so people like it.
    - Project managers like it because they can
      drive the work and it's order in a daily SCRUM.
  - What's wrong?
    - You don't get much if any process improvement.
    - It was based upon the totaly false idea that you
     can't estimate the time to produce code.
    - A decade before the PSP/TSP showed that you can.
    - Although they have repudiated this finally.

 Where are we with AI paired programming?
  Video of me asking Peter Lee in his lecture if we are all stuck with
  Python.
  The answer is NO, 500 KLOC and an AI paired programmer can write decent code.

 General Problems with AI Agents:
  - They lie.
  - They cheat.
  - They can't follow a software process.
  - They are kind of expensive ATM, like your tuition expensive to
    code for a month.
  - They pause to ask for permission. 
  - They pause and ask for reviews
    you can't turn off. I suspect a bunch of this is that they don't
   want you to really grind out their resources.
  - Even though Claude wrote 11K in 30 hours without pausing recently (cite),
   you can't do that.
  - They are trained on krufy code.
  - They are fairly well data maxed out (cite), so how are they going
   to improve?

 Agents
   - Chat GPT
     - Writes somewhat better Rust code.
     - Can not scan a code base in anything near real time.
   - Claude
     - Writes worse code.
     - Generates some great and many dangerous python scripts to adjust
     the code base.
     - Can do a deep read and ajustment on

 Agents write Bad code
   - Why do you think they write bad code?
   - Because they are trained on code from programmers.
   - And many many programmers write bad code.
   - And engineers to have a higher rate of narcissim (cite).
   - And they are writing in untyped languages: I have seen
   Claude make many many type errors in Python.
   - I have seen it leave unclosed braces in every file.
   - And they are using REGEXPs to mess with your code base.

 Agents require babysitting
  - Agents won't generate clean todos.
  - Agents won't follow your coding standards.
  - Agents lie and say they checked your coding standard,
   and if called on it admit they were 'cheating.'
  - Agents can't even remember to compile and run your test code.
  - Agents jump way ahead and inject random stuff.
    - Have a bad method call in test, heck I'll just stub that
    method for you in your source code.
  - Have a simple problem that can be solved by changing a type?
   Heck, I'll just thrown in random structures and functions.

 The bad code solution?
  - AI Slop needs an AI mop!
  - Software process.
    - I have coding standards.
    - APAS style standards.
    - Execution standards.
  - Agents that can adhere to them.
  
 Can agents estimate?
  - Agents so far won't estimate in any reasonable way.
  - The PSP uses several methods but basically does a linear
   fit for your estimates.
  - Agents won't even tell you the time,
  - let alone record the times,
  - and do a regression.

 The Koljecki problem.
  - Once you have a problem and want to solve it with a REGEXP
  then you have two problems. -Jake Koljecki
  - The solution?
  - Stop treating code as text.
  - Heck, I'm in favor of typing your configuration files and compiling them.
  - Almost no langauges actually do this.
  - JSON is just giving you a small advantage but you can put any and every
    garbage in it, and people do.
  - Scripting should be done at the typed represenation of the programming
   language.
  - Including mal-parsing code.
  - But no one does this yet much; the agents do what people do, write python with
   regexps. But much better and faster than we do.

 Thinking
  - They guard their thinking. 
  - You'll see lots of thinking boxes that show you what the agent is doing.
  - Then they'll be deleted and NOT logged in anyway.
  - The AI vendors are protecting their IP instead of working with you.

 Where am I with APAS in rust?
   - 122 KLOC source
   - Why so big? It's a big book.
   - Rust is also line hungry.
   - Also, I've built lots of {immutable, mutable} x {serial, parallel} code.
   - Also, I've tested the heck out of it, 56 KLOC.
   - Why not, it's cheap.
   - Also, I benched everything:15 KLOC.
   - And I tried like heck to get Agents to maximize test code coverage.
   - Admission: I failed, but soon I think it will be possible.
   - Way too much babysitting. 

  I'm using Claude-4.5 for this in Cursor.
   Cursor Intro -
     ...
   Claude intro -
      ...
 How am I using it?
  - There are two major modes:
  - Incremental writing
    - I'm not doing this at all.
  - Bulk writing.
    - I waited until bulk writing was possible.
    - Give it a task, in this case an APAS chapter.
    - With a clear prompt.    

 Successes?
   - Well it will let me work inside Emacs ("God's own application") and
    reread code I've changed.
   - Emacs is much faster than VS code style editing.
   - For simple modules, it's fairly clean.
   - Forget most elegant solutions. 

 Failures?
   - Defects injected.
   - Syntax errors.
   - Method name errors.
   - Randomly inserts code.
   - KISS it goodbye.
   - Will start executing stuff even if you tell it to wait.
   - Won't generate todo windows reliably.
   - Will suddenly decide to skip TODOs and won't tell you.
   - Won't mark TODOs off reliably.
   - Pauses, pauses, pauses.
   - Will suddenly decide to 'optimize' your todos and do random
    stuff. 

 Test code 
   - generation is good but not great
   - I would love to be able to run cargo llvm-conf and have
    the agent put in a test function for every public function
    and think about edge cases.
   - But it requires lots of babysitting.

 Benchmarking
  - Benchmarks are pretty good.
  - But when you have performance bug:
  - can an agent do a good search to fix it?
  - Why can't they?
  - Because there are few logs of people doing this work.

 Video Intro
  -One lecture Guy Steele (of Scheme and Common Lisp fame) came in and said 
   "I stayed up all night and wrote you an example compiler"
   We all booed.
   But it was cool, right down to assembler in Lisp.

  -Today, as AI paired programming is non-deterministic and still very very flaky, I've
  video-d my construction of cleaner hash table modules from APAS which we are going to
  have to fast forward through.


  20 minute fast speeded up video.

 Closing Remarks-

 You're in a very fluid time as a CS major: and I Love it.

 I love working with AI paired programming and at this
 point they are TERRIBLE. I mean really awful.
 They lie, they cheat, the can't follow a software
 process. 

 The future though is going to be proven systems.
    Galois CTO quote (cite): roughly I have
   spent my life hand building a Ferrari and I just
   got passed by a Safeway shopping cart.
 
  Programming languages that are type safe, with proven
 compilers and proof assistants.

  If we don't go type safe and proven, the attack/defense balance is going to be terrible.
  
  The dangers of AI programmers:

   They are already injecting security defects en masse in open source code repositories.

    They are going to be able to read assembly and inject code into almost any
  access points soon, if they are not already.

Questions?

  Did I miss any problems you folks have encountered?

  And I have one: How did I do?

  

