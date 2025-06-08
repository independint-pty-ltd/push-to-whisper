# Creating Release Notes

This document provides guidelines for creating effective release notes for Push-to-Whisper releases.

## Process

1. Copy the template from `tools/RELEASE_NOTES_TEMPLATE.md` to a new file named `RELEASE_NOTES_vX.Y.Z.md` in the project root
2. Update the version number in the title and download link 
3. Update the "Full Changelog" line with the correct version numbers
4. Fill in the relevant sections based on changes since the last release

## Guidelines for Writing Good Release Notes

### Be User-Focused
- Write for the end user, not for developers
- Emphasize benefits, not technical implementation details
- Use clear, non-technical language when possible

### Categorize Changes Properly
- **New Features**: Brand new functionality that didn't exist before
- **Improvements**: Enhancements to existing features
- **Bug Fixes**: Issues that have been resolved
- **Performance**: Changes that affect speed, memory usage, or efficiency
- **Documentation**: Updates to guides, docs, or help systems

### Use Consistent Formatting
- Use bold headings for subcategories within sections
- Use bullet points for individual items
- Keep each bullet point concise (1-2 sentences)
- Start each bullet with an action verb when possible

### Provide Sufficient Detail
- Be specific enough that users understand what changed
- Include context for why the change matters
- For bug fixes, briefly describe both the problem and solution

### Review Before Publishing
- Check for spelling and grammar errors
- Ensure all significant changes are included
- Verify version numbers and links are correct
- Have someone else review the notes if possible

## Example

Instead of:
```
Fixed bug in the UI
```

Write:
```
Fixed issue where the recording button would remain highlighted after stopping a recording session
```

Instead of:
```
Added new feature X
```

Write:
```
**Keyboard Shortcuts**: Added customizable keyboard shortcuts for starting/stopping recording and accessing settings
```

## Final Steps

After completing the release notes:
1. Commit the file to the repository
2. Use the content when creating the GitHub release
3. Include the same content in any distribution announcements 