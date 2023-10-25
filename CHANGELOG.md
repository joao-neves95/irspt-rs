# CHANGELOG

## v0.4.1
- [refactor]: Multiple internal improvements and refactorings.

## v0.4.0
- [fix]: Fixed the "Portal das Finanças" authentication, according to the new online platform changes.
- [fix]: Updated the new issue invoice URL.
- [feat]: Auto-run the webdriver at startup, if needed.
- [refactor]: Internal code refactoring.
- [chore]: Updated multiple dependencies.

## v0.3.0
- [refactor]: Now using traits over multiple structs for the `irspt-api` structure.\
              This is a breaking change, but no one uses this program nor its libs, so the versions were
              all regressed to pre-public denote the alpha state and that breaking changes can happen.
- [chore]: Updated package dependencies.

## v0.2.0
- [feat]: Added support for multiple template schema versions.\
          This is a breaking change, but because I am not providing a binary and no one uses this program,
          I am not going to provide any migration functionality.
- [refactor]: Multiple other internal refactorings and optimizations.
- [chore]: Updated multiple dependencies.

## v0.1.0
- Initial release.
