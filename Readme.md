# Implement some algorithms

## Directories

- arrays/ - Algorithms for arrays

- dynamic_programming/ - Algorithms for dynamic programming

- grids/ - Algorithms for 2D arrays

- linked_lists/ - Algorithms for linked lists
  * `singly_linked_list.rs` - Singly linked list implementation with basic operations and iterator

- lists/ - Algorithms for lists

- strings/ - Algorithms for strings

- trees/ - Algorithms for trees
  * `binary_tree.rs` - Binary tree implementation with basic operations and iterator


## Review Code With Copilot

Python is one of the languages Copilot supports and being good at

How to use Copilot Chat for code reviews

1. Open the Chat window.

Type '/help', and you see a list of available commands:
- /tests - Generate unit tests
- /simplify - Simplify the code
- /fix - Fix problems and compile errors
- /explain - Explain how the code works
- /doc - Document the current selection of code
- /feedback - Steps to provide feedback

Click `/explain`

- Time-efficient
- Easier to find bugs
- Acting as a senior assistant
- Validates and double-checks your own analysis in code reviews

Without Copilot 
- Requires context switching
- Mentally draining

Code reviews were more actionable and completed 15% faster with GitHub Copilot Chat.

Using Copilot Chat for Codebase Analysis

Context
- #selection - The current selection in the active editor
- #editor - The visible source code in the active editor
- #terminalLastCommand - The active terminal's lst run command
- #terminalSelection - The active terminal's selection
- #file - choosing file in the workspace

## Improving prompt

Click a button to enable copilot to generate commit messages in VSCode source control button

- Enhanced productivity
- Particular helpful for bigger pull requests

Pull request summerization

- GitHub Copilot Enterprise version has a Summary button in each Creating Pull Request
- This featuer only works with GitHub Enterprise accounts

How Copilot Works

- Copilot takes into account the context of your code
- If no function prototype, it will just output something based on its training data
    * Prompt engineering give large language models extra context for their operation, to get the best output.
    * Copilot is based on ChatGPT 4
    * Course `Prompt Engineering for Developers with Google Gemini`

Write comments, Copilot will generate code, press tab to accept it.

Examples:
1. Write a funciton to calculat the average ticket price for a given event
2. Implement the function calculate_average_price that takesa a list of prices as input and returns the average price as a floating-point number
   
## Code Refactoring with Copilot

1. Automatically adding unit tests with Copilot
    * Writing tests is time consuming
    * Unit tests must be maintainable
   
   How to do it?
   
   Select the code that needs unit test code. Click * buttion, enter /tests in the prompt window. Click Accept to generate a new file.

2. Solve furthur code issues

3. Find bugs with Copilot

    * Select the code, use /explain command

4. Document Generation

   * Select the code, use /doc command


# GitHub Fundamentals (Pluralsight)

## 1. Overview of GitHub and Git

### Concepts of Git

What is Git?

- Popular source control system
- Distributed system
- Free and open-source

Why use Git?

- Fast
- Disconnected (local repository)
- powerful yet easy to use
- Branching
- Pull requests (enabling colaborating)

But ...Why not Git?

- Different
- Learning curve
- Tools (it really shines when using commandline)
- Binary files
  

What is GitHub?

- Hosting service based on Git
- More than just source control for your code
- Free and paid options

Get your machine ready


- Git-scm.com 
- An editor (VS Code)
- GitHub account
  
  Inside MINGW64

      git config --global user.name "your name"
      git config --global user.email "your email address"
      git config --edit --global

  Change git default editor

      git config --global core.editor "code --wait --new-window"

The 3 States of Git

- Commited
- Modified
- Staged

These states are still local

Working directory

Staging area (waiting to be commited) .git rep

Remote repo GitHub (The fourth area where code can be stored)

Workin with Git

- Command line (CLI)
    - git
    - git config
    - git init
    - git clone
    - git add
    - git commit
  - 
- GUI (some commands are unavailable)

### Demo

    mkdir "d:\code\pluralsight"
    cd "d:\code\pluralsight"
    git init
    ls -la
    git status
    code readme.md
    git add readme.md
    git commit readme.md -m "with comment"
    git add .
    git log
    git commit -am "new files message"


### Understanding GitHub

- Project management
- Pull requests

### Beyond the Basics

## 2. Getting Started with GitHub

### Understanding GitHub

- Hosting service for Git
- Git and a lot of extras
- Social aspect of coding
- Part of Microsoft since 2018 (7.5 Billion dollars) (2007/2008 GitHub founded) (2022 55M users)

GitHub's Main Features

- Code
- 