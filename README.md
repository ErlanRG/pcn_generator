<p>
    <h1 align="center">PCN directory & template creator</h1>
</p>

## DISCLAIMER

**THIS IS AN EXPERIMENTAL PROJECT.**

 Please bear with me 

Use this as a productivity tool and not as part of the official process.

This project has two main goals in mind:

1. Save me some time while working on the
   PCN process and avoid tedious and manual processes.

2. I find that programming is one of my favourites (kind of) hobbies to kill
   some personal time.
   Plus, as a software engineering student, this is just another project I am
   working on to get better and to learn new things.

Because of these two reasons, you may find one or two bugs with the program.
If you do, feel free to report the issues you encounter so I can work on the patches.

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

## Usage

### TL;DR

The program will create a PCN structure for you inside the Documents directory
and automatically place the `PCN_template.xlsx` file (for CEs) inside every
single affected part by the PCN.

Should look something like this:

```
Documents/
├─ PCN/
│  ├─ PCN_template.xlsx
│  ├─ PCN00002/
│  │  ├─ affected_part1/
│  │  │  ├─ affected_part1.xlsx
│  ├─ PCN00001/
│  │  ├─ affected_part1/
│  │  │  ├─ affected_part1.xlsx
│  │  ├─ affected_part2/
│  │  │  ├─ affected_part2.xlsx
│  │  ├─ affected_part3/
│  │  │  ├─ affected_part3.xlsx
```

### Detailed explanation. (Please read).

1. The program will first check your Documents directory for a subdirectory called
   "PCN". This is where all the PCN subdirectories will be created.

- On Windows: `C:\Users\<username>\Documents\PCN`.
- On Linux: `$HOME/Documents/PCN`.

If this PCN directory does not exist, the program will ask you if you want to
create it. As of now, this is the PATH I chosed for everything to be stored in, so
if you decide to not create the directory, then the program will exit.

2. The program will then ask you for the PCN case number. This is usually something
   like "PCNXXXXXX". You can call it whatever you like, but this is usually the
   most eficient way to identify a case. If the PCN case already exist, then
   the program will print an error message and won't create the new directory.

3. After successfully creating the PCN case, it will ask you to enter a list of
   affected parts. Now, it is important to point-out that the list should be in a
   **comma-separated format.** Should look like something like this:

`RES1234, RES2345, RES3454`

Don't worry about the whitespaces, it will handle them automatically.
The program will ask you again for this input if it finds a duplicate in the
list.

4. As seen on the structure shown above, it is recommended to have a
   `PCN_template.xlsx` file for CEs inside `C:\Users\<username>\Documents\PCN`.
   This is because the program will look for this file and copy it inside every
   single affected part directory. Then, it will rename this file as the
   affected part with the `.xlsx` extension.

You can now work on the PCNs that should be ready to upload once finished

## TODO list

- [ ] Ask the user for the desired location different than `Documents/PCN`
- [ ] Do not exit when the PCN case number is duplicated. Instead, prompt to enter it again.
- [ ] Include the `PCN_template.xlsx` file along with the installation and place it inside the PCN directory.
