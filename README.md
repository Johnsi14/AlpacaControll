# AlpacaControll

## (A) Arch (L) Linux (pac) Package Repo Control

A Simple Arch Linux Package Repo Tool That auto-compiles Packages on change, Supports different Architectures and makes creating Packages easier in a TUI built with ratatui in Rust. At Least this is the Goal of the Project

### Preview

No Working Code yet but this is how the TUI should look like.

<img src="Preview.svg" alt="A Preview That might Change" width="600">

# How

## Probable Concept

This is how it Could work but it might Change during the Implementation as i have no Idea how good it might Work and how some Concepts Work as it is my first GUI/TUI App and the first Big rust Programm in general

<img src="Concept.svg" alt="A Concept That might Change" width="800">

### Blocks

#### Render Loop
The Render Loops gets data from the App State and Determines based on Data Like What i am currently Doing and then gets diffrent Layouts based on that in which Things like Progressbars also get the Data

The Render Loop runs at certain Tickspeed depending on what is Configured and runs always at that Speed as everything that might take Time is run on the Event Loop or as a Thread

#### App State
The App State is the State of the Application. It is the Data that is used to Render the TUI. It can only be Changed from the Event Loop

#### Event Loop
The Event Loop is the Loop which handles Keypresses and starts Jobs via Threadpool with a certain max Threads mose usefull for package Building

If the Event Loop runs slow the data doesn't change but the Render Loop will still run at the same speed and show certain Things Smooth 

#### Threadpool
The Threadpool is the Pool of Threads that are used to run Jobs. It is limited to a certain Amount of Threads and if there are no Threads available it will wait until there is one available

#### Keybinds 
Keypresses are handled by the Eventloop and change the Event Loop

#### Jobs
Jobs are Functions run in seperate Threads that can send Messages to the Event Loop to change the App State like Update The Progress

#### App State ---> Render Loop  
The Render Loop gets the App State as an refrence but can't change it

#### Event Loop ---> App State
The Event Loop can change the App State via getting an mutable refrence to the App State

#### Keybinds ---> Event Loop
The Keybinds are Precessed by the Event Loop when an key is Pressed and change the App State via Event Loop

#### Event Loop ---> Threadpool
The Event Loop can start Jobs via the Threadpool by sending a message Which Job to Start

#### Threadpool ---> Jobs
The Threadpool start the Jobs and then don't interact with them anymore. It myght be combined with the Threadpool

#### Jobs ---> Event Loop
The Job can send Messages to the Event Loop which then can change for ex. The Layout or the Progress of an Progressbar
# Functions

1. [ ] Tui Displays
2. [ ] Initial Setup
3. [ ] Setup Environment from Existing Repo
4. [ ] Status Part
5. [ ] Todo Part
6. [ ] Progress Part
7. [ ] Help/Keybinds Part
8. [ ] Information Part
9. [ ] Package Creation
10. [ ] Autobuild Packages

# Why

- There is no existing Tool that does this
- Learning Rust better for ex. Error Handling and Bigger Projects
- Learning the ratatui Library for future Projects
- Learning GUI/TUI Programming Principles
- Learn how to efficiently Code
- Learn how to manage a Project

# I am getting close to the Last year of my apprenticeship as an Electrician so I don't Have much Time yet to build it.