use crate::prelude::*;

#[component]
#[allow(clippy::too_many_lines)]
pub fn Contact() -> impl IntoView {
  let (name, set_name,) = signal(String::new(),);
  let (email, set_email,) = signal(String::new(),);
  let (subject, set_subject,) = signal(String::new(),);
  let (message, set_message,) = signal(String::new(),);
  let (website, set_website,) = signal(String::new(),);
  let (status, set_status,) = signal(Option::<String,>::None,);
  let (submitting, set_submitting,) = signal(false,);

  let submit = move |event : SubmitEvent| {
    event.prevent_default();
    if submitting.get() {
      return;
    }
    set_submitting.set(true,);
    set_status.set(None,);
    let name_value = name.get();
    let email_value = email.get();
    let subject_value = subject.get();
    let message_value = message.get();
    let website_value = website.get();

    spawn_local(async move {
      let result = submit_contact_message(
        name_value,
        email_value,
        subject_value,
        message_value,
        website_value,
      )
      .await;
      set_submitting.set(false,);
      match result {
        Ok((),) => {
          set_name.set(String::new(),);
          set_email.set(String::new(),);
          set_subject.set(String::new(),);
          set_message.set(String::new(),);
          set_status.set(Some("Thanks — your message is on its way. Expect a response from info@craole.cc soon.".into(),),);
        }
        Err(error,) => set_status.set(Some(error.to_string(),),),
      }
    },);
  };

  view! {
    <section id="contact" class="contact-section" aria-labelledby="contact-title">
      <div class="contact-section__inner readable">
        <div class="contact-section__copy">
          <p class="contact-section__label">"Have something to share?"</p>
          <h1 id="contact-title" class="contact-section__title">"Let's connect."</h1>
          <p class="contact-section__sub">
            "Whether you want to talk about a project, a photograph, a visual idea, music, teaching English, or a thought worth developing, I’d love to hear from you."
          </p>
          <ul class="contact-section__topics">
            <li>"Software, data, and technical projects"</li>
            <li>"Photography, graphics, and visual work"</li>
            <li>"Music, collaboration, and creative practice"</li>
            <li>"Teaching English, writing, and ideas"</li>
          </ul>
        </div>
        <form class="contact-section__form" on:submit=submit>
          <div class="contact-section__fields">
            <label>
              "Name"
              <input
                required
                maxlength="120"
                autocomplete="name"
                prop:value=move || name.get()
                on:input=move |event| set_name.set(event_target_value(&event))
              />
            </label>
            <label>
              "Email"
              <input
                required
                type="email"
                maxlength="254"
                autocomplete="email"
                prop:value=move || email.get()
                on:input=move |event| set_email.set(event_target_value(&event))
              />
            </label>
          </div>
          <label>
            "Subject"
            <input
              required
              maxlength="180"
              prop:value=move || subject.get()
              on:input=move |event| set_subject.set(event_target_value(&event))
            />
          </label>
          <label>
            "Message"
            <textarea
              required
              maxlength="8000"
              rows="6"
              prop:value=move || message.get()
              on:input=move |event| set_message.set(event_target_value(&event))
            ></textarea>
          </label>
          // Honeypot field: invisible to normal visitors, useful against simple bots.
          <label class="contact-section__honeypot" aria-hidden="true">
            "Website"
            <input
              tabindex="-1"
              autocomplete="off"
              prop:value=move || website.get()
              on:input=move |event| set_website.set(event_target_value(&event))
            />
          </label>
          <button class="contact-section__button" type="submit" disabled=submitting>
            {move || if submitting.get() { "Sending…" } else { "Send message →" }}
          </button>
          <p class="contact-section__note" aria-live="polite">
            {move || status.get().unwrap_or_else(|| "Expect a response from info@craole.cc soon.".into())}
          </p>
        </form>
      </div>
    </section>
  }
}
