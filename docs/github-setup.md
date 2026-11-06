# GitHub setup status

As of 2026-11-06, the local repository is on `main`, tracking `origin/main`, with origin set to `git@github.com:Varg-sec/libaff4.git`. The prepared issue breakdown was approved on 2026-11-04. The scaffold and issue drafts remain untracked. Remote API access and issue publication have not succeeded.

This session still cannot write Git metadata: staging fails because `.git/index.lock` is on a read-only filesystem. `gh repo view Varg-sec/libaff4` fails to connect to api.github.com. `gh auth status` also reports an invalid token, but authentication must be rechecked with working network access.

The configured target is `Varg-sec/libaff4`; preserve its existing visibility. “GitHub project” is interpreted as a repository with issues; no separate Projects board is assumed.

## Resume setup

From a session with writable Git metadata and working authenticated GitHub access:

1. Verify authenticated access to `Varg-sec/libaff4`. Backlog review was approved by the user on 2026-11-04.
2. Use the existing local Git repository on `main`; restore write access to Git metadata through session permissions.
3. Stage the project files, review, and commit the scaffold and backlog.
4. Push the commit to the existing origin; do not create a replacement repository.
5. Create labels `type:afk`, `type:hitl`, `status:planned` and area labels from the manifest.
6. Publish issue bodies from `docs/issues.json` in listed order using `gh issue create --repo Varg-sec/libaff4 --title TITLE --body-file FILE`. Replace local dependency IDs with the already-created GitHub issue URLs in temporary rendered bodies. Preserve source drafts and record the ID-to-URL map. Add type, area and planned labels. Before retrying, check existing issue bodies for their unique Local ID to avoid duplicates.
7. Replace local backlog links with published issue URLs and record the repository link in the README. Verify every published issue and dependency before declaring setup complete.

The issue bodies contain no parent issue because this plan originated in conversation. No project license was assumed. The to-issues review requirement has been satisfied by the user’s instruction to proceed on 2026-11-04.
