# ci

Learning project to get Git Hub to
- Perform checks before pull requests can be merged
- Run tests
- Embed a release identifier in the release so the application can
  report its version without a manual step
- Build a release

To push, etc. without needing to enter the username and personal access token every time, you can change from https to ssh. You'll need to tell Git Hub your public key, and change the remote origin to use the `git@github.com` project address.

## Push without manually entering a Personal Authentication Token every time

Put your SSH public key [here](https://github.com/settings/keys) and

```
git remote set-url origin git@github.com:pictographer/ci.git
```
