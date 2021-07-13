# auto-class-joiner

A simple program written in Rust to join classes automatically.

Credit to [tduck973564](https://github.com/tduck973564) for refactoring the code and building for Linux.

## Installation

### Windows

Steps:

-   Download the [latest release](https://github.com/KineticTactic/auto-class-joiner/releases) (release_x64-windows.zip).
-   Unzip the zip file to some location.
-   Right click `auto-class-joiner.exe` and click "Create Shortcut"
-   Cut the new shortcut that was created
-   Press `Win + R` and enter `shell:startup`
-   Paste the shortcut in the folder that opens

The program should start from the next boot. It should display a notification if it started successfully.

### Linux

I have full faith that you know what you are doing.

## Configuration

-   Open the folder where you unzipped the folder
-   You should see a folder named `data` containing 3 files
-   Open `classes.json` and fill in your classtimetable. **Make sure same subjects have the exact same name (case sensitive)**
-   Open `links.json` and fill in the links for your classes. **You can add or remove classes according to your needs. Make sure the subject names match those in the `classes.json`**.
-   Open `timings.json` and fill in the timings for your classes.

If you did everything correctly, you should be good to go :D

_Note: Changes in `timings.json` only apply after a restart_
