Bandurria
=========

**Bandurria is a self-hosted lightweight comment system that for static websites and blogs. Built in Rust, takes a few MB of RAM. Included with a single line of JavaScript.**

Bandurria provides no administration interface. It solely relies on email notifications for moderation and Magic Links for identity verification. It also does not provide any CSS preset, only CSS classes in its injected HTML that you can freely style to match your blog or website style.

Spam is prevented by requiring user browsers to submit the result to a Proof of Work challenge (based on an improved variant of Hashcash), while the user is typing their comment. This spam prevention method is CAPTCHA and hassle free, since the proof will already be computed when the user will be ready to submit their comment. Upon submission of their comment, the user will receive a Magic Link over email they will need to click on to confirm their identity and submit their comment. Then, you (the administrator) will receive the user comment over email for moderation. If the user has already sent approved comments in the past under the same email, then their comment will be auto-approved. If not, you will need to approve the comment which will also trust the user. Either way, Bandurria notifies people of new replies to their comments over email.

The Bandurria name refers to the _Bandurria Austral_ (Black-faced Ibis), which is a bird that can be found across Patagonia. It emits interesting [metallic sounds](https://www.youtube.com/watch?v=S5iLNFumfFM).

_Tested at Rust version: `rustc 1.79.0 (129f3b996 2024-06-10)`_

**🇨🇱 Crafted in Santiago, Chile.**

## :fire: Report A Vulnerability

If you find a vulnerability in Bandurria, you are more than welcome to report it directly to [@valeriansaliou](https://github.com/valeriansaliou) by sending an encrypted email to [valerian@valeriansaliou.name](mailto:valerian@valeriansaliou.name). Do not report vulnerabilities in public GitHub issues, as they may be exploited by malicious people to target production servers running an unpatched Bandurria instance.

**:warning: You must encrypt your email using [@valeriansaliou](https://github.com/valeriansaliou) GPG public key: [:key:valeriansaliou.gpg.pub.asc](https://valeriansaliou.name/files/keys/valeriansaliou.gpg.pub.asc).**
