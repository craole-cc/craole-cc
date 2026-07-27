//? Resolves system theme before first paint to prevent flash.
//? Runs synchronously in <head> before any rendering.
pub const INIT_SCRIPT : &str = "(function() {
  var saved = null;
  try { saved = localStorage.getItem('craole-theme'); } catch (_) {}
  var theme = saved || document.documentElement.dataset.theme;
  if (theme !== 'light' && theme !== 'dark') {
    theme = window.matchMedia('(prefers-color-scheme: dark)').matches
      ? 'dark'
      : 'light';
  }
  document.documentElement.dataset.theme = theme;
})();";
