use yew::prelude::*;


#[function_component]
fn Header() -> Html {
	html! {
		<header>

		</header>
	}
}

#[function_component]
fn Footer() -> Html {
	html! {
		<footer uk-sticky="position: bottom">
			<span>
				<span uk-icon="icon: rss" uk-tooltip="Current Server"></span>
				<span class="value">{ "irc.libera.chat" }</span>
			</span>
			<span>
				<span uk-icon="icon: check" uk-tooltip="Channel Count"></span>
				<span class="commenting">{ "43" }</span>
			</span>
			<span class="hoz-spacer" />
			<span class="api-status"><span uk-icon="icon: history" uk-tooltip="Backend Connected"></span></span>
		</footer>
	}
}

#[function_component]
pub fn App() -> Html {
	html! {
		<>
			<Header />
			<main>
				<a class="about-btn" href="#about" uk-toggle=""  uk-tooltip="About Annals">
					<span uk-icon="icon: question; ratio: 1.5"></span>
				</a>
				<div id="about" uk-offcanvas="flip: true; esc-close: true; bg-close: true;">
					<div class="uk-offcanvas-bar">
						<button class="uk-offcanvas-close" type="button" uk-close=""></button>
						<h2>{ "meow!" }</h2>
					</div>
				</div>
			</main>
			<Footer />
		</>
	}
}
