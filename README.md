# project-view

## Outline

This is a basic data pipeline created as a hobby project, partly to help
organise our projects. As happens when developing academic software, we
assume that we have we have a rapidly changing source code, and we wish
to ensure not only that we have a reasonable data pipeline.

Not only do we want to check whether data is present, but also if it was
created before a breaking change was introduced that makes it unsuitable
for further analysis. This may be caused by a change in the data format,
or an improved method where we do not wish to "contaminate" our new
analysis.

## Goals

  - \[0/3\] Configuration file for the dataflow
      - \[ \] Regex and location of data files
      - \[ \] Linking the data files in order
      - \[ \] Tools needed at each step
  - \[0/4\] Source code comparison How up to date is the data in
    comparison to the source code?
      - \[ \] Get the most recently updated file matching the regex in
        given directory
      - \[ \] Check for consistency in directory Are any of the files
        significantly out of date? We need to decide on a consistent
        definition that adapts to the creation rates of the other files
          - This would be a good way to look at time series and
            frequency data, if most of the files take 10 seconds to
            create, then any files that are created a few hours before
            the last are likely out of date.
      - \[ \] Compare this to the source code
          - \[ \] Get the last commit before a given time
          - \[ \] Parse the commit information
          - \[ \] Get the a measure of difference between the data
            creation and current source.
              - The exact measure is unclear
              - \[ \] Number of commits on all branches
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
  - \[0/0\] Set up Travis
      - This project is also a test of continuous integration, so we
        have proof that this is portable to other systems. This will be
        more important if we bring in GUI components.
  - Language choice, I would like to try this in rust, to get more
    practice, however this is not currently supported on the server.
