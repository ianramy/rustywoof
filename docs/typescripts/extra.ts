
document.addEventListener("DOMContentLoaded", () => {
	// 1. Assert that the selected element is an HTMLElement (or null if not found)
	const heroBg = document.querySelector('.tx-hero__bg') as HTMLElement | null;

	if (heroBg) {
		window.addEventListener('scroll', () => {
			const scrollPos = window.scrollY;
			const windowHeight = window.innerHeight;

			let opacity = 1 - (scrollPos / windowHeight) * 1.5;
			const translateY = scrollPos * 0.4;

			if (opacity < 0) opacity = 0;

			// 2. TypeScript now knows .style exists.
			// Note: .opacity expects a string in strict TS, so we convert the number.
			heroBg.style.opacity = opacity.toString();
			heroBg.style.transform = `translateY(${translateY}px)`;
		});
	}
});
