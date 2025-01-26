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
  var worker_mint_path = "/assets/workers/mint.js";

  // Define states
  var load_fired = false;

  // Define methods
  var request_api = function (action, payload) {
    return fetch(
      options.base_url +
        "/api/" +
        action +
        "/?" +
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
    ).then(function (response) {
      if (!response.ok) {
        return Promise.error("API error: " + action);
      }

      return response.json();
    });
  };

  var load_comments = function (options) {
    // Safety: assert that we did not load twice
    if (load_fired === true) {
      throw new Error("Comments load already fired. Cannot load twice!");
    }

    load_fired = true;

    // Fetch comments
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
        if (!response.ok) {
          return Promise.reject(
            response.status
              ? response.status + " " + response.statusText
              : response,
          );
        }

        return response.text();
      })
      .then(function (html) {
        var document = new DOMParser().parseFromString(html, "text/html");

        inject_document(options, document);
      })
      .catch(function (error) {
        console.error(
          "[Bandurria] Could not load comments: is the database healthy?",
          error,
        );
      });
  };

  var inject_document = function (options, document) {
    // Read form template
    var form_template = document.body.querySelector(
      ".bandurria-template--form",
    );

    // Inject form
    inject_form(form_template, document.body.querySelector(".bandurria-form"));

    // Bind comment events (if any)
    var comments = document.body.querySelector(".bandurria-comments");

    if (comments) {
      bind_comment_events(form_template, comments);
    }

    // Localize datetimes
    localize_datetimes(document.body.querySelectorAll("[data-datetime]"));

    // Inject all document contents
    while (document.body.firstChild) {
      options.target.appendChild(
        document.body.removeChild(document.body.firstChild),
      );
    }

    // Fire dummy hash change event? (if we have comments)
    // Notice: this will auto-detect if an anchor is set on URL upon loading, \
    //   and no nothing otherwise.
    if (comments) {
      handle_comment_anchor_change(comments, true);
    }
  };

  var show_banner = function (form, name) {
    for (var banner of form.querySelectorAll(".bandurria-banner")) {
      banner.style.display = "none";
    }

    if (name) {
      form.querySelector(".bandurria-banner--" + name).style.display = "block";
    }
  };

  var inject_form = function (form_template, form, autofocus) {
    // Append form
    form.appendChild(form_template.content.cloneNode(true));

    // Bind events on form
    bind_form_events(form, autofocus);
  };

  var submit_form = function (form, identity, button, payload) {
    show_banner(form, "submitting");

    request_api("challenge", payload)
      .then(function (challenge) {
        payload.comment_id = challenge.data.comment_id;
        payload.attestation = challenge.data.attestation;

        // Mint solutions
        return mint_challenge_solutions(
          challenge.data.problems,
          challenge.data.difficulty_expect,
          challenge.data.solutions_expect,
        );
      })
      .then(function (solutions) {
        payload.mints = solutions;

        // Submit comment
        return request_api("comment", payload);
      })
      .then(function () {
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

      show_banner(form, null);

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

    window.addEventListener("hashchange", function () {
      handle_comment_anchor_change(comments, false);
    });
  };

  var handle_comment_anchor_change = function (comments, scroll_to) {
    if ((location.hash || "").startsWith("#comment-") === true) {
      var anchored_class = "bandurria-comment--anchored";

      // Clear existing anchor (if any)
      var anchored_comment = comments.querySelector("." + anchored_class);

      if (anchored_comment) {
        anchored_comment.classList.remove(anchored_class);
      }

      // Add new anchor (if comment found)
      var anchor_comment = comments.querySelector(location.hash);

      if (anchor_comment) {
        // Process at next tick, since we want to re-trigger animations for \
        //   sub-comments if their parent was previously anchored.
        setTimeout(function () {
          anchor_comment.classList.add(anchored_class);

          if (scroll_to === true) {
            anchor_comment.scrollIntoView();
          }
        }, 10);
      }
    }
  };

  var localize_datetimes = function (datetimes) {
    if (datetimes.length > 0) {
      var formatter = new Intl.DateTimeFormat(undefined, {
        dateStyle: "short",
        timeStyle: "short",
      });

      for (var datetime of datetimes) {
        var datetime_utc = datetime.dataset.datetime || "";

        try {
          datetime.innerText = formatter.format(new Date(datetime_utc));
        } catch (error) {
          console.error(
            "[Bandurria] Failed localizing UTC datetime: " + datetime_utc,
            error,
          );
        }
      }
    }
  };

  var mint_challenge_solutions = function (
    problems,
    difficulty_expect,
    solutions_expect,
  ) {
    return new Promise(function (resolve, reject) {
      var worker = new Worker(options.base_url + worker_mint_path);

      worker.addEventListener("message", function (event) {
        worker.terminate();

        resolve(event.data);
      });

      worker.addEventListener("error", function (event) {
        var error = event.message || "Cannot spawn";

        console.error(
          "[Bandurria] Could not start anti-bot: are Web Workers allowed?",
          error,
        );

        worker.terminate();

        reject(error);
      });

      worker.postMessage({
        problems: problems,
        difficulty_expect: difficulty_expect,
        solutions_expect: solutions_expect,
      });
    });
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
      "[Bandurria] Could not initialize: does bandurria-target exist?",
    );
  }
})();
