# spike-a1hilfe-mdbook

Following instructions are based on an Ubuntu 24.04 host

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