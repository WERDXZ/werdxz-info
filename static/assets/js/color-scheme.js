const colorscheme=[
	"tokyonight-day",
	"tokyonight-night"
]

const preference = {
	light: colorscheme[0],
	dark: colorscheme[1]
}

document.addEventListener("DOMContentLoaded", function() {
	if (
		window.matchMedia &&
		window.matchMedia("(prefers-color-scheme: dark)").matches
	) {
		document.body.classList.add(preference.dark);
	} else {
		document.body.classList.add(preference.light);
	}

	window
		.matchMedia("(prefers-color-scheme: dark)")
		.addEventListener("change", (event) => {
			if (event.matches) {
				document.body.classList.remove(preference.light);
				document.body.classList.add(preference.dark);
			} else {
				document.body.classList.remove(preference.dark);
				document.body.classList.add(preference.light);
			}
		});
});
