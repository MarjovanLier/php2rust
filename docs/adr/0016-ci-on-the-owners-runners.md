# Run CI on the owner's server-140 runners, and a fork's pull request on hosted runners

CI runs on two self-hosted runners on the owner's home server, server-140, registered for this repository only as `server-140-php2rust-1` and `server-140-php2rust-2`, with the labels `self-hosted`, `linux`, `x64`, `docker` and `php2rust`. They run as the `github-runner` user, which has its own rootless Docker daemon and cannot reach the daemons that run the server's other services. Each job builds the image of `.github/ci/Dockerfile` for its PHP version (the official `php:<version>-cli` image with git, unzip, curl, `mbstring` where the image lacks it, Composer 2.10.3 and the Rust toolchain of `rust-toolchain.toml`) and runs the tests in it, with named volumes that keep Cargo's downloads and each version's build between runs. The owner asked for CI on that server on 28 Nov 2026.

The repository is public, and GitHub warns that a self-hosted runner on a public repository runs the code of any pull request that reaches it. Two measures keep a stranger's code off the server: the jobs of a pull request from a fork choose `ubuntu-latest` instead of the self-hosted labels (`github.event.pull_request.head.repo.fork`), and Actions requires the owner's approval before it runs any workflow of an outside contributor (`approval_policy: all_external_contributors`, set the same day). The second covers a fork that edits the workflow to name the self-hosted labels, which the first cannot, because a pull request runs its own copy of the workflow. The owner chose these measures over making the repository private or keeping CI on hosted runners.

Facts this rests on, checked on 28 Nov 2026: server-140 runs Ubuntu 26.04.1 with 12 cores, 21 of 46 GB of memory in use and 140 GB free, and already hosts three runners for the Twin Stories repositories under the same user. The runners are version 2.338.2, checked against the release's SHA-256. The images `php:8.3-cli` to `php:8.6-cli` gave PHP 8.3.37, 8.4.28, 8.5.13 and 8.6.0; the 8.6 image lacks `mbstring`, which the Dockerfile builds. The first build of the 8.6 image took 77 s; later builds reuse Docker's layer cache.

## Considered Options

- GitHub's hosted runners for everything: rejected by the owner. Each push runs four jobs of 8 to 10 minutes there, and the account's hosted runners stopped starting for another repository in August 2026 on a spending limit.
- Make the repository private, as the Twin Stories repositories are: rejected by the owner, who keeps php2rust public.
- `container:` jobs on the official PHP images: rejected, because the 8.6 image lacks `mbstring` and a job cannot add an extension before its steps run in the container; a Dockerfile the job builds can.
- One runner: rejected, because a runner takes one job at a time, and two let the four PHP versions run two at a time.

## Consequences

- A run of the owner's branches leaves its images and volumes on server-140; the volumes grow with each version's build and need a prune now and then, as the Twin Stories runner's did.
- A fork's pull request runs the same Dockerfile and steps on a hosted runner, so the checks mean the same on both.
- If server-140 is down, the owner's pull requests wait in the queue; nothing falls back to hosted runners on its own.
