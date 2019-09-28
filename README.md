# project-view

[![Build Status](https://travis-ci.org/CMJones001/project-view.svg?branch=develop)](https://travis-ci.org/CMJones001/project-view)

# Installation

Installing this program is as simple as installing rust,
https://www.rust-lang.org/tools/install, and then running ``cargo build
--release`` in this directory.

## Quick start

As this is in alpha development, the configuration values are hard coded for
now, see ``src/main.rs`` for experiment parts that we are using. 

Then run ``./target/relase/project_summary`` as usual.

# Outline

Academic software spends most of its lifetime in an active development
cycle. We therefore introduce this tool to keep track of the users data in
comparison to the stage of the source code, allowing us to see if any stages
of analysis are out of date due to changes in format or method.

Not only do we want to check whether data is present, but also if it was
created before a breaking change was introduced that makes it unsuitable
for further analysis. This may be caused by a change in the data format,
or an improved method where we do not wish to "contaminate" our new
analysis.

For now we simply report on the status of the data, but we wish to extend
this to allow automatic running of the relevant scripts.

## Data structure

Our common dataflow boils down to a large set of initial data that requires
multiple stages of analysis. The stages of this analysis are organised into
sub-directories of a main experiment directory (not including the initial
data).

In some cases, there may be multiple experiment directories, this may
correspond to different starting data or different approaches in analysis.

## Reproducibilty

For reproducibilty each of these Experiment directories should contain a
configuration file that contains any of the metadata on the run, such as
parameters or methods used in the analysis. Therefore, a user should be able
to recreate the analysis using only the initial data and this configuration
file.

This program aims to automate this final step, not only for the initial
author of the analysis during development but any later users of the
software.

## Goals

  - \[1/3\] Configuration file for the dataflow
      - \[X\] Regex and location of data files
      - \[ \] Linking the data files in order
      - \[ \] Tools needed at each step
  - \[1/4\] Source code comparison How up to date is the data in
    comparison to the source code?
      - \[X\] Get the most recently updated file matching the regex in
        given directory
      - \[ \] Check for consistency in directory Are any of the files
        significantly out of date? We need to decide on a consistent
        definition that adapts to the creation rates of the other files
          - This would be a good way to look at time series and
            frequency data, if most of the files take 10 seconds to
            create, then any files that are created a few hours before
            the last are likely out of date.
      - \[ \] Compare this to the source code
          - \[X\] Get the last commit before a given time
          - \[X\] Parse the commit information
          - \[ \] Get the a measure of difference between the data
            creation and current source.
              - The exact measure is unclear
              - \[X\] Number of commits on all branches
                  - While this is simplest it is perhaps not too
                    relevant as we have no idea when a breaking change
                    is introduced.
              - \[ \] Semantic versioning
                  - This would be the most ideal, as we have a clear
                    definition of what is a breaking change, but this
                    requires a lot of assumptions about the code.
                  - How do we deal with v0.x.y changes as most of these
                    are assumed to be breaking.
              - \[ \] Number of merges into master or develop
                  - This also requires a number of assumptions, but is
                    hopefully adapted by most developers
      - \[ \] Generalise for many programs/tools
          - Currently a lot of our work is based around a single
            package/project at time, but bio-informatics problems are
            based around many tools.
          - Probably don't need to reinvent \`\`snake-make\`\` or
            similar.
  - \[X\] Set up Travis
      - This project is also a test of continuous integration, so we
        have proof that this is portable to other systems. This will be
        more important if we bring in GUI components.
