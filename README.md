# spike-a1hilfe-mdbook

## Dev machine

Currently we don't have a reference workstation with a prefered setup. There are several options to get this running. But for now, the one with less friction with our administration involves Hyper-V. Go ahead, give it a try.

A general gist with Hyper-V:

- create a VM - Windows or Linux, your choice
- [install Rust](https://rust-lang.org/tools/install/) - depends on your VM, don't follow the one below which is designed to do pipeline work!
- install mdbook and its extensions - hint, for dev machine the --root parameter ist not needed
  ```sh
  cargo install mdbook
  cargo install mdbook-pdf
  cargo install mdbook-gitinfo
  ```
- additionally for PDF generation:
  - if you are using Linux, install Chromium - the open-source version of Chrome!
  - if you are using Windows most likely Edge is already installed, so probably nothing to do here

If you want to run the website with a container - assuming we are using a VM described above

- on Windows install Podman, on Linux either Podman or Docker works

**Caution** - in our case, Docker for Windows requires a license!

There are other ways that includes a local setup or WSL2, but those need to be approved and prepared.

When a setup is decided, a detailed instruction / notification will be released.

## Communication with the repository

I highly recommend to use [SSH](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/about-ssh)

[How to clone a repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository) - here again, prefer SSH

## Pipeline

Instruction how to deal with action runner, well ... look it up, it is documented pretty good! :-)

Following instructions are based on an Ubuntu 24.04 host for CI/CD work.

- configure Rust

  ```sh
  # install rustup with snap
  sudo snap install rustup --classic

  # install rust with rustup
  rustup install stable
  ```

- install a cargo binary with --root parameter

  ```sh
  cargo install mdbook --root /opt/cargo
  cargo install mdbook-pdf --root /opt/cargo
  cargo install mdbook-gitinfo --root /opt/cargo

  # for convenience - with this, we can update ALL cargo extensions at once
  cargo install cargo-update --root /opt/cargo
  ```

- add the above cargo/bin to PATH globally

  ```sh
  # root required
  # create a script in /etc/profile.d
  sudo touch /etc/profile.d/register-opt-cargo-bin.sh
  ```

  Content of the file

  ```txt
  export PATH=$PATH:/opt/cargo/bin
  ```

  Or short

  ```sh
  sudo echo "export PATH=$PATH:/opt/cargo/bin" > /etc/profile.d/register-opt-cargo-bin.sh
  ```

### For mdbook-pdf

Puppeteer uses Chrome, following dependencies are needed to set up Chrome properly:

- unzip
- libnspr4
- libnss3
- libatk1.0-0
- libatk-bridge2.0-0t64
- libasound2t64
- libgdm1
- libgbm1
- libcairo2
- libpango-1.0-0
- libxcomposite1
- libxdamage1
- libxfixes3
- libxrandr2

Here, we don't simply install Chromium / Chrome machinewide. The pipeline is doing it LOCALLY for an actual job.
