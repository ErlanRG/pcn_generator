<p>
    <h1 align="center">PCN directory & template creator</h1>
</p>

## DISCLAIMER

**THIS IS AN EXPERIMENTAL PROJECT.**

 Please bear with me 

Use this as a productivity tool.

This project has two main goals in mind:

1. Save me some time while working on the
   PCN process and avoid tedious and manual processes.

2. I find that programming is one of my favourites (kind of) hobbies to kill
   some personal time.
   Plus, as a software engineering student, this is just another project I am
   working on to get better and to learn new things.

Because of these two reasons, you may find one or two bugs with the program.
If you do, feel free to report the issues you encounter so I can work on the patches.

However, and since NO ONE asked me to this and it is not an official Teradyne
process, I am **_not_** forced to keep developing this tool to comply with all the demands.

## Motivation

Sometimes it is required to create local directories to store multiple
templates for the parts affected by a Part Change Notification. This workflow
consist in **_manually_** create a directory with the name of the PCN case number, then
for every single affected part in the PCN case, one sould create a PCN template
file (usually an Excel document).

There is a chance to find a PCN case with a large number of affected parts, so
this process just becomes inefficient and tedious, without mentioning the amount
of time you should spend on this over and over again.

In order to improve the time of invested in a single case, I have created this
program that automatically do this for you.
And the good thing is it only requires a few steps to get the PCN environment
set up in your computer.

## Usage

Using this tool should be pretty straightforward. Your only inputs are:

1. The PCN case number: you should be able to find it in Teamcenter.
2. List of affected parts (should be formatted. More on this later).

The default directory in which the new PCN directories will be created is the following:
`C:\Users\<your_user>\Documents\PCN`. If this path is not in your computer, this program
will automatically create it for you.

## TODO list
