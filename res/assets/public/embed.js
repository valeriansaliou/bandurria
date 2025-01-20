// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

// Try Bandurria: https://github.com/valeriansaliou/bandurria

(function () {
  // Acquire context
  var embed_script = document.currentScript;
  var embed_path = "/assets/embed.js";

  // Define methods
  var load_comments = function (options) {
    fetch(
      options.base_url +
        "/page/comments/?" +
        new URLSearchParams({
          page: options.page_path,
        }).toString(),
      {
        method: "GET",

        headers: {
          Accept: "text/html",
        },
      },
    )
      .then(function (response) {
        return response.text();
      })
      .then(function (html) {
        var document = new DOMParser().parseFromString(html, "text/html");

        inject_document(options, document);
      });
  };

  var inject_document = function (options, document) {
    // Read form template
    var form_template = document.body.querySelector(
      ".bandurria-template--form",
    );

    // Inject form
    inject_form(form_template, document.body.querySelector(".bandurria-form"));

    // Bind comment events
    bind_comment_events(
      form_template,
      document.body.querySelector(".bandurria-comments"),
    );

    // Inject all document contents
    while (document.body.firstChild) {
      options.target.appendChild(
        document.body.removeChild(document.body.firstChild),
      );
    }
  };

  var show_banner = function (form, name) {
    form.querySelector(".bandurria-banner").style.display = "none";
    form.querySelector(".bandurria-banner--" + name).style.display = "block";
  };

  var inject_form = function (form_template, form, autofocus) {
    // Append form
    form.appendChild(form_template.content.cloneNode(true));

    // Bind events on form
    bind_form_events(form, autofocus);
  };

  var submit_form = function (form, identity, button, payload) {
    // Submit comment
    fetch(
      options.base_url +
        "/api/comment/?" +
        new URLSearchParams({
          page: options.page_path,
        }).toString(),
      {
        method: "POST",
        body: JSON.stringify(payload),

        headers: {
          "Content-Type": "application/json",
        },
      },
    )
      .then(function (response) {
        if (!response.ok) {
          return Promise.error("Submit error");
        }

        identity.style.display = "none";
        button.style.display = "none";

        show_banner(form, "submitted");
      })
      .catch(function () {
        button.disabled = false;

        show_banner(form, "submiterror");
      });
  };

  var bind_form_events = function (form, autofocus) {
    var textarea = form.querySelector("textarea[name='comment_text']"),
      input_name = form.querySelector("input[name='comment_name']"),
      input_email = form.querySelector("input[name='comment_email']"),
      identity = form.querySelector(".bandurria-identity"),
      button = form.querySelector("button");

    textarea.onkeyup = function () {
      button.disabled = textarea.value ? false : true;
    };
    input_name.onkeyup = input_email.onkeyup = function () {
      button.disabled = input_name.value && input_email.value ? false : true;
    };

    form.onsubmit = function (event) {
      event.preventDefault();

      if (textarea.value) {
        button.disabled = true;

        if (input_name.value && input_email.value) {
          // Submit comment form
          input_name.disabled = true;
          input_email.disabled = true;

          submit_form(form, identity, button, {
            name: input_name.value,
            email: input_email.value,
            text: textarea.value,
            reply_to: form.dataset.replyTo || null,
          });
        } else {
          // Require user to provide their name and email
          textarea.disabled = true;

          identity.style.display = "inline-block";

          show_banner(form, "presubmit");

          input_name.focus();
        }
      }
    };

    if (autofocus === true) {
      textarea.focus();
    }
  };

  var bind_comment_events = function (form_template, comments) {
    for (var comment_reply of comments.querySelectorAll(".bandurria-reply")) {
      comment_reply.onclick = function (event) {
        var form = event.target.parentNode;

        event.target.remove();

        inject_form(form_template, form, true);
      };
    }
  };

  // Read options
  var options = {
    base_url: embed_script.src.replace(embed_path, ""),
    page_path: window.location.pathname,
    target: document.querySelector(embed_script.dataset.bandurriaTarget),
  };

  // Load comments? (injection target is defined)
  if (options.target) {
    load_comments(options);
  } else {
    console.error(
      "Could not initialize Bandurria: does bandurria-target exist?",
    );
  }
})();
