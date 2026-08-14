/*
 * Global theme JavaScript.
 *
 * This file runs on every public page. You can add more .js files at the
 * theme root or in assets/. Files inside a feature folder only run there.
 *
 * `spot:page-load` fires after all scripts for the current page have run:
 *
 * window.addEventListener("spot:page-load", (event) => {
 *   console.log("Loaded theme page:", event.detail.feature);
 * });
 */

document.documentElement.dataset.spotTheme = "default";

/* Highlight the navigation link that best matches the page we are on. */
function spotMarkActiveNavigation() {
  const path = window.location.pathname;
  let best = null;

  document.querySelectorAll(".spot-rail-nav a").forEach((link) => {
    link.classList.remove("is-active");
    const href = link.getAttribute("href");
    if (href === "/" ? path === "/" : path.startsWith(href)) {
      if (!best || href.length > best.getAttribute("href").length) {
        best = link;
      }
    }
  });

  if (best) {
    best.classList.add("is-active");
  }
}

window.addEventListener("spot:page-load", spotMarkActiveNavigation);
spotMarkActiveNavigation();
