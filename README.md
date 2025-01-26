Bandurria
=========

[![Test and Build](https://github.com/valeriansaliou/bandurria/actions/workflows/test.yml/badge.svg)](https://github.com/valeriansaliou/bandurria/actions/workflows/test.yml) [![Build and Release](https://github.com/valeriansaliou/bandurria/actions/workflows/build.yml/badge.svg)](https://github.com/valeriansaliou/bandurria/actions/workflows/build.yml) [![dependency status](https://deps.rs/repo/github/valeriansaliou/bandurria/status.svg)](https://deps.rs/repo/github/valeriansaliou/bandurria) [![Buy Me A Coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-donate-yellow.svg)](https://www.buymeacoffee.com/valeriansaliou)

**Bandurria is a self-hosted lightweight comment system for static websites and blogs. Built in Rust, it consumes only 4MB of RAM. Can be included with a single line of JavaScript (8KB!).**

---

<img align="left" height="208" src="https://valeriansaliou.github.io/bandurria/images/bandurria-comic.png" alt="Bandurria Comic">

This project has been started after I used another comment system on my [personal blog](https://valeriansaliou.name/blog/), named [Schnack](https://schnack.cool), that requires users to authenticate through OAuth (eg. via Google) before they can send their comment. I have noticed that requiring to OAuth to a Google or GitHub account to send a comment on a public blog might discourage a lot of people from commenting, since user trust in the blog is possibly low.

Bandurria comes as a lighter and even simpler alternative to Schnack, without Schnack's memory overhead due to NodeJS and installing 60MB+ of NPM dependencies. Bandurria is a single tiny portable binary, that comes with an [official Docker image](https://hub.docker.com/r/valeriansaliou/bandurria/).

---

👉 If you come from Schnack, migrating to Bandurria is very easy: the include script and CSS classes are very similar; you could also easily convert Schnack's SQLite database into Bandurria's MySQL database with a simple script. I'd like to thank [Schnack's authors](https://github.com/orgs/schn4ck/people) for their work, and the inspiration it's been for Bandurria.

_Tested at Rust version: `rustc 1.84.0 (9fc6b4312 2025-01-07)`_

**🇨🇱 Crafted in Santiago, Chile.**

## Live Demo

You can try Bandurria live on my [personal blog](https://valeriansaliou.name/blog/3d-printing-formlabs-enclosure-project/#comments). Bandurria is included at the end of the article. Note that I might not accept all comments going through moderation.

![Bandurria Screenshot](https://valeriansaliou.github.io/bandurria/images/bandurria-screenshot.png)

## Features

* **Built-in spam control** with a Proof of Work anti-bot system (without annoying CAPTCHAs!)
* **Email-based notifications** and Magic Link comment moderation (no complicated Web admin UI)
* **Zero-dependencies server runtime (4MB)** and **lightweight JavaScript script (8KB)**
* **Compatible with any static website** or blog system (as long as your can add 1 line of JavaScript)
* **Customize it in a few lines of CSS** to match your website or blog style

Bandurria provides no administration interface. It solely relies on email notifications for moderation and Magic Links for approving or rejecting comments. It also does not provide any built-in CSS styles, only CSS classes in its injected HTML that you can freely style to match your blog or website style. For convenience, it comes with an [example CSS](https://github.com/valeriansaliou/bandurria/blob/master/res/assets/dev/test-page/bandurria.css) you can copy and paste to start with.

Spam is prevented by requiring user browsers to submit the result to a Proof of Work challenge, based on an improved variant of [Hashcash](http://www.hashcash.org/papers/hashcash.pdf) in order to [reduce the mint time variance](https://friendlycaptcha.com/insights/controlling-variance-in-proof-of-work-algorithms/). A solution to the server challenge is computed when the user submits their comment. This spam prevention method is CAPTCHA-free and hassle-free, as it should only take a few seconds to compute under normal circumstances.

Upon submission of their comment, the user will be informed that their comment has been submitted and is awaiting moderation. Then, you (the administrator) will receive the user comment over email for moderation. Bandurria also notifies people of new replies to their comments over email (if they opted in to receive reply alerts).

**Oh and what about that name?!** Well, the Bandurria name refers to the _Bandurria Austral_ ([Black-faced Ibis](https://en.wikipedia.org/wiki/Black-faced_ibis)), which is a bird that can be found across Patagonia. It emits interesting [metallic sounds](https://www.youtube.com/watch?v=S5iLNFumfFM).

## 🚧 Work In Progress

All features might not have yet been implemented, but this is what Bandurria aims for:

- [x] No social auth, no admin interface, no multiple notification channels, do it all over email notifications with magic links.
- [x] Public users can write their comment, give a name and email and submit in a simple way (WordPress like).
- [x] Once an user first comment got approved then all further comments will be auto approved (unless the user gets banned by the admin).
- [x] Built in theme is to be generic and simple with no colors, it can be extended by the user by styling CSS classes in their own blog theme (CSS class names should be stable, and never use important rules).
- [x] Built with Rust, goal is to produce a 4MB binary using the same amount of RAM and distribute lightweight Docker images for all platforms.
- [x] Format URLs into clickable links.
- [x] Admin users can manage comments and remove or allow them from their email inbox using magic links.
- [x] Notify admin of new comments over email.
- [x] Provide ability to customize every action, button and input placeholder wordings, since there will be no internationalization, it will solely be done via configuring custom eg. button labels from the configuration file.
- [x] Upon sending the first comment for a given page, internally check that the blog page exists with a HTTP request (it should return 200), if the page already exists in database then no need to check again (this prevents inserting junk in the database).
- [x] Make comments anchorable when clicking on the comment date, and auto-scroll to the target comment when page URL has anchor.
- [x] Upon sending a comment and passing the PoW, always require administrators to moderate the comment, even if it comes from an administrator email (no-fault spam prevention).
- [x] Proof of work anti spam mechanism, with progress bar if still computing upon submitting the comment (multiple parallel hash computation), with ability to configure difficulty and parallel games.
- [x] Remove the trusted flag in the authors database table, and the verified flag in the comments database table (not used anymore).
- [x] Generate a cool drawing of a Bandurria with AI and include it in this read-me
- [ ] Notify of replies to user comments over email to users if they opted to receive replies once the comment passed moderation (enable engagement, which was an issue with other simple commenting systems since users didn’t get notified of replies to their own comments).
- [ ] Perform a static analysis and security review of the source code, especially the PoW part (once 100% of features have been implemented).
- [ ] Release first stable version (`v1.0.0`)
- [ ] Write a personal blog article about Bandurria and direct all demo URLs to this article (instead of the current one).

## How to use it?

### Installation

**Install from Docker Hub:**

You might find it convenient to run Bandurria via Docker. You can find the pre-built Bandurria image on Docker Hub as [valeriansaliou/bandurria](https://hub.docker.com/r/valeriansaliou/bandurria/).

First, pull the `valeriansaliou/bandurria` image:

```bash
docker pull valeriansaliou/bandurria:v0.5.1
```

Then, provide a configuration file and run it (replace `/path/to/your/bandurria/config.cfg` with the path to your configuration file):

```bash
docker run -p 8080:8080 -v /path/to/your/bandurria/config.cfg:/etc/bandurria.cfg valeriansaliou/bandurria:v0.5.1
```

In the configuration file, ensure that:

* `server.inet` is set to `0.0.0.0:8080` (this lets Bandurria be reached from outside the container)
* `assets.path` is set to `./res/assets/` (this refers to an internal path in the container, as the assets are contained there; _you do not need to mount a volume for this!_)

Bandurria will be reachable from `http://localhost:8080`.

**Install from binary:**

A pre-built binary of Bandurria is shared in the releases on GitHub. You can simply download the latest binary version from the [releases page](https://github.com/valeriansaliou/bandurria/releases), and run it on your server.

You will still need to provide the binary with the configuration file, so make sure you have a Bandurria `config.cfg` file ready somewhere.

_The binary provided is statically-linked, which means that it will be able to run on any Linux-based system. Still, it will not work on MacOS or Windows machines._

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
* `inet` (type: _string_, allowed: IPv4 / IPv6 + port, default: `[::1]:8080`) — Host and TCP port the Bandurria server should listen on

**[assets]**

* `path` (type: _string_, allowed: UNIX path, default: `./res/assets/`) — Path to Bandurria assets directory

**[database]**

**[database.mysql]**

* `uri` (type: _string_, allowed: MySQL connection URI, no default) — MySQL URI (format: `mysql://user:password@server:port/database`)

**[email]**

**[email.smtp]**

* `server_host` (type: _string_, allowed: hostname, IPv4, IPv6, default: no default) — SMTP host to connect to
* `server_port` (type: _integer_, allowed: TCP port, default: `587`) — SMTP TCP port to connect to
* `server_starttls` (type: _boolean_, allowed: `true`, `false`, default: `true`) — Whether to encrypt SMTP connection with `STARTTLS` or not
* `server_tls` (type: _boolean_, allowed: `true`, `false`, default: `false`) — Whether to encrypt SMTP connection with `TLS` or not
* `auth_user` (type: _string_, allowed: any string, default: no default) — SMTP username to use for authentication (if any)
* `auth_password` (type: _string_, allowed: any string, default: no default) — SMTP password to use for authentication (if any)

**[email.identity]**

* `from_name` (type: _string_, allowed: any string, default: `Comments`) — Name to send the emails from
* `from_email` (type: _string_, allowed: email address, default: no default) — Email to send the emails from

**[site]**

* `name` (type: _string_, allowed: any string, default: no default) — Name of the site
* `admin_emails` (type: _array[string]_, allowed: email addresses, default: no default) — Email addresses of site administrators
* `site_url` (type: _string_, allowed: URL, default: no default) — URL of the site
* `comments_url` (type: _string_, allowed: URL, default: no default) — URL of the comment system

**[security]**

* `secret_key` (type: _string_, allowed: any hexadecimal string, default: auto-generated secret) — Secret key to use to sign all authenticated payloads (generate yours with `openssl rand -hex 32`)
* `check_pages_exist` (type: _boolean_, allowed: `true`, `false`, default: `false`) — Whether to check over HTTP that a page a comment is left on actually exists (when sending the first comment for that page; _it is safer to enable_)

**[antispam]**

* `difficulty` (type: _integer_, allowed: any number, default: `17`) — Difficulty of the antispam PoW problem (not too low, not too high: `17` takes 3 seconds on a MacBook Pro M1 Pro, while `20` takes 21 seconds on the same device!)
* `problems_parallel` (type: _integer_, allowed: any number, default: `10`) — Number of antispam PoW problems to solve in parallel (this value should usually not be changed)
* `solutions_require` (type: _integer_, allowed: any number, default: `6`) — Number of antispam PoW problems to solve to pass the test (should be less or equal to `problems_parallel`, 60% of its value is a sweet spot, [read why](https://friendlycaptcha.com/insights/controlling-variance-in-proof-of-work-algorithms/))

**[i18n]**

* `field_write_your_comment` (type: _string_, allowed: any string, default: `Write your comment...`) — Translated string for the comment textarea
* `field_whats_your_name` (type: _string_, allowed: any string, default: `What's your name?`) — Translated string for the name input
* `field_whats_your_email` (type: _string_, allowed: any string, default: `Enter your email`) — Translated string for the email input
* `button_post_comment` (type: _string_, allowed: any string, default: `Post comment`) — Translated string for the submit button
* `button_reply` (type: _string_, allowed: any string, default: `Reply`) — Translated string for the reply button
* `label_leave_a_comment` (type: _string_, allowed: any string, default: `Leave a comment:`) — Translated string for the main label
* `banner_presubmit` (type: _string_, allowed: any string, default: `Your email is only used to check you are not a bot. It will not be stored.`) — Translated string for the pre-submit banner
* `banner_submitting` (type: _string_, allowed: any string, default: `Sending and proving you are not a bot. This might take a few seconds...`) — Translated string for the submitting banner
* `banner_submitted_important` (type: _string_, allowed: any string, default: `Your comment has been submitted.`) — Translated string for the submitted banner (important part)
* `banner_submitted_notice` (type: _string_, allowed: any string, default: `It will appear here after it gets accepted by moderation.`) — Translated string for the submitted banner (notice part)
* `banner_submiterror` (type: _string_, allowed: any string, default: `Your comment could not be submitted. Mind try again?`) — Translated string for the error banner

### Run Bandurria

In order to run Bandurria on your server and add comments to your blog or website, follow those steps in order:

#### 1. Create the SQL database

Before you can run Bandurria, you need to create its SQL database in MySQL:

1. Create your MySQL database: `CREATE DATABASE bandurria CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;`
2. Import the [MySQL database schema](https://github.com/valeriansaliou/bandurria/blob/master/doc/fixtures/bandurria.sql)
3. Adjust the Bandurria configuration file so that the configuration value at `database.mysql.uri` points to your MySQL database

#### 2. Start Bandurria

When you are ready, you can start Bandurria as such:

`./bandurria -c /path/to/bandurria/config.cfg`

If you use Docker, refer to the Docker instructions above ("_Install from Docker Hub_").

#### 3. Create a proxy rule on NGINX

On your NGINX reverse proxy, add the following location block:

```nginx
location /bandurria/ {
    proxy_pass http://localhost:8080;

    # Important: remove the '/bandurria' prefix from the URL upon proxying!
    rewrite ^/bandurria/(.*)$ /$1 break;

    proxy_http_version 1.1;

    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
}
```

If you are using a different HTTP server, the proxying configuration will be different. Please adapt it to your needs.

#### 4. Include Bandurria on your site

Now that Bandurria is running, we can include the script on our website!

**First, create the HTML container where your comments will be injected by Bandurria:**

```html
<!-- Your blog article is right before that -->

<aside id="comments" class="post-comments"></aside>
```

This container is usually placed at the end of blog articles (if you are including Bandurria on a blog).

**Then, add the Bandurria loader script right before the `</body>` closing tag:**

```html
<html>
    <body>
        <!-- The rest of your body goes here -->

        <script
            src="/bandurria/assets/embed.js"
            data-bandurria-target=".post-comments"
        ></script>
    </body>
</html>
```

We are assuming here that Bandurria is running over a reverse proxy such as NGINX, proxying the `/bandurria/` path to Bandurria's root.

**Finally, in your `<head>` section, include Bandurria's style, which you may customize to fit your own design:**

```html
<html>
    <head>
        <!-- The rest of your head goes here -->

        <link rel="stylesheet" type="text/css" href="/path/to/your/own/bandurria.css" />
    </head>
</html>
```

You may copy and paste the example [bandurria.css](https://github.com/valeriansaliou/bandurria/blob/master/res/assets/dev/test-page/bandurria.css) file that we provide, and start from there.

## :fire: Report A Vulnerability

If you find a vulnerability in Bandurria, you are more than welcome to report it directly to [@valeriansaliou](https://github.com/valeriansaliou) by sending an encrypted email to [valerian@valeriansaliou.name](mailto:valerian@valeriansaliou.name). Do not report vulnerabilities in public GitHub issues, as they may be exploited by malicious people to target production servers running an unpatched Bandurria instance.

**:warning: You must encrypt your email using [@valeriansaliou](https://github.com/valeriansaliou) GPG public key: [:key:valeriansaliou.gpg.pub.asc](https://valeriansaliou.name/files/keys/valeriansaliou.gpg.pub.asc).**
