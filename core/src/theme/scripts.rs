//? Resolves system theme before first paint to prevent flash.
//? Runs synchronously in <head> before any rendering.
pub const INIT_SCRIPT : &str = "(function() {
  var theme = document.documentElement.dataset.theme;
  if (!theme || theme === 'system') {
    document.documentElement.dataset.theme =
      window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light';
    }
})();";
