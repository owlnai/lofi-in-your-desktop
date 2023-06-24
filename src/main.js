const { appWindow } = window.__TAURI__.window;
const { event } = window.__TAURI__.event;
const { open } = window.__TAURI__.shell;
  function navigateTo(link) {
    open(link);
}
  
window.addEventListener("DOMContentLoaded", () => {
  document.getElementById("goToGitHub").addEventListener("click", () => {
    open("https://github.com/owlnai/lofi-in-your-desktop")
  });
  document
    .getElementById("titlebar-minimize")
    .addEventListener("click", () => appWindow.minimize());
  document
    .getElementById("titlebar-maximize")
    .addEventListener("click", () => appWindow.toggleMaximize());
  document
    .getElementById("titlebar-close")
    .addEventListener("click", () => appWindow.close());
});
