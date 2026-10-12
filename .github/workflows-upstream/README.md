# Upstream workflows (disabled)

These are the CI and release workflows inherited from the original project. They sit outside
`.github/workflows/` on purpose, so GitHub does not run them on this fork (several run nightly and
some call reusable workflows from `storytold/craft-common-actions`). Move one back to
`.github/workflows/` to enable it.
