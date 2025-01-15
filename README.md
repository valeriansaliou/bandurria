Bandurria
=========

**Bandurria is a self-hosted lightweight comment system that for static websites and blogs. Built in Rust, takes a few MB of RAM. Included with a single line of JavaScript.**

Bandurria provides no administration interface. It solely relies on email notifications for moderation and Magic Links for identity verification. It also does not provide any CSS preset, only CSS classes in its injected HTML that you can freely style to match your blog or website style.

Spam is prevented by requiring user browsers to submit the result to a Proof of Work challenge (based on an improved variant of Hashcash), while the user is typing their comment. This spam prevention method is CAPTCHA-free and hassle-free, since the proof will already be computed when the user will be ready to submit their comment. Upon submission of their comment, the user will receive a Magic Link over email they will need to click on to confirm their identity and submit their comment. Then, you (the administrator) will receive the user comment over email for moderation. If the user has already sent approved comments in the past under the same email, then their comment will be auto-approved. If not, you will need to approve the comment which will also trust the user. Either way, Bandurria notifies people of new replies to their comments over email.

The Bandurria name refers to the _Bandurria Austral_ ([Black-faced Ibis](https://en.wikipedia.org/wiki/Black-faced_ibis)), which is a bird that can be found across Patagonia. It emits interesting [metallic sounds](https://www.youtube.com/watch?v=S5iLNFumfFM).

ℹ️ _This project has been started after I used another similar comment system on my [personal blog](https://valeriansaliou.name/blog/), named [Schnack](https://schnack.cool), that requires users to authenticate through OAuth (eg. via Google) before they can send their comment. I have noticed that requiring to OAuth to a Google or GitHub account to send a comment on a random blog (where user trust is possibly low), might discourage a lot of people from commenting._

_Tested at Rust version: `rustc 1.79.0 (129f3b996 2024-06-10)`_

**🇨🇱 Crafted in Santiago, Chile.**

## Features

* **Built-in spam control** with Proof of Work anti-bot system and Magic Link email verification
* **Email-based notifications** and comment moderation (no complex Web admin UI)
* **Zero-dependencies server runtime** and **lightweight JavaScript script**
* **Compatible with any static website** or blog system (as long as your can add 1 line of JavaScript)
* **Customize it in a few lines of CSS** to match your website or blog style

## How to use it?

### Installation

**Install from Docker Hub:**

You might find it convenient to run Bandurria via Docker. You can find the pre-built Bandurria image on Docker Hub as [valeriansaliou/bandurria](https://hub.docker.com/r/valeriansaliou/bandurria/).

First, pull the `valeriansaliou/bandurria` image:

```bash
docker pull valeriansaliou/bandurria:v1.0.0
```

Then, provide a configuration file and run it (replace `/path/to/your/bandurria/config.cfg` with the path to your configuration file):

```bash
docker run -p 8080:8080 -v /path/to/your/bandurria/config.cfg:/etc/bandurria.cfg valeriansaliou/bandurria:v1.0.0
```

In the configuration file, ensure that:

* `server.inet` is set to `0.0.0.0:8080` (this lets Bandurria be reached from outside the container)
* `assets.path` is set to `./res/assets/` (this refers to an internal path in the container, as the assets are contained there)

Bandurria will be reachable from `http://localhost:8080`.

**Install from binary:**

A pre-built binary of Bandurria is shared in the releases on GitHub. You can simply download the latest binary version from the [releases page](https://github.com/valeriansaliou/bandurria/releases), and run it on your server.

You will still need to provide the binary with the configuration file, so make sure you have a Bandurria `config.cfg` file ready somewhere.

_The binary provided is statically-linked, which means that it will be able to run on any Linux-based server. Still, it will not work on MacOS or Windows machines._

👉 _Each release binary comes with an `.asc` signature file, which can be verified using [@valeriansaliou](https://github.com/valeriansaliou) GPG public key: [:key:valeriansaliou.gpg.pub.asc](https://valeriansaliou.name/files/keys/valeriansaliou.gpg.pub.asc)._

**Install from Cargo:**

If you prefer managing `bandurria` via Rust's Cargo, install it directly via `cargo install`:

```bash
cargo install bandurria
```

Ensure that your `$PATH` is properly configured to source the Crates binaries, and then run Bandurria using the `bandurria` command.

**Install from source:**

The last option is to pull the source code from Git and compile Bandurria via `cargo`:

```bash
cargo build --release
```

You can find the built binaries in the `./target/release` directory.

### Configuration

Use the sample [config.cfg](https://github.com/valeriansaliou/bandurria/blob/master/config.cfg) configuration file and adjust it to your own environment.

You can also use environment variables with string interpolation in your configuration file, eg. `server.inet = ${BANDURRIA_INET}`.

**Available configuration options are commented below, with allowed values:**

**[server]**

* `log_level` (type: _string_, allowed: `debug`, `info`, `warn`, `error`, default: `error`) — Verbosity of logging, set it to `error` in production

### Run Bandurria

Bandurria can be run as such:

`./bandurria -c /path/to/config.cfg`

## :fire: Report A Vulnerability

If you find a vulnerability in Bandurria, you are more than welcome to report it directly to [@valeriansaliou](https://github.com/valeriansaliou) by sending an encrypted email to [valerian@valeriansaliou.name](mailto:valerian@valeriansaliou.name). Do not report vulnerabilities in public GitHub issues, as they may be exploited by malicious people to target production servers running an unpatched Bandurria instance.

**:warning: You must encrypt your email using [@valeriansaliou](https://github.com/valeriansaliou) GPG public key: [:key:valeriansaliou.gpg.pub.asc](https://valeriansaliou.name/files/keys/valeriansaliou.gpg.pub.asc).**
