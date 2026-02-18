use crate::prelude::*;

#[component]
pub fn ThemeModeSwitcher() -> impl IntoView {
  let theme_context = expect_context::<ThemeProviderContext,>();
  let theme_mode = RwSignal::new("auto".to_string(),);

  Effect::new(move || theme_context.set(theme_mode.get().into(),),);

  view! {
    <RadioGroup value=theme_mode>
      <legend>"Theme Mode"</legend>
      <Radio value="auto">"Auto"</Radio>
      <Radio value="dark">"Dark"</Radio>
      <Radio value="light">"Light"</Radio>
    </RadioGroup>
  }
}
