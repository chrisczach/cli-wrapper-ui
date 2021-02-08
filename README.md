Experiment wrapping cli helper scripts with UI in a generic way.

Ideally, this would be done with a manifest file that would be a recursive data structure that mirrors the different states for the helper scripts.

For the POC just try to to handle starting a script, showing stdout while running, return to main menu on exit 0, and valid error state for other exit codes.

Ideally multi-step cli scripts and tooling would be able to be represented in the manifest and have added ui states generated automatically from the manifest.